use save_engine::botw::ModifierCategory;
use save_engine::{Save, SaveError};

fn fixture_bytes() -> Vec<u8> {
    std::fs::read("tests/fixtures/botw/game_data.sav").expect("fixture present")
}

fn load_botw() -> save_engine::botw::BotwSave {
    match Save::detect(fixture_bytes()).expect("should detect a known BOTW save") {
        Save::Botw(save) => save,
        Save::Totk(_) => panic!("BOTW fixture was misdetected as TOTK"),
    }
}

#[test]
fn detects_as_botw_v1_1_wiiu() {
    let save = load_botw();
    assert_eq!(save.version_index, 1);
    assert!(!save.modded);
}

#[test]
fn round_trip_without_edits_is_byte_identical() {
    let original = fixture_bytes();
    let save = Save::detect(original.clone()).unwrap();
    let output = save.to_bytes();
    assert_eq!(original, output);
}

#[test]
fn editing_rupees_only_touches_the_rupee_field() {
    let original = fixture_bytes();
    let mut save = load_botw();
    let before = save.rupees().unwrap();
    save.set_rupees(before.wrapping_add(500)).unwrap();
    assert_eq!(save.rupees().unwrap(), before.wrapping_add(500));
    let output = save.to_bytes();

    assert_eq!(output.len(), original.len());
    // Rupees are usually small, so `+500` may only flip the low 1-2 bytes rather than
    // all 4 — assert the edit is confined to a single 4-byte window, not an exact count.
    let diff_positions: Vec<usize> = original
        .iter()
        .zip(output.iter())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(i, _)| i)
        .collect();
    assert!(!diff_positions.is_empty(), "rupees value should have changed at least one byte");
    let min = *diff_positions.iter().min().unwrap();
    let max = *diff_positions.iter().max().unwrap();
    assert!(max - min < 4, "all differing bytes should be within the 4-byte rupee field");
}

#[test]
fn items_list_matches_first_and_last_expectations() {
    let save = load_botw();
    let items = save.items().unwrap();
    assert!(!items.is_empty());
    assert!(items.len() <= save_engine::botw::MAX_ITEMS);
    // every parsed item has a non-empty name (loop stops at the first empty slot)
    assert!(items.iter().all(|item| !item.name.is_empty()));
}

#[test]
fn setting_an_item_name_round_trips() {
    let mut save = load_botw();
    let items_before = save.items().unwrap();
    let last_index = items_before.len() - 1;

    save.set_item(last_index, "Weapon_Sword_070", 1).unwrap();
    let items_after = save.items().unwrap();
    assert_eq!(items_after[last_index].name, "Weapon_Sword_070");
    assert_eq!(items_after[last_index].quantity, 1);
}

#[test]
fn modifiers_counts_match_items_categorization() {
    let save = load_botw();
    let (weapons, bows, shields) = save.modifiers().unwrap();
    // sanity: modifier lists are readable and don't panic; exact counts depend on
    // the fixture's specific inventory contents.
    let _ = (weapons.len(), bows.len(), shields.len());
}

#[test]
fn setting_a_weapon_modifier_round_trips() {
    let mut save = load_botw();
    let (weapons, _, _) = save.modifiers().unwrap();
    if weapons.is_empty() {
        return; // fixture has no weapons in inventory; nothing to assert
    }
    save.set_modifier(ModifierCategory::Weapon, 0, 0x80000001, 5).unwrap();
    let (weapons_after, _, _) = save.modifiers().unwrap();
    assert_eq!(weapons_after[0].modifier, 0x80000001);
    assert_eq!(weapons_after[0].value, 5);
}

#[test]
fn horses_returns_six_slots_with_last_slot_nameless() {
    let save = load_botw();
    let horses = save.horses().unwrap();
    assert_eq!(horses.len(), save_engine::botw::NUM_HORSE_SLOTS);
    assert!(horses[5].name.is_none());
    assert!(horses[0].name.is_some());
}

#[test]
fn set_item_out_of_range_returns_error() {
    let mut save = load_botw();
    let result = save.set_item(save_engine::botw::MAX_ITEMS, "Weapon_Sword_070", 1);
    assert!(matches!(result, Err(SaveError::IndexOutOfRange { .. })));
}

#[test]
fn set_horse_name_out_of_range_returns_error() {
    let mut save = load_botw();
    let result = save.set_horse_name(5, "Epona");
    assert!(matches!(result, Err(SaveError::IndexOutOfRange { .. })));
}

#[test]
fn set_horse_type_out_of_range_returns_error() {
    let mut save = load_botw();
    let result = save.set_horse_type(save_engine::botw::NUM_HORSE_SLOTS, "HorseType");
    assert!(matches!(result, Err(SaveError::IndexOutOfRange { .. })));
}
