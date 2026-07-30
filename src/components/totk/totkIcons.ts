const CATEGORY_DIRS = {
  weapon: "weapons",
  bow: "bows",
  shield: "shields",
  armor: "armors",
  arrow: "arrows",
  material: "materials",
  keyItem: "key",
  device: "devices",
  food: "food",
} as const;

export type TotkIconCategory = keyof typeof CATEGORY_DIRS;

export const totkUnknownIconUrl = "/icons/totk/unknown.png";

export function totkIconUrl(category: TotkIconCategory, id: string): string {
  return `/icons/totk/${CATEGORY_DIRS[category]}/${id}.png`;
}
