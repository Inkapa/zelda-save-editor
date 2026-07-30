import type {
  TotkArmor,
  TotkArrow,
  TotkBow,
  TotkDevice,
  TotkFood,
  TotkKeyItem,
  TotkMaterial,
  TotkShield,
  TotkWeapon,
} from "../../api";
import * as api from "../../api";
import EditableEntryTable, { ColumnDef } from "./EditableEntryTable";
import SectionHeading from "../../theme/SectionHeading";
import TotkMotif from "../../theme/motifs/TotkMotif";
import { totkArmorIconUrl, totkIconUrl } from "./totkIcons";

function text<T>(key: keyof T, label: string): ColumnDef<T> {
  return { key, label, type: "text" };
}

function num<T>(key: keyof T, label: string): ColumnDef<T> {
  return { key, label, type: "number", parse: (raw) => Number(raw) as T[keyof T] };
}

const weaponColumns: ColumnDef<TotkWeapon>[] = [
  text("id", "Id"),
  num("durability", "Durability"),
  num("modifier", "Modifier"),
  num("modifier_value", "Modifier Value"),
  text("fuse_id", "Fuse Id"),
  num("fuse_durability", "Fuse Durability"),
  num("extra_durability", "Extra Durability"),
  num("record_extra_durability", "Record Extra Durability"),
];

const bowColumns: ColumnDef<TotkBow>[] = [
  text("id", "Id"),
  num("durability", "Durability"),
  num("modifier", "Modifier"),
  num("modifier_value", "Modifier Value"),
];

const shieldColumns: ColumnDef<TotkShield>[] = [
  text("id", "Id"),
  num("durability", "Durability"),
  num("modifier", "Modifier"),
  num("modifier_value", "Modifier Value"),
  text("fuse_id", "Fuse Id"),
  num("fuse_durability", "Fuse Durability"),
  num("extra_durability", "Extra Durability"),
];

const armorColumns: ColumnDef<TotkArmor>[] = [text("id", "Id"), num("dye_color", "Dye Color")];

const arrowColumns: ColumnDef<TotkArrow>[] = [text("id", "Id"), num("quantity", "Quantity")];

const materialColumns: ColumnDef<TotkMaterial>[] = [
  text("id", "Id"),
  num("quantity", "Quantity"),
  num("get_order", "Get Order"),
  num("use_order", "Use Order"),
];

const keyItemColumns: ColumnDef<TotkKeyItem>[] = [text("id", "Id"), num("quantity", "Quantity")];

const deviceColumns: ColumnDef<TotkDevice>[] = [
  text("id", "Id"),
  num("quantity", "Quantity"),
  num("use_order", "Use Order"),
];

const foodColumns: ColumnDef<TotkFood>[] = [
  text("id", "Id"),
  num("quantity", "Quantity"),
  num("hearts_heal", "Hearts Heal"),
  num("effect", "Effect"),
  num("effect_multiplier", "Effect Multiplier"),
  num("effect_time", "Effect Time"),
  num("price", "Price"),
  {
    key: "recipe",
    label: "Recipe (comma-separated)",
    format: (v) => (v as string[]).join(","),
    parse: (raw) => {
      const parts = raw.split(",").map((s) => s.trim());
      const padded = [parts[0] ?? "", parts[1] ?? "", parts[2] ?? "", parts[3] ?? "", parts[4] ?? ""];
      return padded as TotkFood["recipe"];
    },
  },
];

interface Props {
  weapons: TotkWeapon[];
  bows: TotkBow[];
  shields: TotkShield[];
  armor: TotkArmor[];
  arrows: TotkArrow[];
  materials: TotkMaterial[];
  keyItems: TotkKeyItem[];
  devices: TotkDevice[];
  food: TotkFood[];
  onError: (message: string) => void;
}

export default function TotkPouchTables({
  weapons,
  bows,
  shields,
  armor,
  arrows,
  materials,
  keyItems,
  devices,
  food,
  onError,
}: Props) {
  return (
    <div>
      <SectionHeading title="Pouch" motif={<TotkMotif />} />
      <EditableEntryTable
        title="Weapons"
        entries={weapons}
        columns={weaponColumns}
        setter={api.setPouchWeapons}
        onError={onError}
        iconFor={(e) => totkIconUrl("weapon", e.id)}
      />
      <EditableEntryTable
        title="Bows"
        entries={bows}
        columns={bowColumns}
        setter={api.setPouchBows}
        onError={onError}
        iconFor={(e) => totkIconUrl("bow", e.id)}
      />
      <EditableEntryTable
        title="Shields"
        entries={shields}
        columns={shieldColumns}
        setter={api.setPouchShields}
        onError={onError}
        iconFor={(e) => totkIconUrl("shield", e.id)}
      />
      <EditableEntryTable
        title="Armor"
        entries={armor}
        columns={armorColumns}
        setter={api.setArmor}
        onError={onError}
        iconFor={(e) => totkArmorIconUrl(e.id, e.dye_color)}
      />
      <EditableEntryTable
        title="Arrows"
        entries={arrows}
        columns={arrowColumns}
        setter={api.setArrows}
        onError={onError}
        iconFor={(e) => totkIconUrl("arrow", e.id)}
      />
      <EditableEntryTable
        title="Materials"
        entries={materials}
        columns={materialColumns}
        setter={api.setMaterials}
        onError={onError}
        iconFor={(e) => totkIconUrl("material", e.id)}
      />
      <EditableEntryTable
        title="Key Items"
        entries={keyItems}
        columns={keyItemColumns}
        setter={api.setKeyItems}
        onError={onError}
        iconFor={(e) => totkIconUrl("keyItem", e.id)}
      />
      <EditableEntryTable
        title="Devices"
        entries={devices}
        columns={deviceColumns}
        setter={api.setDevices}
        onError={onError}
        iconFor={(e) => totkIconUrl("device", e.id)}
      />
      <EditableEntryTable
        title="Food"
        entries={food}
        columns={foodColumns}
        setter={api.setFood}
        onError={onError}
        iconFor={(e) => totkIconUrl("food", e.id)}
      />
    </div>
  );
}
