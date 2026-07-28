use tauri::State;

use crate::dto::BotwState;
use crate::error::ShellError;
use crate::state::AppState;
use save_engine::botw::BotwSave;
use save_engine::{Save, SaveError};

/// Builds the DTO from a loaded `BotwSave`. Pure and Tauri-free — directly unit-testable
/// against a real fixture, and reused (extended) by Task 4 once the list-shaped fields exist.
pub fn read_state(save: &BotwSave) -> Result<BotwState, ShellError> {
    Ok(BotwState {
        rupees: save.rupees()?,
        mons: save.mons()?,
        max_hearts: save.max_hearts()?,
        max_stamina: save.max_stamina()?,
        relic_gerudo: save.relic_gerudo()?,
        relic_goron: save.relic_goron()?,
        relic_rito: save.relic_rito()?,
        korok_seed_counter: save.korok_seed_counter()?,
        defeated_hinox_counter: save.defeated_hinox_counter()?,
        defeated_talus_counter: save.defeated_talus_counter()?,
        defeated_molduga_counter: save.defeated_molduga_counter()?,
        playtime_seconds: save.playtime_seconds()?,
        motorcycle: save.motorcycle(),
        player_position: save.player_position()?,
        horse_position: save.horse_position()?,
        map: save.map()?,
        map_type: save.map_type()?,
    })
}

pub fn get_botw_state_impl(app_state: &AppState) -> Result<BotwState, ShellError> {
    let guard = app_state.save.lock().unwrap();
    match guard.as_ref() {
        Some(Save::Botw(save)) => read_state(save),
        Some(Save::Totk(_)) => Err(ShellError::wrong_game("BOTW")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn get_botw_state(state: State<'_, AppState>) -> Result<BotwState, ShellError> {
    get_botw_state_impl(state.inner())
}

/// Shared plumbing for every BOTW setter: lock the state, confirm a BOTW save is loaded,
/// run `f` against it, map any engine error through `ShellError`. Pure and Tauri-free.
pub fn with_botw<T>(
    app_state: &AppState,
    f: impl FnOnce(&mut BotwSave) -> Result<T, SaveError>,
) -> Result<T, ShellError> {
    let mut guard = app_state.save.lock().unwrap();
    match guard.as_mut() {
        Some(Save::Botw(save)) => f(save).map_err(ShellError::from),
        Some(Save::Totk(_)) => Err(ShellError::wrong_game("BOTW")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn set_rupees(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_rupees(val))
}

#[tauri::command]
pub fn set_mons(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_mons(val))
}

#[tauri::command]
pub fn set_max_hearts(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_max_hearts(val))
}

#[tauri::command]
pub fn set_max_stamina(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_max_stamina(val))
}

#[tauri::command]
pub fn set_relic_gerudo(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_relic_gerudo(val))
}

#[tauri::command]
pub fn set_relic_goron(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_relic_goron(val))
}

#[tauri::command]
pub fn set_relic_rito(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_relic_rito(val))
}

#[tauri::command]
pub fn set_korok_seed_counter(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_korok_seed_counter(val))
}

#[tauri::command]
pub fn set_defeated_hinox_counter(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_defeated_hinox_counter(val))
}

#[tauri::command]
pub fn set_defeated_talus_counter(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_defeated_talus_counter(val))
}

#[tauri::command]
pub fn set_defeated_molduga_counter(
    state: State<'_, AppState>,
    val: u32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_defeated_molduga_counter(val))
}

#[tauri::command]
pub fn set_playtime_seconds(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_playtime_seconds(val))
}

#[tauri::command]
pub fn set_motorcycle(state: State<'_, AppState>, val: bool) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| {
        save.set_motorcycle(val);
        Ok(())
    })
}

#[tauri::command]
pub fn set_player_position(
    state: State<'_, AppState>,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_player_position(x, y, z))
}

#[tauri::command]
pub fn set_horse_position(
    state: State<'_, AppState>,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_horse_position(x, y, z))
}

#[tauri::command]
pub fn set_map(state: State<'_, AppState>, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_map(&value))
}

#[tauri::command]
pub fn set_map_type(state: State<'_, AppState>, value: String) -> Result<(), ShellError> {
    with_botw(state.inner(), |save| save.set_map_type(&value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_bytes() -> Vec<u8> {
        std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present")
    }

    fn loaded_app_state() -> AppState {
        let save = Save::detect(fixture_bytes()).expect("should detect a known BOTW save");
        AppState { save: std::sync::Mutex::new(Some(save)), path: std::sync::Mutex::new(None) }
    }

    #[test]
    fn get_botw_state_impl_reads_real_fixture_values() {
        let app_state = loaded_app_state();
        let dto = get_botw_state_impl(&app_state).unwrap();

        // Cross-check against a second, independently loaded instance of the same
        // fixture calling the engine's own accessor directly — a real assertion that
        // the DTO faithfully mirrors `save-engine`, not just that the call didn't panic.
        let expected_rupees = match Save::detect(fixture_bytes()).unwrap() {
            Save::Botw(save) => save.rupees().unwrap(),
            Save::Totk(_) => panic!("fixture should be BOTW"),
        };
        assert_eq!(dto.rupees, expected_rupees);
        assert!(!dto.map.is_empty());
    }

    #[test]
    fn get_botw_state_impl_on_totk_save_returns_wrong_game() {
        let totk_bytes = std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present");
        let save = Save::detect(totk_bytes).expect("should detect a known TOTK save");
        let app_state = AppState { save: std::sync::Mutex::new(Some(save)), path: std::sync::Mutex::new(None) };
        let result = get_botw_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "wrong_game");
    }

    #[test]
    fn get_botw_state_impl_with_no_save_returns_no_save_loaded() {
        let app_state = AppState::default();
        let result = get_botw_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "no_save_loaded");
    }

    #[test]
    fn with_botw_edits_rupees_and_persists_in_state() {
        let app_state = loaded_app_state();
        let before = get_botw_state_impl(&app_state).unwrap().rupees;
        with_botw(&app_state, |save| save.set_rupees(before.wrapping_add(500))).unwrap();
        let after = get_botw_state_impl(&app_state).unwrap();
        assert_eq!(after.rupees, before.wrapping_add(500));
    }

    #[test]
    fn with_botw_on_totk_save_returns_wrong_game() {
        let totk_bytes = std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present");
        let save = Save::detect(totk_bytes).expect("should detect a known TOTK save");
        let app_state = AppState { save: std::sync::Mutex::new(Some(save)), path: std::sync::Mutex::new(None) };
        let result = with_botw(&app_state, |save| save.set_rupees(1));
        assert_eq!(result.unwrap_err().kind, "wrong_game");
    }

    #[test]
    fn with_botw_edits_motorcycle_and_persists_in_state() {
        let app_state = loaded_app_state();
        let initial_motorcycle = get_botw_state_impl(&app_state).unwrap().motorcycle;

        // If the fixture has the motorcycle field, setting it should persist; if not, it stays None.
        with_botw(&app_state, |save| {
            save.set_motorcycle(true);
            Ok(())
        })
        .unwrap();

        let after_motorcycle = get_botw_state_impl(&app_state).unwrap().motorcycle;
        // If the field exists in the save version, it should be updated to true.
        // If not (None initially), it should remain None.
        if initial_motorcycle.is_some() {
            assert_eq!(after_motorcycle, Some(true));
        } else {
            assert_eq!(after_motorcycle, None);
        }
    }
}
