pub mod commands;
pub mod dto;
pub mod error;
pub mod state;

use state::AppState;

#[tauri::command]
fn ping() -> String {
    "pong".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            ping,
            commands::botw::get_botw_state,
            commands::botw::set_rupees,
            commands::botw::set_mons,
            commands::botw::set_max_hearts,
            commands::botw::set_max_stamina,
            commands::botw::set_relic_gerudo,
            commands::botw::set_relic_goron,
            commands::botw::set_relic_rito,
            commands::botw::set_korok_seed_counter,
            commands::botw::set_defeated_hinox_counter,
            commands::botw::set_defeated_talus_counter,
            commands::botw::set_defeated_molduga_counter,
            commands::botw::set_playtime_seconds,
            commands::botw::set_motorcycle,
            commands::botw::set_player_position,
            commands::botw::set_horse_position,
            commands::botw::set_map,
            commands::botw::set_map_type,
            commands::botw::set_item,
            commands::botw::set_modifier,
            commands::botw::set_horse_name,
            commands::botw::set_horse_saddle,
            commands::botw::set_horse_reins,
            commands::botw::set_horse_type,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
