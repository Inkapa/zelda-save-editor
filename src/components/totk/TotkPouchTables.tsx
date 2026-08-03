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
import { totkArmorIconUrl, totkIconUrl, type TotkIconCategory } from "./totkIcons";
import { TOTK_ITEM_NAMES } from "./totkItemNames.data";
import { TOTK_CATEGORY_ITEMS } from "./totkCategoryItems.data";
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

// A category-restricted item picker for the "Id" column: shows an icon + name dropdown listing
// only that category's items, in place of a blind free-text field.
function idPicker<T extends { id: string }>(category: TotkIconCategory): ColumnDef<T> {
  const iconFor =
    category === "armor"
      ? (id: string) => totkArmorIconUrl(id, 0)
      : (id: string) => totkIconUrl(category, id);
  return {
    key: "id" as keyof T,
    label: "Id",
    type: "itempicker",
    picker: { items: TOTK_CATEGORY_ITEMS[category], iconFor, nameFor: (id) => TOTK_ITEM_NAMES[id] },
  };
}

function num<T>(key: keyof T, label: string): ColumnDef<T> {
  return { key, label, type: "number", parse: (raw) => Number(raw) as T[keyof T] };
}

// Append a new entry to a category, defaulting to the next id after the last row (wrapping),
// copying the last row's other fields so the new item is otherwise valid. Mirrors marcrobledo's
// addItem next-id cycling. `blank` is only used when the category is currently empty.
function addEntry<T extends { id: string }>(
  entries: T[],
  category: TotkIconCategory,
  blank: Omit<T, "id">,
  setter: (entries: T[]) => Promise<void>,
  onError: (message: string) => void,
) {
  const items = TOTK_CATEGORY_ITEMS[category];
  const last = entries[entries.length - 1];
  const nextId = last ? items[(items.indexOf(last.id) + 1) % items.length] : items[0];
  const next = { ...(last ?? (blank as T)), id: nextId } as T;
  setter([...entries, next]).catch((err) => onError(String(err)));
}

// Field defaults used only when adding to a currently-empty category (otherwise the last row's
// fields are copied).
const BLANK_WEAPON: Omit<TotkWeapon, "id"> = { durability: 0, modifier: 0, modifier_value: 0, fuse_id: "", fuse_durability: 0, extra_durability: 0, record_extra_durability: 0 };
const BLANK_BOW: Omit<TotkBow, "id"> = { durability: 0, modifier: 0, modifier_value: 0 };
const BLANK_SHIELD: Omit<TotkShield, "id"> = { durability: 0, modifier: 0, modifier_value: 0, fuse_id: "", fuse_durability: 0, extra_durability: 0 };
const BLANK_ARMOR: Omit<TotkArmor, "id"> = { dye_color: 0 };
const BLANK_ARROW: Omit<TotkArrow, "id"> = { quantity: 1 };
const BLANK_MATERIAL: Omit<TotkMaterial, "id"> = { quantity: 1, get_order: 0, use_order: 0 };
const BLANK_KEYITEM: Omit<TotkKeyItem, "id"> = { quantity: 1 };
const BLANK_DEVICE: Omit<TotkDevice, "id"> = { quantity: 1, use_order: 0 };
const BLANK_FOOD: Omit<TotkFood, "id"> = { quantity: 1, hearts_heal: 0, effect: 0, effect_multiplier: 0, effect_time: 0, price: 0, recipe: ["", "", "", "", ""] };

const weaponColumns: ColumnDef<TotkWeapon>[] = [
  idPicker<TotkWeapon>("weapon"),
  num("durability", "Durability"),
  { key: "modifier", label: "Modifier", type: "select", options: WEAPON_MODIFIER_OPTIONS },
  { ...num("modifier_value", "Modifier Value"), iconFor: (e) => weaponModifierIconUrl(e.modifier) },
  { key: "fuse_id", label: "Fuse Id", type: "text", datalist: ITEM_DATALIST },
  num("fuse_durability", "Fuse Durability"),
  num("extra_durability", "Extra Durability"),
  num("record_extra_durability", "Record Extra Durability"),
];

const bowColumns: ColumnDef<TotkBow>[] = [
  idPicker<TotkBow>("bow"),
  num("durability", "Durability"),
  { key: "modifier", label: "Modifier", type: "select", options: BOW_MODIFIER_OPTIONS },
  { ...num("modifier_value", "Modifier Value"), iconFor: (e) => bowModifierIconUrl(e.modifier) },
];

const shieldColumns: ColumnDef<TotkShield>[] = [
  idPicker<TotkShield>("shield"),
  num("durability", "Durability"),
  { key: "modifier", label: "Modifier", type: "select", options: SHIELD_MODIFIER_OPTIONS },
  { ...num("modifier_value", "Modifier Value"), iconFor: (e) => shieldModifierIconUrl(e.modifier) },
  { key: "fuse_id", label: "Fuse Id", type: "text", datalist: ITEM_DATALIST },
  num("fuse_durability", "Fuse Durability"),
  num("extra_durability", "Extra Durability"),
];

const armorColumns: ColumnDef<TotkArmor>[] = [
  idPicker<TotkArmor>("armor"),
  { key: "dye_color", label: "Dye Color", type: "select", options: DYE_COLOR_OPTIONS },
];

const arrowColumns: ColumnDef<TotkArrow>[] = [idPicker<TotkArrow>("arrow"), num("quantity", "Quantity")];

const materialColumns: ColumnDef<TotkMaterial>[] = [
  idPicker<TotkMaterial>("material"),
  num("quantity", "Quantity"),
  num("get_order", "Get Order"),
  num("use_order", "Use Order"),
];

const keyItemColumns: ColumnDef<TotkKeyItem>[] = [idPicker<TotkKeyItem>("keyItem"), num("quantity", "Quantity")];

const deviceColumns: ColumnDef<TotkDevice>[] = [
  idPicker<TotkDevice>("device"),
  num("quantity", "Quantity"),
  num("use_order", "Use Order"),
];

const foodColumns: ColumnDef<TotkFood>[] = [
  idPicker<TotkFood>("food"),
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
      onAdd={() => addEntry(weapons, "weapon", BLANK_WEAPON, api.setPouchWeapons, onError)}
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
      onAdd={() => addEntry(bows, "bow", BLANK_BOW, api.setPouchBows, onError)}
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
      onAdd={() => addEntry(shields, "shield", BLANK_SHIELD, api.setPouchShields, onError)}
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
      onAdd={() => addEntry(armor, "armor", BLANK_ARMOR, api.setArmor, onError)}
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
      onAdd={() => addEntry(arrows, "arrow", BLANK_ARROW, api.setArrows, onError)}
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
      onAdd={() => addEntry(materials, "material", BLANK_MATERIAL, api.setMaterials, onError)}
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
      onAdd={() => addEntry(keyItems, "keyItem", BLANK_KEYITEM, api.setKeyItems, onError)}
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
      onAdd={() => addEntry(devices, "device", BLANK_DEVICE, api.setDevices, onError)}
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
      onAdd={() => addEntry(food, "food", BLANK_FOOD, api.setFood, onError)}
      iconFor={(e) => totkIconUrl("food", e.id)}
      nameFor={nameFor}
    />
  );
}
