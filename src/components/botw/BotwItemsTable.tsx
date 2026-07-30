import { useState } from "react";
import type { BotwItem, BotwItemCategory, ItemModifier, ModifierCategory } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import { botwIconStyle, botwUnknownIconUrl } from "./botwIcons";
import { BOTW_ITEM_NAMES } from "./botwItemNames.data";
import { MODIFIER_OPTIONS, DYE_COLOR_OPTIONS } from "./botwEnums.data";
import { withCurrentValue } from "../Select";
import styles from "./BotwItemsTable.module.css";

const ICON_SIZE = 48;

const ITEM_DATALIST_ID = "botw-item-ids";
const ITEM_DATALIST_OPTIONS = Object.entries(BOTW_ITEM_NAMES).map(([value, label]) => ({
  value,
  label: `${label} (${value})`,
}));

function ItemIcon({ item }: { item: BotwItem }) {
  const dyeColor = item.category === "armor" ? item.quantity : undefined;
  const style = botwIconStyle(item.name, dyeColor, ICON_SIZE);
  if (!style) return <img className={styles.icon} src={botwUnknownIconUrl} alt="" style={{ width: ICON_SIZE, height: ICON_SIZE }} />;
  return (
    <div className={styles.icon} style={style.wrapper}>
      <div style={style.cell} />
    </div>
  );
}

function ModifierCell({
  category,
  categoryIndex,
  modifier,
  onError,
}: {
  category: ModifierCategory;
  categoryIndex: number;
  modifier: ItemModifier;
  onError: (message: string) => void;
}) {
  const [value, setValue] = useState(String(modifier.value));
  const commitModifier = (raw: string) =>
    api.setModifier(category, categoryIndex, Number(raw), modifier.value).catch((err) => onError(String(err)));
  const commitValue = () =>
    api.setModifier(category, categoryIndex, modifier.modifier, Number(value)).catch((err) => onError(String(err)));
  return (
    <>
      <td>
        <select
          className={styles.input}
          value={String(modifier.modifier)}
          onChange={(e) => commitModifier(e.target.value)}
        >
          {withCurrentValue(MODIFIER_OPTIONS, modifier.modifier).map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>
      </td>
      <td>
        <input
          className={styles.input}
          type="number"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onBlur={commitValue}
        />
      </td>
    </>
  );
}

function ItemRow({
  index,
  item,
  valueLabel,
  modifierCategory,
  modifier,
  categoryIndex,
  onError,
}: {
  index: number;
  item: BotwItem;
  valueLabel: string;
  modifierCategory: ModifierCategory | null;
  modifier: ItemModifier | undefined;
  categoryIndex: number;
  onError: (message: string) => void;
}) {
  const [name, setName] = useState(item.name);
  const [quantity, setQuantity] = useState(String(item.quantity));
  const commit = () =>
    api.setItem(index, name, Number(quantity)).catch((err) => onError(String(err)));
  const isDye = valueLabel === "Dye Color";

  return (
    <tr>
      <td>
        <ItemIcon item={item} />
      </td>
      <td>{index}</td>
      <td>{BOTW_ITEM_NAMES[item.name] ?? ""}</td>
      <td>
        <input
          className={styles.input}
          value={name}
          onChange={(e) => setName(e.target.value)}
          onBlur={commit}
          list={ITEM_DATALIST_ID}
        />
      </td>
      <td>
        {isDye ? (
          <select
            className={styles.input}
            value={quantity}
            onChange={(e) => {
              setQuantity(e.target.value);
              api.setItem(index, name, Number(e.target.value)).catch((err) => onError(String(err)));
            }}
          >
            {withCurrentValue(DYE_COLOR_OPTIONS, Number(quantity)).map((o) => (
              <option key={o.value} value={o.value}>
                {o.label}
              </option>
            ))}
          </select>
        ) : (
          <input
            className={styles.input}
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(e.target.value)}
            onBlur={commit}
          />
        )}
      </td>
      {modifierCategory &&
        (modifier ? (
          <ModifierCell category={modifierCategory} categoryIndex={categoryIndex} modifier={modifier} onError={onError} />
        ) : (
          <>
            <td />
            <td />
          </>
        ))}
    </tr>
  );
}

interface CategoryTableProps {
  title: string;
  valueLabel: string;
  items: BotwItem[];
  category: BotwItemCategory;
  modifiers?: ItemModifier[];
  onError: (message: string) => void;
}

export function BotwCategoryTable({ title, valueLabel, items, category, modifiers, onError }: CategoryTableProps) {
  const entries = items.map((item, index) => ({ item, index })).filter(({ item }) => item.category === category);
  const modifierCategory: ModifierCategory | null =
    category === "weapon" || category === "bow" || category === "shield" ? category : null;

  return (
    <div>
      <SectionHeading title={`${title} (${entries.length})`} motif={<BotwMotif />} />
      <datalist id={ITEM_DATALIST_ID}>
        {ITEM_DATALIST_OPTIONS.map((o) => (
          <option key={o.value} value={o.value}>
            {o.label}
          </option>
        ))}
      </datalist>
      <table className={styles.table}>
        <thead>
          <tr>
            <th></th>
            <th>#</th>
            <th>Name</th>
            <th>Id</th>
            <th>{valueLabel}</th>
            {modifierCategory && (
              <>
                <th>Modifier</th>
                <th>Modifier Value</th>
              </>
            )}
          </tr>
        </thead>
        <tbody>
          {entries.map(({ item, index }, categoryIndex) => (
            <ItemRow
              key={index}
              index={index}
              item={item}
              valueLabel={valueLabel}
              modifierCategory={modifierCategory}
              modifier={modifiers?.[categoryIndex]}
              categoryIndex={categoryIndex}
              onError={onError}
            />
          ))}
        </tbody>
      </table>
    </div>
  );
}
