import { invoke } from "@tauri-apps/api/core";
import type { BotwState, ModifierCategory, OpenResult, TotkState } from "./types";

export function openSave(): Promise<OpenResult> {
  return invoke("open_save");
}

export function saveSave(): Promise<void> {
  return invoke("save");
}

export function saveAs(): Promise<void> {
  return invoke("save_as");
}

// --- BOTW ---

export function getBotwState(): Promise<BotwState> {
  return invoke("get_botw_state");
}

export function setRupees(val: number): Promise<void> {
  return invoke("set_rupees", { val });
}

export function setMons(val: number): Promise<void> {
  return invoke("set_mons", { val });
}

export function setMaxHearts(val: number): Promise<void> {
  return invoke("set_max_hearts", { val });
}

export function setMaxStaminaBotw(val: number): Promise<void> {
  return invoke("set_max_stamina", { val });
}

export function setRelicGerudo(val: number): Promise<void> {
  return invoke("set_relic_gerudo", { val });
}

export function setRelicGoron(val: number): Promise<void> {
  return invoke("set_relic_goron", { val });
}

export function setRelicRito(val: number): Promise<void> {
  return invoke("set_relic_rito", { val });
}

export function setKorokSeedCounter(val: number): Promise<void> {
  return invoke("set_korok_seed_counter", { val });
}

export function setDefeatedHinoxCounter(val: number): Promise<void> {
  return invoke("set_defeated_hinox_counter", { val });
}

export function setDefeatedTalusCounter(val: number): Promise<void> {
  return invoke("set_defeated_talus_counter", { val });
}

export function setDefeatedMoldugaCounter(val: number): Promise<void> {
  return invoke("set_defeated_molduga_counter", { val });
}

export function setPlaytimeSeconds(val: number): Promise<void> {
  return invoke("set_playtime_seconds", { val });
}

export function setMotorcycle(val: boolean): Promise<void> {
  return invoke("set_motorcycle", { val });
}

export function setPlayerPosition(x: number, y: number, z: number): Promise<void> {
  return invoke("set_player_position", { x, y, z });
}

export function setHorsePosition(x: number, y: number, z: number): Promise<void> {
  return invoke("set_horse_position", { x, y, z });
}

export function setMap(value: string): Promise<void> {
  return invoke("set_map", { value });
}

export function setMapType(value: string): Promise<void> {
  return invoke("set_map_type", { value });
}

export function setItem(index: number, name: string, quantity: number): Promise<void> {
  return invoke("set_item", { index, name, quantity });
}

export function setModifier(
  category: ModifierCategory,
  index: number,
  modifier: number,
  value: number,
): Promise<void> {
  return invoke("set_modifier", { category, index, modifier, value });
}

export function setHorseName(index: number, value: string): Promise<void> {
  return invoke("set_horse_name", { index, value });
}

export function setHorseSaddle(index: number, value: string): Promise<void> {
  return invoke("set_horse_saddle", { index, value });
}

export function setHorseReins(index: number, value: string): Promise<void> {
  return invoke("set_horse_reins", { index, value });
}

export function setHorseType(index: number, value: string): Promise<void> {
  return invoke("set_horse_type", { index, value });
}

// --- TOTK ---

export function getTotkState(): Promise<TotkState> {
  return invoke("get_totk_state");
}

export function setMaxLife(val: number): Promise<void> {
  return invoke("set_max_life", { val });
}

export function setCurrentRupees(val: number): Promise<void> {
  return invoke("set_current_rupees", { val });
}

export function setMaxStaminaTotk(val: number): Promise<void> {
  return invoke("totk_set_max_stamina", { val });
}

export function setMaxEnergy(val: number): Promise<void> {
  return invoke("set_max_energy", { val });
}

export function setPlaytime(val: number): Promise<void> {
  return invoke("set_playtime", { val });
}

export function setHorseInnMemberPoint(val: number): Promise<void> {
  return invoke("set_horse_inn_member_point", { val });
}

export function setSavePos(x: number, y: number, z: number): Promise<void> {
  return invoke("set_save_pos", { x, y, z });
}

export function setSequenceCurrentBanc(value: string): Promise<void> {
  return invoke("set_sequence_current_banc", { value });
}

export function setPouchWeaponValidNum(val: number): Promise<void> {
  return invoke("set_pouch_weapon_valid_num", { val });
}

export function setPouchBowValidNum(val: number): Promise<void> {
  return invoke("set_pouch_bow_valid_num", { val });
}

export function setPouchShieldValidNum(val: number): Promise<void> {
  return invoke("set_pouch_shield_valid_num", { val });
}
