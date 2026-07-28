use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotwItemDto {
    pub name: String,
    pub quantity: u32,
}

impl From<save_engine::botw::BotwItem> for BotwItemDto {
    fn from(item: save_engine::botw::BotwItem) -> Self {
        BotwItemDto { name: item.name, quantity: item.quantity }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ItemModifierDto {
    pub modifier: u32,
    pub value: u32,
}

impl From<save_engine::botw::ItemModifier> for ItemModifierDto {
    fn from(m: save_engine::botw::ItemModifier) -> Self {
        ItemModifierDto { modifier: m.modifier, value: m.value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotwHorseDto {
    pub name: Option<String>,
    pub saddle: Option<String>,
    pub reins: Option<String>,
    pub horse_type: String,
}

impl From<save_engine::botw::BotwHorse> for BotwHorseDto {
    fn from(h: save_engine::botw::BotwHorse) -> Self {
        BotwHorseDto { name: h.name, saddle: h.saddle, reins: h.reins, horse_type: h.horse_type }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModifierCategoryDto {
    Weapon,
    Bow,
    Shield,
}

impl From<ModifierCategoryDto> for save_engine::botw::ModifierCategory {
    fn from(c: ModifierCategoryDto) -> Self {
        match c {
            ModifierCategoryDto::Weapon => save_engine::botw::ModifierCategory::Weapon,
            ModifierCategoryDto::Bow => save_engine::botw::ModifierCategory::Bow,
            ModifierCategoryDto::Shield => save_engine::botw::ModifierCategory::Shield,
        }
    }
}

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
    pub items: Vec<BotwItemDto>,
    pub weapon_modifiers: Vec<ItemModifierDto>,
    pub bow_modifiers: Vec<ItemModifierDto>,
    pub shield_modifiers: Vec<ItemModifierDto>,
    pub horses: Vec<BotwHorseDto>,
}
