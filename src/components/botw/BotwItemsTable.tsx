import { useState } from "react";
import type { BotwItem } from "../../api";
import * as api from "../../api";

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
      <td>{index}</td>
      <td>
        <input value={name} onChange={(e) => setName(e.target.value)} onBlur={commit} />
      </td>
      <td>
        <input
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
      <h3>Items ({items.length})</h3>
      <table>
        <thead>
          <tr>
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
