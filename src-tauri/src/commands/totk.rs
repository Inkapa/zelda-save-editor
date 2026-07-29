use tauri::State;

use crate::dto::{
    TotkArmorDto, TotkArrowDto, TotkBowDto, TotkDeviceDto, TotkFoodDto, TotkHorseDto,
    TotkKeyItemDto, TotkMaterialDto, TotkShieldDto, TotkState, TotkWeaponDto,
};
use crate::error::ShellError;
use crate::state::AppState;
use save_engine::totk::TotkSave;
use save_engine::{Save, SaveError};

pub fn read_state(save: &TotkSave) -> Result<TotkState, ShellError> {
    Ok(TotkState {
        max_life: save.max_life()?,
        current_rupees: save.current_rupees()?,
        max_stamina: save.max_stamina()?,
        max_energy: save.max_energy()?,
        playtime: save.playtime()?,
        horse_inn_member_point: save.horse_inn_member_point()?,
        save_pos: save.save_pos()?,
        sequence_current_banc: save.sequence_current_banc()?,
        pouch_weapon_valid_num: save.pouch_weapon_valid_num()?,
        pouch_bow_valid_num: save.pouch_bow_valid_num()?,
        pouch_shield_valid_num: save.pouch_shield_valid_num()?,
        pouch_weapons: save.pouch_weapons()?.into_iter().map(TotkWeaponDto::from).collect(),
        pouch_bows: save.pouch_bows()?.into_iter().map(TotkBowDto::from).collect(),
        pouch_shields: save.pouch_shields()?.into_iter().map(TotkShieldDto::from).collect(),
        armor: save.armor()?.into_iter().map(TotkArmorDto::from).collect(),
        arrows: save.arrows()?.into_iter().map(TotkArrowDto::from).collect(),
        materials: save.materials()?.into_iter().map(TotkMaterialDto::from).collect(),
        key_items: save.key_items()?.into_iter().map(TotkKeyItemDto::from).collect(),
        devices: save.devices()?.into_iter().map(TotkDeviceDto::from).collect(),
        food: save.food()?.into_iter().map(TotkFoodDto::from).collect(),
        horses: save.horses()?.into_iter().map(TotkHorseDto::from).collect(),
    })
}

pub fn get_totk_state_impl(app_state: &AppState) -> Result<TotkState, ShellError> {
    let guard = app_state.save.lock().unwrap();
    match guard.as_ref() {
        Some(Save::Totk(save)) => read_state(save),
        Some(Save::Botw(_)) => Err(ShellError::wrong_game("TOTK")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn get_totk_state(state: State<'_, AppState>) -> Result<TotkState, ShellError> {
    get_totk_state_impl(state.inner())
}

pub fn with_totk<T>(
    app_state: &AppState,
    f: impl FnOnce(&mut TotkSave) -> Result<T, SaveError>,
) -> Result<T, ShellError> {
    let mut guard = app_state.save.lock().unwrap();
    match guard.as_mut() {
        Some(Save::Totk(save)) => f(save).map_err(ShellError::from),
        Some(Save::Botw(_)) => Err(ShellError::wrong_game("TOTK")),
        None => Err(ShellError::no_save_loaded()),
    }
}

#[tauri::command]
pub fn set_max_life(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_max_life(val))
}

#[tauri::command]
pub fn set_current_rupees(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_current_rupees(val))
}

// Tauri v2's #[tauri::command] macro generates crate-level symbols per literal function
// name, so a TOTK command sharing a name with a BOTW command collides at compile time
// (E0428). Only this one name (`set_max_stamina`) actually collides with anything in
// commands::botw. Every other TOTK setter keeps its plain name matching TotkSave's own
// method name; renamed only this one instead of prefixing the whole file.
#[tauri::command]
pub fn totk_set_max_stamina(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_max_stamina(val))
}

#[tauri::command]
pub fn set_max_energy(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_max_energy(val))
}

#[tauri::command]
pub fn set_playtime(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_playtime(val))
}

#[tauri::command]
pub fn set_horse_inn_member_point(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_horse_inn_member_point(val))
}

#[tauri::command]
pub fn set_save_pos(state: State<'_, AppState>, x: f32, y: f32, z: f32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_save_pos(x, y, z))
}

#[tauri::command]
pub fn set_sequence_current_banc(state: State<'_, AppState>, value: String) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_sequence_current_banc(&value))
}

#[tauri::command]
pub fn set_pouch_weapon_valid_num(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_pouch_weapon_valid_num(val))
}

#[tauri::command]
pub fn set_pouch_bow_valid_num(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_pouch_bow_valid_num(val))
}

#[tauri::command]
pub fn set_pouch_shield_valid_num(state: State<'_, AppState>, val: u32) -> Result<(), ShellError> {
    with_totk(state.inner(), |save| save.set_pouch_shield_valid_num(val))
}

#[tauri::command]
pub fn set_pouch_weapons(state: State<'_, AppState>, entries: Vec<TotkWeaponDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_pouch_weapons(&entries))
}

#[tauri::command]
pub fn set_pouch_bows(state: State<'_, AppState>, entries: Vec<TotkBowDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_pouch_bows(&entries))
}

