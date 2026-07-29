import type { TotkHorse } from "../../api";
import * as api from "../../api";
import EditableEntryTable, { ColumnDef } from "./EditableEntryTable";
import TotkMotif from "../../theme/motifs/TotkMotif";

function text(key: keyof TotkHorse, label: string): ColumnDef<TotkHorse> {
  return { key, label, type: "text" };
}

function num(key: keyof TotkHorse, label: string): ColumnDef<TotkHorse> {
  return { key, label, type: "number", parse: (raw) => Number(raw) as TotkHorse[keyof TotkHorse] };
}

// Appearance-only fields (icon_pattern, icon_eye_color, the five RGB tuples,
// amiibo_uid_hash) are left out of this table. They're cosmetic icon rendering data, not
// stats a save editor user would typically want to hand-edit. Add columns for them if that
// changes.
const columns: ColumnDef<TotkHorse>[] = [
  text("id", "Id"),
  text("name", "Name"),
  num("mane", "Mane"),
  num("saddle", "Saddle"),
  num("rein", "Rein"),
  num("bond", "Bond"),
  { key: "bond_checked", label: "Bond Checked", type: "checkbox" },
  num("stats_strength", "Strength"),
  num("stats_speed", "Speed"),
  num("stats_stamina", "Stamina"),
  num("stats_pull", "Pull"),
  num("horse_type", "Type"),
  num("color_type", "Color"),
  num("foot_type", "Foot"),
  num("room_id", "Room Id"),
];

interface Props {
  horses: TotkHorse[];
  onError: (message: string) => void;
}

export default function TotkHorsesTable({ horses, onError }: Props) {
  return (
    <EditableEntryTable
      title="Horses"
      entries={horses}
      columns={columns}
      setter={api.setHorses}
      onError={onError}
      level="h3"
      motif={<TotkMotif />}
    />
  );
}
