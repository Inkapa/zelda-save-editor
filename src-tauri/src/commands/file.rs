use std::path::{Path, PathBuf};

use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::dto::{OpenResult, TotkCaptionDto};
use crate::error::ShellError;
use crate::state::AppState;
use save_engine::Save;

fn backup_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(".bak");
    PathBuf::from(name)
}

fn tmp_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(".tmp");
    PathBuf::from(name)
}

/// Writes `bytes` to `path` via write-to-temp-then-rename, so a crash or disk-full failure
/// mid-write can never leave `path` itself truncated: it either keeps its old content or gets
/// the new content in full, never a partial mix of both.
fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = tmp_path(path);
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)
}

/// Detects the save format from raw bytes and builds the DTO to return to the frontend,
/// without touching `AppState`. The caller is responsible for storing the resulting
/// `Save` and file path. Pure and Tauri-free: directly unit-testable against real fixtures.
///
/// `caption.sav` isn't a `Save`: it has no editable state and nothing to write back
/// (the source only ever reads it, to show a thumbnail next to a save slot), so a
/// successful caption parse returns `None` in place of a `Save` for the caller to store.
pub fn detect_and_build(bytes: Vec<u8>) -> Result<(Option<Save>, OpenResult), ShellError> {
    let detect_err = match Save::detect(bytes.clone()) {
        Ok(save) => {
            let result = match &save {
                Save::Botw(botw_save) => {
                    OpenResult::Botw { state: crate::commands::botw::read_state(botw_save)? }
                }
                Save::Totk(totk_save) => {
                    OpenResult::Totk { state: crate::commands::totk::read_state(totk_save)? }
                }
            };
            return Ok((Some(save), result));
        }
        Err(err) => err,
    };

    match save_engine::totk::caption::parse(bytes) {
        Ok(info) => Ok((None, OpenResult::Caption { info: TotkCaptionDto::from(info) })),
        Err(_) => Err(detect_err.into()),
    }
}

/// Reads a save from `path`, detects its format, and (for an editable BOTW/TOTK save) stores it
/// plus its path in `AppState`. Shared by the file-picker `open_save` and the path-based
/// `open_save_at` used by the recent-files list.
fn load_from_path(state: &AppState, path: PathBuf) -> Result<OpenResult, ShellError> {
    let bytes = std::fs::read(&path).map_err(ShellError::io)?;
    let (save, result) = detect_and_build(bytes)?;
    if let Some(save) = save {
        *crate::state::lock(&state.save) = Some(save);
        *crate::state::lock(&state.path) = Some(path);
    }
    Ok(result)
}

#[tauri::command]
pub fn open_save(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<OpenResult, ShellError> {
    let picked = app
        .dialog()
        .file()
        .add_filter("Save files", &["sav"])
        .blocking_pick_file()
        .ok_or_else(ShellError::dialog_cancelled)?;
    let path: PathBuf = picked
        .into_path()
        .map_err(|e| ShellError::io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string())))?;
    load_from_path(state.inner(), path)
}

/// Opens a save at a known path (a recent-files entry), skipping the file dialog.
#[tauri::command]
pub fn open_save_at(path: String, state: State<'_, AppState>) -> Result<OpenResult, ShellError> {
    load_from_path(state.inner(), PathBuf::from(path))
}

/// The path of the currently-loaded save, if any, so the frontend can record it in its
/// recent-files list after a successful open.
#[tauri::command]
pub fn current_path(state: State<'_, AppState>) -> Option<String> {
    crate::state::lock(&state.path)
        .as_ref()
        .map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn save(state: State<'_, AppState>) -> Result<(), ShellError> {
    let path = crate::state::lock(&state.path).clone().ok_or_else(ShellError::no_save_loaded)?;
    write_current_save(state.inner(), &path)
}

#[tauri::command]
pub fn save_as(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), ShellError> {
    let picked = app
        .dialog()
        .file()
        .add_filter("Save files", &["sav"])
        .blocking_save_file()
        .ok_or_else(ShellError::dialog_cancelled)?;
    let path: PathBuf = picked
        .into_path()
        .map_err(|e| ShellError::io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string())))?;
    write_current_save(state.inner(), &path)?;
    *crate::state::lock(&state.path) = Some(path);
    Ok(())
}

