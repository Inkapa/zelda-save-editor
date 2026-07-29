export interface BotwItem {
  name: string;
  quantity: number;
}

export interface ItemModifier {
  modifier: number;
  value: number;
}

export interface BotwHorse {
  name: string | null;
  saddle: string | null;
  reins: string | null;
  horse_type: string;
}

export type ModifierCategory = "weapon" | "bow" | "shield";

export interface BotwState {
  rupees: number;
  mons: number;
  max_hearts: number;
  max_stamina: number;
  relic_gerudo: number;
  relic_goron: number;
  relic_rito: number;
  korok_seed_counter: number;
  defeated_hinox_counter: number;
  defeated_talus_counter: number;
  defeated_molduga_counter: number;
  playtime_seconds: number;
  motorcycle: boolean | null;
  player_position: [number, number, number];
  horse_position: [number, number, number];
  map: string;
  map_type: string;
  items: BotwItem[];
  weapon_modifiers: ItemModifier[];
  bow_modifiers: ItemModifier[];
  shield_modifiers: ItemModifier[];
  horses: BotwHorse[];
}

export interface TotkState {
  max_life: number;
  current_rupees: number;
  max_stamina: number;
  max_energy: number;
  playtime: number;
  horse_inn_member_point: number;
  save_pos: [number, number, number];
  sequence_current_banc: string;
  pouch_weapon_valid_num: number;
  pouch_bow_valid_num: number;
  pouch_shield_valid_num: number;
}

export type OpenResult =
  | { kind: "botw"; state: BotwState }
  | { kind: "totk"; state: TotkState };

export interface ShellError {
  kind: string;
  message: string;
}
