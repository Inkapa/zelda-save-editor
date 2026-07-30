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
import { totkArmorIconUrl, totkIconUrl } from "./totkIcons";
import { TOTK_ITEM_NAMES } from "./totkItemNames.data";
import {
  WEAPON_MODIFIER_OPTIONS,
  BOW_MODIFIER_OPTIONS,
  SHIELD_MODIFIER_OPTIONS,
  FOOD_EFFECT_OPTIONS,
  DYE_COLOR_OPTIONS,
  weaponModifierIconUrl,
  bowModifierIconUrl,
  shieldModifierIconUrl,
  foodEffectIconUrl,
} from "./totkEnums.data";

function nameFor(entry: { id: string }): string | undefined {
  return TOTK_ITEM_NAMES[entry.id];
}

// Every known item id, searchable by name or raw id via the browser's native <datalist>
// autocomplete, shared by the "Id" column on every category table plus weapon/shield fuse
// targets, so picking a real item doesn't mean typing its exact internal id from memory.
const ITEM_DATALIST = {
  id: "totk-item-ids",
  options: Object.entries(TOTK_ITEM_NAMES).map(([value, label]) => ({ value, label: `${label} (${value})` })),
};

function text<T>(key: keyof T, label: string): ColumnDef<T> {
  return { key, label, type: "text" };
}

function idColumn<T extends { id: string }>(): ColumnDef<T> {
  return { key: "id" as keyof T, label: "Id", type: "text", datalist: ITEM_DATALIST };
}

function num<T>(key: keyof T, label: string): ColumnDef<T> {
  return { key, label, type: "number", parse: (raw) => Number(raw) as T[keyof T] };
}

const weaponColumns: ColumnDef<TotkWeapon>[] = [
  idColumn<TotkWeapon>(),
  num("durability", "Durability"),
  { key: "modifier", label: "Modifier", type: "select", options: WEAPON_MODIFIER_OPTIONS },
  { ...num("modifier_value", "Modifier Value"), iconFor: (e) => weaponModifierIconUrl(e.modifier) },
  { key: "fuse_id", label: "Fuse Id", type: "text", datalist: ITEM_DATALIST },
  num("fuse_durability", "Fuse Durability"),
  num("extra_durability", "Extra Durability"),
  num("record_extra_durability", "Record Extra Durability"),
];

const bowColumns: ColumnDef<TotkBow>[] = [
  idColumn<TotkBow>(),
  num("durability", "Durability"),
  { key: "modifier", label: "Modifier", type: "select", options: BOW_MODIFIER_OPTIONS },
  { ...num("modifier_value", "Modifier Value"), iconFor: (e) => bowModifierIconUrl(e.modifier) },
];

const shieldColumns: ColumnDef<TotkShield>[] = [
  idColumn<TotkShield>(),
  num("durability", "Durability"),
  { key: "modifier", label: "Modifier", type: "select", options: SHIELD_MODIFIER_OPTIONS },
  { ...num("modifier_value", "Modifier Value"), iconFor: (e) => shieldModifierIconUrl(e.modifier) },
  { key: "fuse_id", label: "Fuse Id", type: "text", datalist: ITEM_DATALIST },
  num("fuse_durability", "Fuse Durability"),
  num("extra_durability", "Extra Durability"),
];

const armorColumns: ColumnDef<TotkArmor>[] = [
  idColumn<TotkArmor>(),
  { key: "dye_color", label: "Dye Color", type: "select", options: DYE_COLOR_OPTIONS },
];

const arrowColumns: ColumnDef<TotkArrow>[] = [idColumn<TotkArrow>(), num("quantity", "Quantity")];

const materialColumns: ColumnDef<TotkMaterial>[] = [
  idColumn<TotkMaterial>(),
  num("quantity", "Quantity"),
  num("get_order", "Get Order"),
  num("use_order", "Use Order"),
];

const keyItemColumns: ColumnDef<TotkKeyItem>[] = [idColumn<TotkKeyItem>(), num("quantity", "Quantity")];

const deviceColumns: ColumnDef<TotkDevice>[] = [
  idColumn<TotkDevice>(),
  num("quantity", "Quantity"),
  num("use_order", "Use Order"),
];

const foodColumns: ColumnDef<TotkFood>[] = [
  idColumn<TotkFood>(),
  num("quantity", "Quantity"),
  num("hearts_heal", "Hearts Heal"),
  { key: "effect", label: "Food Effect", type: "select", options: FOOD_EFFECT_OPTIONS },
  { ...num("effect_multiplier", "Multiplier"), iconFor: (e) => foodEffectIconUrl(e.effect) },
  num("effect_time", "Duration (in seconds)"),
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

interface ErrorProp {
  onError: (message: string) => void;
}

export function TotkWeaponsTable({ weapons, onError }: { weapons: TotkWeapon[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Weapons"
      entries={weapons}
      columns={weaponColumns}
      setter={api.setPouchWeapons}
      onError={onError}
      iconFor={(e) => totkIconUrl("weapon", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkBowsTable({ bows, onError }: { bows: TotkBow[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Bows"
      entries={bows}
      columns={bowColumns}
      setter={api.setPouchBows}
      onError={onError}
      iconFor={(e) => totkIconUrl("bow", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkShieldsTable({ shields, onError }: { shields: TotkShield[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Shields"
      entries={shields}
      columns={shieldColumns}
      setter={api.setPouchShields}
      onError={onError}
      iconFor={(e) => totkIconUrl("shield", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkArmorTable({ armor, onError }: { armor: TotkArmor[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Armor"
      entries={armor}
      columns={armorColumns}
      setter={api.setArmor}
      onError={onError}
      iconFor={(e) => totkArmorIconUrl(e.id, e.dye_color)}
      nameFor={nameFor}
    />
  );
}

export function TotkArrowsTable({ arrows, onError }: { arrows: TotkArrow[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Arrows"
      entries={arrows}
      columns={arrowColumns}
      setter={api.setArrows}
      onError={onError}
      iconFor={(e) => totkIconUrl("arrow", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkMaterialsTable({ materials, onError }: { materials: TotkMaterial[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Materials"
      entries={materials}
      columns={materialColumns}
      setter={api.setMaterials}
      onError={onError}
      iconFor={(e) => totkIconUrl("material", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkKeyItemsTable({ keyItems, onError }: { keyItems: TotkKeyItem[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Key Items"
      entries={keyItems}
      columns={keyItemColumns}
      setter={api.setKeyItems}
      onError={onError}
      iconFor={(e) => totkIconUrl("keyItem", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkDevicesTable({ devices, onError }: { devices: TotkDevice[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Devices"
      entries={devices}
      columns={deviceColumns}
      setter={api.setDevices}
      onError={onError}
      iconFor={(e) => totkIconUrl("device", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkFoodTable({ food, onError }: { food: TotkFood[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Food"
      entries={food}
      columns={foodColumns}
      setter={api.setFood}
      onError={onError}
      iconFor={(e) => totkIconUrl("food", e.id)}
      nameFor={nameFor}
    />
  );
}
