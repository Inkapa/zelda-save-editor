import { useState, type ReactNode } from "react";
import SectionHeading from "../../theme/SectionHeading";
import { totkUnknownIconUrl } from "./totkIcons";
import { withCurrentValue, type SelectOption } from "../Select";
import styles from "./EditableEntryTable.module.css";

export type { SelectOption };

export interface DatalistDef {
  /** Must be unique within the page; shared by every row's input in this column via the
   * standard HTML `list` attribute, so the option list itself is rendered once per column
   * rather than duplicated per row. */
  id: string;
  options: { value: string; label: string }[];
}

export interface ColumnDef<T> {
  key: keyof T;
  label: string;
  type?: "text" | "number" | "checkbox" | "readonly" | "select";
  format?: (value: T[keyof T]) => string;
  parse?: (raw: string) => T[keyof T];
  options?: SelectOption[];
  /** Turns a text column into a searchable autocomplete (native <datalist>) instead of a blind
   * free-text field, for id-shaped values a human can't reasonably type from memory (fuse
   * targets, etc). Still a plain text input underneath, so an unrecognized/custom id can still
   * be typed directly. */
  datalist?: DatalistDef;
  /** Small icon shown next to this cell, derived from the whole row (e.g. a bonus icon
   * matching a sibling `modifier` column's current value). */
  iconFor?: (entry: T) => string | undefined;
}

interface CellProps<T> {
  entries: T[];
  index: number;
  column: ColumnDef<T>;
  setter: (entries: T[]) => Promise<void>;
  onError: (message: string) => void;
}

function defaultFormat(value: unknown): string {
  return value === null || value === undefined ? "" : String(value);
}

function replaceAt<T>(entries: T[], index: number, key: keyof T, value: T[keyof T]): T[] {
  return entries.map((e, i) => (i === index ? { ...e, [key]: value } : e));
}

function CellIcon<T>({ column, entry }: { column: ColumnDef<T>; entry: T }) {
  const url = column.iconFor?.(entry);
  if (!url) return null;
  return <img className={styles.bonusIcon} src={url} alt="" />;
}

function TextCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const format = column.format ?? defaultFormat;
  const [text, setText] = useState(format(entries[index][column.key]));
  const commit = () => {
    const parse = column.parse ?? ((raw: string) => raw as unknown as T[keyof T]);
    setter(replaceAt(entries, index, column.key, parse(text))).catch((err) => onError(String(err)));
  };
  return (
    <td>
      <div className={styles.cellRow}>
        <input
          className={styles.input}
          type={column.type === "number" ? "number" : "text"}
          value={text}
          onChange={(e) => setText(e.target.value)}
          onBlur={commit}
          list={column.datalist?.id}
        />
        <CellIcon column={column} entry={entries[index]} />
      </div>
    </td>
  );
}

function SelectCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const rawValue = entries[index][column.key] as unknown as number;
  const options = withCurrentValue(column.options ?? [], rawValue);
  const commit = (raw: string) => {
    const value = Number(raw) as T[keyof T];
    setter(replaceAt(entries, index, column.key, value)).catch((err) => onError(String(err)));
  };
  return (
    <td>
      <div className={styles.cellRow}>
        <select className={styles.select} value={String(rawValue)} onChange={(e) => commit(e.target.value)}>
          {options.map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>
        <CellIcon column={column} entry={entries[index]} />
      </div>
    </td>
  );
}

function CheckboxCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const checked = Boolean(entries[index][column.key]);
  const commit = (next: boolean) => {
    setter(replaceAt(entries, index, column.key, next as unknown as T[keyof T])).catch((err) =>
      onError(String(err)),
    );
  };
  return (
    <td>
      <input type="checkbox" checked={checked} onChange={(e) => commit(e.target.checked)} />
    </td>
  );
}

function ReadOnlyCell<T>({ entries, index, column }: CellProps<T>) {
  const format = column.format ?? defaultFormat;
  return <td>{format(entries[index][column.key])}</td>;
}

function Cell<T>(props: CellProps<T>) {
  if (props.column.type === "checkbox") return <CheckboxCell {...props} />;
  if (props.column.type === "readonly") return <ReadOnlyCell {...props} />;
  if (props.column.type === "select") return <SelectCell {...props} />;
  return <TextCell {...props} />;
}

interface Props<T> {
  title: string;
  entries: T[];
  columns: ColumnDef<T>[];
  setter: (entries: T[]) => Promise<void>;
  onError: (message: string) => void;
  level?: "h3" | "h4";
  motif?: ReactNode;
  iconFor?: (entry: T) => string;
  nameFor?: (entry: T) => string | undefined;
}

export default function EditableEntryTable<T>({
  title,
  entries,
  columns,
  setter,
  onError,
  level = "h4",
  motif,
  iconFor,
  nameFor,
}: Props<T>) {
  return (
    <div>
      <SectionHeading level={level} motif={motif} title={`${title} (${entries.length})`} />
      {Array.from(new Map(columns.filter((c) => c.datalist).map((c) => [c.datalist!.id, c.datalist!])).values()).map(
        (dl) => (
          <datalist key={dl.id} id={dl.id}>
            {dl.options.map((o) => (
              <option key={o.value} value={o.value}>
                {o.label}
              </option>
            ))}
          </datalist>
        ),
      )}
      <table className={styles.table}>
        <thead>
          <tr>
            {iconFor && <th></th>}
            <th>#</th>
            {nameFor && <th>Name</th>}
            {columns.map((c) => (
              <th key={String(c.key)}>{c.label}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {entries.map((entry, i) => (
            <tr key={i}>
              {iconFor && (
                <td>
                  <img
                    className={styles.icon}
                    src={iconFor(entry)}
                    alt=""
                    onError={(e) => {
                      e.currentTarget.onerror = null;
                      e.currentTarget.src = totkUnknownIconUrl;
                    }}
                  />
                </td>
              )}
              <td>{i}</td>
              {nameFor && <td>{nameFor(entry) ?? ""}</td>}
              {columns.map((c) => (
                <Cell key={String(c.key)} entries={entries} index={i} column={c} setter={setter} onError={onError} />
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
