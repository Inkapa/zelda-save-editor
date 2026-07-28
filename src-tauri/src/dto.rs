use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotwState {
    pub rupees: u32,
    pub mons: u32,
    pub max_hearts: u32,
    pub max_stamina: u32,
    pub relic_gerudo: u32,
    pub relic_goron: u32,
    pub relic_rito: u32,
    pub korok_seed_counter: u32,
    pub defeated_hinox_counter: u32,
    pub defeated_talus_counter: u32,
    pub defeated_molduga_counter: u32,
    pub playtime_seconds: u32,
    pub motorcycle: Option<bool>,
    pub player_position: (f32, f32, f32),
    pub horse_position: (f32, f32, f32),
    pub map: String,
    pub map_type: String,
}