/// Serializes the currently-loaded save, writes it to `path`, then re-detects the freshly
/// written bytes to put a newly-constructed `Save` back into `AppState`. `Save::to_bytes`
/// consumes `self`, so this is how editing continues to work after a save without changing
/// that signature.
///
/// Before touching `path`, copies whatever's already there to a `.bak` sibling (overwriting
/// any previous backup), so a bad edit is always one file away from recoverable. Done before
/// `AppState.save` is taken, so a failed backup leaves the in-memory edits untouched instead
/// of silently discarding them.
///
/// The write itself goes through `write_atomic` (write-to-temp-then-rename) so a partial write
/// never corrupts `path` in place; on any write failure the pre-existing `.bak` is still exactly
/// what was there before this call.
fn write_current_save(app_state: &AppState, path: &PathBuf) -> Result<(), ShellError> {
    if path.exists() {
        std::fs::copy(path, backup_path(path)).map_err(ShellError::io)?;
    }
    let save = crate::state::lock(&app_state.save).take().ok_or_else(ShellError::no_save_loaded)?;
    let bytes = save.to_bytes();
    let write_result = write_atomic(path, &bytes);
    // Re-detect and restore state before propagating any write error, otherwise a disk-write
    // failure would permanently empty AppState.save, discarding in-memory edits until the user
    // manually reopens the file. If re-detection itself fails, the freshly written bytes don't
    // round-trip back into a Save (an engine invariant this crate's own tests otherwise cover),
    // and there's no prior Save left to restore either since `to_bytes` above consumed it: the
    // error propagates and AppState.save is left empty, same as a save that was never loaded.
    let (reloaded, _) = detect_and_build(bytes)?;
    *crate::state::lock(&app_state.save) = reloaded;
    write_result.map_err(ShellError::io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_and_build_recognizes_botw_fixture() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present");
        let (_, result) = detect_and_build(bytes).unwrap();
        assert!(matches!(result, OpenResult::Botw { .. }));
    }

    #[test]
    fn detect_and_build_recognizes_totk_fixture() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present");
        let (_, result) = detect_and_build(bytes).unwrap();
        assert!(matches!(result, OpenResult::Totk { .. }));
    }

    #[test]
    fn detect_and_build_rejects_garbage_bytes() {
        let result = detect_and_build(vec![0u8; 16]);
        assert!(result.is_err());
    }

    #[test]
    fn detect_and_build_falls_back_to_caption_when_not_a_botw_or_totk_save() {
        const HASH_TABLE_END: usize = 0x1c0;
        let mut bytes = vec![0u8; 0x28];
        let rows: [(u32, u32); 7] = [
            (0x9811a3f7, 2026),
            (0xdfd840d3, 7),
            (0xbd46f485, 30),
            (0x23f3d75e, 14),
            (0x27853bf7, 45),
            (0x25f03caa, 0),
            (0x63696a32, HASH_TABLE_END as u32),
        ];
        for (hash, value) in rows {
            bytes.extend_from_slice(&hash.to_le_bytes());
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.resize(HASH_TABLE_END, 0);
        let jpeg_bytes = [0xffu8, 0xd8, 0xff, 0xd9];
        bytes.extend_from_slice(&(jpeg_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&jpeg_bytes);

        let (save, result) = detect_and_build(bytes).unwrap();
        assert!(save.is_none(), "caption.sav has no editable Save to store");
        match result {
            OpenResult::Caption { info } => {
                assert_eq!(info.year, 2026);
                assert_eq!(info.thumbnail_jpeg, vec![0xff, 0xd8, 0xff, 0xd9]);
            }
            _ => panic!("expected OpenResult::Caption"),
        }
    }

    #[test]
    fn write_current_save_round_trips_and_leaves_state_reloaded() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present");
        let original_len = bytes.len();
        let (save, _) = detect_and_build(bytes).unwrap();
        let save = save.expect("botw fixture should produce a Save");
        let app_state = AppState {
            save: std::sync::Mutex::new(Some(save)),
            path: std::sync::Mutex::new(None),
        };
        let tmp_path = std::env::temp_dir().join("zelda_shell_write_current_save_test.sav");
        write_current_save(&app_state, &tmp_path).unwrap();

        let written = std::fs::read(&tmp_path).unwrap();
        assert_eq!(written.len(), original_len);
        assert!(app_state.save.lock().unwrap().is_some(), "state should be reloaded, not left empty");

        std::fs::remove_file(&tmp_path).ok();
    }

    #[test]
    fn write_current_save_backs_up_existing_file_before_overwriting() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present");
        let tmp_path = std::env::temp_dir().join("zelda_shell_backup_test.sav");
        let original_bytes = b"stand-in for a previous save on disk".to_vec();
        std::fs::write(&tmp_path, &original_bytes).unwrap();

        let (save, _) = detect_and_build(bytes).unwrap();
        let save = save.expect("botw fixture should produce a Save");
        let app_state = AppState {
            save: std::sync::Mutex::new(Some(save)),
            path: std::sync::Mutex::new(None),
        };
        write_current_save(&app_state, &tmp_path).unwrap();

        let backup = std::fs::read(backup_path(&tmp_path)).expect("backup file should exist");
        assert_eq!(backup, original_bytes, "backup should hold the pre-overwrite content");
        let overwritten = std::fs::read(&tmp_path).unwrap();
        assert_ne!(overwritten, original_bytes, "the actual file should now hold the new save");

        std::fs::remove_file(&tmp_path).ok();
        std::fs::remove_file(backup_path(&tmp_path)).ok();
    }

    #[test]
    fn write_current_save_restores_state_even_when_write_fails() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present");
        let (save, _) = detect_and_build(bytes).unwrap();
        let save = save.expect("botw fixture should produce a Save");
        let app_state = AppState {
            save: std::sync::Mutex::new(Some(save)),
            path: std::sync::Mutex::new(None),
        };
        // A path whose parent directory doesn't exist. std::fs::write fails reliably here.
        let bad_path = std::env::temp_dir()
            .join("zelda_shell_nonexistent_dir_xyz")
            .join("test.sav");

        let result = write_current_save(&app_state, &bad_path);

        assert!(result.is_err(), "write to a nonexistent directory should fail");
        assert!(
            app_state.save.lock().unwrap().is_some(),
            "state should still be populated after a failed write, not discarded"
        );
    }
}
