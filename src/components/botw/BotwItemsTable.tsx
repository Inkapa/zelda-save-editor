import { useState } from "react";
import type { BotwItem, BotwItemCategory } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import { botwIconStyle, botwUnknownIconUrl } from "./botwIcons";
import styles from "./BotwItemsTable.module.css";

const ICON_SIZE = 32;

const CATEGORIES: { key: BotwItemCategory; title: string; quantityLabel: string }[] = [
  { key: "weapon", title: "Weapons", quantityLabel: "Quantity" },
  { key: "bow", title: "Bows", quantityLabel: "Quantity" },
  { key: "shield", title: "Shields", quantityLabel: "Quantity" },
  { key: "armor", title: "Armor", quantityLabel: "Dye Color" },
  { key: "material", title: "Materials", quantityLabel: "Quantity" },
  { key: "food", title: "Food", quantityLabel: "Quantity" },
  { key: "key_item", title: "Key Items", quantityLabel: "Quantity" },
];

function ItemIcon({ item }: { item: BotwItem }) {
  const dyeColor = item.category === "armor" ? item.quantity : undefined;
  const style = botwIconStyle(item.name, dyeColor, ICON_SIZE);
  if (!style) return <img className={styles.icon} src={botwUnknownIconUrl} alt="" />;
  return <div className={styles.icon} style={style} />;
}

function ItemRow({
  index,
  item,
  onError,
}: {
  index: number;
  item: BotwItem;
  onError: (message: string) => void;
}) {
  const [name, setName] = useState(item.name);
  const [quantity, setQuantity] = useState(String(item.quantity));
  const commit = () =>
    api.setItem(index, name, Number(quantity)).catch((err) => onError(String(err)));
  return (
    <tr>
      <td>
        <ItemIcon item={item} />
      </td>
      <td>{index}</td>
      <td>
        <input className={styles.input} value={name} onChange={(e) => setName(e.target.value)} onBlur={commit} />
      </td>
      <td>
        <input
          className={styles.input}
          type="number"
          value={quantity}
          onChange={(e) => setQuantity(e.target.value)}
          onBlur={commit}
        />
      </td>
    </tr>
  );
}

function CategoryTable({
  title,
  quantityLabel,
  entries,
  onError,
}: {
  title: string;
  quantityLabel: string;
  entries: { item: BotwItem; index: number }[];
  onError: (message: string) => void;
}) {
  return (
    <div>
      <SectionHeading level="h4" title={`${title} (${entries.length})`} />
      <table className={styles.table}>
        <thead>
          <tr>
            <th></th>
            <th>#</th>
            <th>Name</th>
            <th>{quantityLabel}</th>
          </tr>
        </thead>
        <tbody>
          {entries.map(({ item, index }) => (
            <ItemRow key={index} index={index} item={item} onError={onError} />
          ))}
        </tbody>
      </table>
    </div>
  );
}

interface Props {
  items: BotwItem[];
  onError: (message: string) => void;
}

export default function BotwItemsTable({ items, onError }: Props) {
  return (
    <div>
      <SectionHeading title={`Items (${items.length})`} motif={<BotwMotif />} />
      {CATEGORIES.map((cat) => (
        <CategoryTable
          key={cat.key}
          title={cat.title}
          quantityLabel={cat.quantityLabel}
          entries={items.map((item, index) => ({ item, index })).filter(({ item }) => item.category === cat.key)}
          onError={onError}
        />
      ))}
    </div>
  );
}
