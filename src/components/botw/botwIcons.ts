import { ICON_COLS, ICONS, REPEAT_ARMOR_ICONS_1, REPEAT_ARMOR_ICONS_2 } from "./botwIcons.data";

export const botwUnknownIconUrl = "/icons/botw/_blank.png";

interface Resolved {
  sheet: string;
  slot: string;
}

// Ports `zelda-botw.icons.js`'s `_getItemIcon` classification: turns a save's raw item id into
// (sprite sheet, slot id within that sheet). Order matters, each branch's regex is checked in
// the same sequence as the source so earlier, more specific patterns win over the catch-all.
function classify(itemNameId: string): Resolved | null {
  let match: RegExpMatchArray | null;

  if ((match = itemNameId.match(/^Armor_([0-9]{3})_(Head|Upper|Lower)(_Dye[0-9]{2})?/))) {
    const sheet = "Armor_" + match[2];
    let slot = match[1];
    if (match[2] === "Lower" && slot === "140") slot = "141";
    const repeatIndex = REPEAT_ARMOR_ICONS_1.indexOf(slot);
    if (repeatIndex >= 0) slot = REPEAT_ARMOR_ICONS_2[Math.floor(repeatIndex / 4)];
    return { sheet, slot };
  }

  if ((match = itemNameId.match(/^Item_(Chilled|ChilledFish|Enemy|Material|Meat|Roast|RoastFish)_([0-9]{2})/))) {
    return { sheet: "Item_" + match[1], slot: match[2] };
  }

  if (itemNameId === "Item_Enemy_Put_57") {
    return { sheet: "Item_Enemy", slot: "Put_57" };
  }

  const normalized = itemNameId
    .replace("Animal_Insect_", "Item_InsectGet_")
    .replace("Mushroom_", "MushroomGet_")
    .replace("Plant_", "PlantGet_");
  if ((match = normalized.match(/^Item_(FishGet|Fruit|InsectGet|MushroomGet|Ore|PlantGet)_([A-Z][A-B]?)/))) {
    const sheet = "Item_" + match[1];
    let slot = match[2];
    if ((slot === "K" || slot === "O" || slot === "Z") && itemNameId.startsWith("Animal_Insect_")) {
      slot = "Normal" + slot;
    }
    return { sheet, slot };
  }

  if ((match = itemNameId.match(/^Item_Cook_([A-P]_[0-9]{2})/))) {
    return { sheet: "Item_Cook", slot: match[1] };
  }

  if ((match = itemNameId.match(/^Weapon_(Sword|Lsword|Spear|Bow|Shield)_([0-9]{3})/))) {
    return { sheet: "Weapon_" + match[1], slot: match[2] };
  }

  return { sheet: "Other", slot: itemNameId };
}

/**
 * CSS for a `<div>` rendering the given item's icon at `displaySize` pixels, sliced from its
 * sprite sheet via `background-position`. `background-size` scales the whole sheet so each cell
 * lands on exactly `displaySize`, without needing to know the sheet's row count up front (every
 * sheet is `ICON_COLS` cells wide, so scaling width to `ICON_COLS * displaySize` and letting
 * height follow via `auto` scales every cell by the same factor).
 */
export function botwIconStyle(itemNameId: string, dyeColor: number | undefined, displaySize: number) {
  const resolved = classify(itemNameId);
  if (!resolved) return null;

  const slots = ICONS[resolved.sheet];
  let index = slots ? slots.indexOf(resolved.slot) : -1;
  if (index === -1) return null;

  if (itemNameId.startsWith("Armor_") && dyeColor && dyeColor <= 15) {
    index += dyeColor;
  }

  const col = index % ICON_COLS;
  const row = Math.floor(index / ICON_COLS);
  return {
    backgroundImage: `url(/icons/botw/${resolved.sheet}.png)`,
    backgroundSize: `${ICON_COLS * displaySize}px auto`,
    backgroundPosition: `${-(col * displaySize)}px ${-(row * displaySize)}px`,
    width: displaySize,
    height: displaySize,
  };
}
