use std::path::PathBuf;

use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::dto::OpenResult;
use crate::error::ShellError;
use crate::state::AppState;
use save_engine::Save;

/// Detects the save format from raw bytes and builds the DTO to return to the frontend,
/// without touching `AppState` — the caller is responsible for storing the resulting
/// `Save` and file path. Pure and Tauri-free: directly unit-testable against real fixtures.
pub fn detect_and_build(bytes: Vec<u8>) -> Result<(Save, OpenResult), ShellError> {
    let save = Save::detect(bytes)?;
    let result = match &save {
        Save::Botw(botw_save) => {
            OpenResult::Botw { state: crate::commands::botw::read_state(botw_save)? }
        }
        Save::Totk(totk_save) => {
            OpenResult::Totk { state: crate::commands::totk::read_state(totk_save)? }
        }
    };
    Ok((save, result))
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
    let bytes = std::fs::read(&path).map_err(ShellError::io)?;
    let (save, result) = detect_and_build(bytes)?;
    *state.save.lock().unwrap() = Some(save);
    *state.path.lock().unwrap() = Some(path);
    Ok(result)
}

#[tauri::command]
pub fn save(state: State<'_, AppState>) -> Result<(), ShellError> {
    let path = state.path.lock().unwrap().clone().ok_or_else(ShellError::no_save_loaded)?;
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
    *state.path.lock().unwrap() = Some(path);
    Ok(())
}

/// Serializes the currently-loaded save, writes it to `path`, then re-detects the freshly
/// written bytes to put a newly-constructed `Save` back into `AppState` — `Save::to_bytes`
/// consumes `self`, so this is how editing continues to work after a save without changing
/// the engine's already-shipped signature (see Global Constraints).
fn write_current_save(app_state: &AppState, path: &PathBuf) -> Result<(), ShellError> {
    let save = app_state.save.lock().unwrap().take().ok_or_else(ShellError::no_save_loaded)?;
    let bytes = save.to_bytes();
    let write_result = std::fs::write(path, &bytes);
    // Re-detect and restore state before propagating any write error — otherwise a disk-write
    // failure would permanently empty AppState.save, discarding in-memory edits until the user
    // manually reopens the file.
    let (reloaded, _) = detect_and_build(bytes)?;
    *app_state.save.lock().unwrap() = Some(reloaded);
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
    fn write_current_save_round_trips_and_leaves_state_reloaded() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present");
        let original_len = bytes.len();
        let (save, _) = detect_and_build(bytes).unwrap();
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
    fn write_current_save_restores_state_even_when_write_fails() {
        let bytes = std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present");
        let (save, _) = detect_and_build(bytes).unwrap();
        let app_state = AppState {
            save: std::sync::Mutex::new(Some(save)),
            path: std::sync::Mutex::new(None),
        };
        // A path whose parent directory doesn't exist — std::fs::write fails reliably here.
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
