import { useState } from "react";
import type { ItemModifier, ModifierCategory } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import styles from "./BotwModifiersTable.module.css";

interface Props {
  weaponModifiers: ItemModifier[];
  bowModifiers: ItemModifier[];
  shieldModifiers: ItemModifier[];
  onError: (message: string) => void;
}

function ModifierRow({
  category,
  index,
  modifier,
  onError,
}: {
  category: ModifierCategory;
  index: number;
  modifier: ItemModifier;
  onError: (message: string) => void;
}) {
  const [flag, setFlag] = useState(String(modifier.modifier));
  const [value, setValue] = useState(String(modifier.value));
  const commit = () =>
    api
      .setModifier(category, index, Number(flag), Number(value))
      .catch((err) => onError(String(err)));
  return (
    <tr>
      <td>{index}</td>
      <td>
        <input className={styles.input} value={flag} onChange={(e) => setFlag(e.target.value)} onBlur={commit} />
      </td>
      <td>
        <input className={styles.input} value={value} onChange={(e) => setValue(e.target.value)} onBlur={commit} />
      </td>
    </tr>
  );
}

function ModifierTable({
  title,
  category,
  modifiers,
  onError,
}: {
  title: string;
  category: ModifierCategory;
  modifiers: ItemModifier[];
  onError: (message: string) => void;
}) {
  return (
    <div>
      <SectionHeading level="h4" title={`${title} (${modifiers.length})`} />
      <table className={styles.table}>
        <thead>
          <tr>
            <th>#</th>
            <th>Modifier flags</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {modifiers.map((m, i) => (
            <ModifierRow key={i} category={category} index={i} modifier={m} onError={onError} />
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default function BotwModifiersTable({
  weaponModifiers,
  bowModifiers,
  shieldModifiers,
  onError,
}: Props) {
  return (
    <div>
      <SectionHeading title="Item Modifiers" motif={<BotwMotif />} />
      <ModifierTable title="Weapon" category="weapon" modifiers={weaponModifiers} onError={onError} />
      <ModifierTable title="Bow" category="bow" modifiers={bowModifiers} onError={onError} />
      <ModifierTable title="Shield" category="shield" modifiers={shieldModifiers} onError={onError} />
    </div>
  );
}
