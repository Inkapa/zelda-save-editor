import { useState, type ReactNode } from "react";
import SectionHeading from "../../theme/SectionHeading";
import styles from "./EditableEntryTable.module.css";

export interface ColumnDef<T> {
  key: keyof T;
  label: string;
  type?: "text" | "number" | "checkbox" | "readonly";
  format?: (value: T[keyof T]) => string;
  parse?: (raw: string) => T[keyof T];
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

function TextCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const format = column.format ?? defaultFormat;
  const [text, setText] = useState(format(entries[index][column.key]));
  const commit = () => {
    const parse = column.parse ?? ((raw: string) => raw as unknown as T[keyof T]);
    setter(replaceAt(entries, index, column.key, parse(text))).catch((err) => onError(String(err)));
  };
  return (
    <td>
      <input
        className={styles.input}
        type={column.type === "number" ? "number" : "text"}
        value={text}
        onChange={(e) => setText(e.target.value)}
        onBlur={commit}
      />
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
}

export default function EditableEntryTable<T>({
  title,
  entries,
  columns,
  setter,
  onError,
  level = "h4",
  motif,
}: Props<T>) {
  return (
    <div>
      <SectionHeading level={level} motif={motif} title={`${title} (${entries.length})`} />
      <table className={styles.table}>
        <thead>
          <tr>
            <th>#</th>
            {columns.map((c) => (
              <th key={String(c.key)}>{c.label}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {entries.map((_, i) => (
            <tr key={i}>
              <td>{i}</td>
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
