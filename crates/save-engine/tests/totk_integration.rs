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
fn malformed_input_returns_err_instead_of_panicking() {
    // Right magic and in-range size, but all-zero content: no sentinel hash exists in
    // it, so the hash-table-end scan should fail gracefully rather than panic or scan
    // forever.
    let mut bytes = vec![0u8; 2307552];
    bytes[0..4].copy_from_slice(&0x01020304u32.to_le_bytes());
    let result = Save::detect(bytes);
    assert!(result.is_err());
}
