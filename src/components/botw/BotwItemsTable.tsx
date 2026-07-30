import { useState } from "react";
import type { BotwItem } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import { botwIconStyle, botwUnknownIconUrl } from "./botwIcons";
import styles from "./BotwItemsTable.module.css";

const ICON_SIZE = 32;

function ItemIcon({ item }: { item: BotwItem }) {
  const dyeColor = item.category === "armor" ? item.quantity : undefined;
  const style = botwIconStyle(item.name, dyeColor, ICON_SIZE);
  if (!style) return <img className={styles.icon} src={botwUnknownIconUrl} alt="" />;
  return <div className={styles.icon} style={style} />;
}

interface Props {
  items: BotwItem[];
  onError: (message: string) => void;
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

export default function BotwItemsTable({ items, onError }: Props) {
  return (
    <div>
      <SectionHeading title={`Items (${items.length})`} motif={<BotwMotif />} />
      <table className={styles.table}>
        <thead>
          <tr>
            <th></th>
            <th>#</th>
            <th>Name</th>
            <th>Quantity</th>
          </tr>
        </thead>
        <tbody>
          {items.map((item, i) => (
            <ItemRow key={i} index={i} item={item} onError={onError} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
