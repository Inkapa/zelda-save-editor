use save_engine::totk::pouch;
use save_engine::Save;

fn fixture_bytes() -> Vec<u8> {
    std::fs::read("tests/fixtures/totk/progress.sav").expect("fixture present")
}

fn load_totk() -> save_engine::totk::TotkSave {
    match Save::detect(fixture_bytes()).expect("should detect a known TOTK save") {
        Save::Totk(save) => save,
        Save::Botw(_) => panic!("TOTK fixture was misdetected as BOTW"),
    }
}

#[test]
fn detects_as_totk_v1_1_x_v1_2_x() {
    let save = load_totk();
    assert_eq!(save.version_label, "v1.1.x/v1.2.x");
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
fn editing_current_rupees_only_touches_the_rupee_field() {
    let original = fixture_bytes();
    let mut save = load_totk();
    let before = save.current_rupees().unwrap();
    save.set_current_rupees(before.wrapping_add(500)).unwrap();
    assert_eq!(save.current_rupees().unwrap(), before.wrapping_add(500));
    let output = save.to_bytes();

    assert_eq!(output.len(), original.len());
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
fn setting_save_pos_round_trips_through_pointer_indirection() {
    let mut save = load_totk();
    save.set_save_pos(100.5, 20.25, -300.75).unwrap();
    let (x, y, z) = save.save_pos().unwrap();
    assert_eq!((x, y, z), (100.5, 20.25, -300.75));
}

#[test]
fn setting_sequence_current_banc_round_trips_through_pointer_indirection() {
    let mut save = load_totk();
    save.set_sequence_current_banc("TestBanc001").unwrap();
    assert_eq!(save.sequence_current_banc().unwrap(), "TestBanc001");
}

#[test]
fn pouch_valid_num_accessors_round_trip() {
    let mut save = load_totk();
    save.set_pouch_weapon_valid_num(5).unwrap();
    save.set_pouch_bow_valid_num(3).unwrap();
    save.set_pouch_shield_valid_num(2).unwrap();
    assert_eq!(save.pouch_weapon_valid_num().unwrap(), 5);
    assert_eq!(save.pouch_bow_valid_num().unwrap(), 3);
    assert_eq!(save.pouch_shield_valid_num().unwrap(), 2);
}

#[test]
fn pouch_weapons_reads_real_fixture_contents() {
    let save = load_totk();
    let weapons = save.pouch_weapons().unwrap();
    assert_eq!(weapons.len(), 2);
    assert_eq!(weapons[0].id, "Weapon_Sword_070"); // Master Sword
    assert_eq!(weapons[0].durability, 40);
    assert_eq!(weapons[0].fuse_id, "");
    assert_eq!(weapons[1].id, "Weapon_Sword_166"); // Gloom Sword
    assert_eq!(weapons[1].durability, 40);
    assert_eq!(weapons[1].fuse_id, "Item_Enemy_226");
}

#[test]
fn set_pouch_weapons_round_trips_and_updates_valid_num() {
    let mut save = load_totk();
    let mut weapons = save.pouch_weapons().unwrap();
    weapons.push(pouch::WeaponEntry {
        id: "Weapon_Sword_001".into(),
        durability: 20,
        modifier: 0,
        modifier_value: 0,
        fuse_id: String::new(),
        fuse_durability: 0,
        extra_durability: 0,
        record_extra_durability: -1,
    });
    save.set_pouch_weapons(&weapons).unwrap();
    assert_eq!(save.pouch_weapon_valid_num().unwrap(), 3);
    let reloaded = save.pouch_weapons().unwrap();
    assert_eq!(reloaded.len(), 3);
    assert_eq!(reloaded[2].id, "Weapon_Sword_001");
}

#[test]
fn pouch_bows_reads_real_fixture_contents() {
    let save = load_totk();
    let bows = save.pouch_bows().unwrap();
    assert_eq!(bows.len(), 2);
    assert_eq!(bows[0].id, "Weapon_Bow_036");
    assert_eq!(bows[0].durability, 55);
    assert_eq!(bows[0].modifier, 0xdad10617);
    assert_eq!(bows[0].modifier_value, 10);
    assert_eq!(bows[1].id, "Weapon_Bow_013");
    assert_eq!(bows[1].durability, 17);
    assert_eq!(bows[1].modifier, 0xb6eede09); // hash32("None"): no modifier
    assert_eq!(bows[1].modifier_value, 0);
}

#[test]
fn set_pouch_bows_round_trips_and_updates_valid_num() {
    let mut save = load_totk();
    let mut bows = save.pouch_bows().unwrap();
    bows.push(pouch::BowEntry {
        id: "Weapon_Bow_001".into(),
        durability: 20,
        modifier: 0,
        modifier_value: 0,
    });
    save.set_pouch_bows(&bows).unwrap();
    assert_eq!(save.pouch_bow_valid_num().unwrap(), 3);
    let reloaded = save.pouch_bows().unwrap();
    assert_eq!(reloaded.len(), 3);
    assert_eq!(reloaded[2].id, "Weapon_Bow_001");
}

#[test]
fn pouch_shields_reads_real_fixture_contents() {
    let save = load_totk();
    let shields = save.pouch_shields().unwrap();
    assert_eq!(shields.len(), 2);
    assert_eq!(shields[0].id, "Weapon_Shield_057");
    assert_eq!(shields[0].durability, 90);
    assert_eq!(shields[0].modifier, 0xb6eede09); // hash32("None"): no modifier
    assert_eq!(shields[0].fuse_id, "");
    assert_eq!(shields[0].fuse_durability, -1);
    assert_eq!(shields[1].id, "Weapon_Shield_022");
    assert_eq!(shields[1].durability, 29);
    assert_eq!(shields[1].modifier, 0xb3c94e5);
    assert_eq!(shields[1].modifier_value, 6);
}

#[test]
fn set_pouch_shields_round_trips_and_updates_valid_num() {
    let mut save = load_totk();
    let mut shields = save.pouch_shields().unwrap();
    shields.push(pouch::ShieldEntry {
        id: "Weapon_Shield_001".into(),
        durability: 20,
        modifier: 0,
        modifier_value: 0,
        fuse_id: String::new(),
        fuse_durability: 0,
        extra_durability: 0,
    });
    save.set_pouch_shields(&shields).unwrap();
    assert_eq!(save.pouch_shield_valid_num().unwrap(), 3);
    let reloaded = save.pouch_shields().unwrap();
    assert_eq!(reloaded.len(), 3);
    assert_eq!(reloaded[2].id, "Weapon_Shield_001");
}

#[test]
fn pouch_armor_reads_real_fixture_contents() {
    let save = load_totk();
    let armor = save.armor().unwrap();
    assert_eq!(armor.len(), 87);
    assert_eq!(armor[0].id, "Armor_1043_Upper"); // Archaic Tunic
    assert_eq!(armor[0].dye_color, save_engine::totk::murmur3::hash32("None"));
    assert_eq!(armor[3].id, "Armor_003_Head");
}

#[test]
fn malformed_input_returns_err_instead_of_panicking() {
    // Right magic and in-range size, but all-zero content: no sentinel hash exists in
    // it, so the hash-table-end scan should fail gracefully rather than panic or scan
    // forever.
    let mut bytes = vec![0u8; 2307552];
    bytes[0..4].copy_from_slice(&0x01020304u32.to_le_bytes());
    let result = Save::detect(bytes);
    assert!(result.is_err());
}