#[tauri::command]
pub fn set_pouch_shields(state: State<'_, AppState>, entries: Vec<TotkShieldDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_pouch_shields(&entries))
}

#[tauri::command]
pub fn set_armor(state: State<'_, AppState>, entries: Vec<TotkArmorDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_armor(&entries))
}

#[tauri::command]
pub fn set_arrows(state: State<'_, AppState>, entries: Vec<TotkArrowDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_arrows(&entries))
}

#[tauri::command]
pub fn set_materials(state: State<'_, AppState>, entries: Vec<TotkMaterialDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_materials(&entries))
}

#[tauri::command]
pub fn set_key_items(state: State<'_, AppState>, entries: Vec<TotkKeyItemDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_key_items(&entries))
}

#[tauri::command]
pub fn set_devices(state: State<'_, AppState>, entries: Vec<TotkDeviceDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_devices(&entries))
}

#[tauri::command]
pub fn set_food(state: State<'_, AppState>, entries: Vec<TotkFoodDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_food(&entries))
}

#[tauri::command]
pub fn set_horses(state: State<'_, AppState>, entries: Vec<TotkHorseDto>) -> Result<(), ShellError> {
    let entries: Vec<_> = entries.into_iter().map(Into::into).collect();
    with_totk(state.inner(), |save| save.set_horses(&entries))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn totk_fixture_bytes() -> Vec<u8> {
        std::fs::read("../crates/save-engine/tests/fixtures/totk/progress.sav")
            .expect("fixture present")
    }

    fn botw_fixture_bytes() -> Vec<u8> {
        std::fs::read("../crates/save-engine/tests/fixtures/botw/game_data.sav")
            .expect("fixture present")
    }

    fn loaded_app_state() -> AppState {
        let save = Save::detect(totk_fixture_bytes()).expect("should detect a known TOTK save");
        AppState { save: std::sync::Mutex::new(Some(save)), path: std::sync::Mutex::new(None) }
    }

    #[test]
    fn get_totk_state_impl_reads_real_fixture_values() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert!(dto.max_life > 0);
    }

    #[test]
    fn get_totk_state_impl_on_botw_save_returns_wrong_game() {
        let save = Save::detect(botw_fixture_bytes()).expect("should detect a known BOTW save");
        let app_state = AppState { save: std::sync::Mutex::new(Some(save)), path: std::sync::Mutex::new(None) };
        let result = get_totk_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "wrong_game");
    }

    #[test]
    fn get_totk_state_impl_with_no_save_returns_no_save_loaded() {
        let app_state = AppState::default();
        let result = get_totk_state_impl(&app_state);
        assert_eq!(result.unwrap_err().kind, "no_save_loaded");
    }

    #[test]
    fn with_totk_edits_current_rupees_and_persists_in_state() {
        let app_state = loaded_app_state();
        let before = get_totk_state_impl(&app_state).unwrap().current_rupees;
        with_totk(&app_state, |save| save.set_current_rupees(before.wrapping_add(500))).unwrap();
        let after = get_totk_state_impl(&app_state).unwrap();
        assert_eq!(after.current_rupees, before.wrapping_add(500));
    }

    #[test]
    fn with_totk_edits_save_pos_and_persists_in_state() {
        let app_state = loaded_app_state();
        with_totk(&app_state, |save| save.set_save_pos(1.0, 2.0, 3.0)).unwrap();
        let after = get_totk_state_impl(&app_state).unwrap();
        assert_eq!(after.save_pos, (1.0, 2.0, 3.0));
    }

    #[test]
    fn get_totk_state_impl_includes_pouch_and_horse_entries() {
        let app_state = loaded_app_state();
        let dto = get_totk_state_impl(&app_state).unwrap();
        assert!(!dto.pouch_weapons.is_empty());
        assert!(!dto.armor.is_empty());
        assert!(!dto.horses.is_empty());
        assert_eq!(dto.horses[0].name, "Max");
    }

    #[test]
    fn set_pouch_weapons_round_trips_through_with_totk() {
        let app_state = loaded_app_state();
        let mut weapons = get_totk_state_impl(&app_state).unwrap().pouch_weapons;
        weapons.push(crate::dto::TotkWeaponDto {
            id: "Weapon_Sword_001".into(),
            durability: 20,
            modifier: 0,
            modifier_value: 0,
            fuse_id: String::new(),
            fuse_durability: 0,
            extra_durability: 0,
            record_extra_durability: -1,
        });
        with_totk(&app_state, |save| {
            save.set_pouch_weapons(&weapons.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().pouch_weapons;
        assert_eq!(after.last().unwrap().id, "Weapon_Sword_001");
    }

    #[test]
    fn set_horses_round_trips_through_with_totk() {
        let app_state = loaded_app_state();
        let mut horses = get_totk_state_impl(&app_state).unwrap().horses;
        horses[0].name = "Renamed".to_string();
        with_totk(&app_state, |save| {
            save.set_horses(&horses.iter().cloned().map(Into::into).collect::<Vec<_>>())
        })
        .unwrap();
        let after = get_totk_state_impl(&app_state).unwrap().horses;
        assert_eq!(after[0].name, "Renamed");
    }
}
