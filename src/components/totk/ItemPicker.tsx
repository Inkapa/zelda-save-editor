import { useEffect, useMemo, useState, type SyntheticEvent } from "react";
import { totkUnknownIconUrl } from "./totkIcons";
import styles from "./ItemPicker.module.css";

interface Props {
  value: string;
  items: string[];
  iconFor: (id: string) => string;
  nameFor: (id: string) => string | undefined;
  onCommit: (id: string) => void;
}

function fallback(e: SyntheticEvent<HTMLImageElement>) {
  e.currentTarget.onerror = null;
  e.currentTarget.src = totkUnknownIconUrl;
}

/** Category-restricted item chooser: a trigger showing the current item's icon + name, and a
 * searchable dropdown of the same category's items (icon + name), matching marcrobledo's own
 * filterable item picker. A native <select> can't render per-option images, hence the custom
 * combobox. */
export default function ItemPicker({ value, items, iconFor, nameFor, onCommit }: Props) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);

  const label = (id: string) => {
    const name = nameFor(id);
    return name ? `${name} (${id})` : id;
  };

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return items;
    return items.filter((id) => label(id).toLowerCase().includes(q));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [query, items]);

  useEffect(() => {
    setActive((a) => Math.min(a, Math.max(0, filtered.length - 1)));
  }, [filtered.length]);

  const openDropdown = () => {
    setQuery("");
    const idx = items.indexOf(value);
    setActive(idx >= 0 ? idx : 0);
    setOpen(true);
  };
  const commit = (id: string) => {
    if (id !== value) onCommit(id);
    setOpen(false);
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setActive((a) => Math.min(a + 1, filtered.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setActive((a) => Math.max(a - 1, 0));
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (filtered[active]) commit(filtered[active]);
    } else if (e.key === "Escape") {
      e.preventDefault();
      setOpen(false);
    }
  };

  return (
    <div
      className={styles.picker}
      onBlur={(e) => {
        if (!e.currentTarget.contains(e.relatedTarget as Node)) setOpen(false);
      }}
    >
      <button
        type="button"
        className={styles.trigger}
        onClick={() => (open ? setOpen(false) : openDropdown())}
      >
        <img className={styles.triggerIcon} src={iconFor(value)} alt="" onError={fallback} />
        <span className={styles.triggerLabel}>{label(value)}</span>
        <span className={styles.caret}>▾</span>
      </button>
      {open && (
        <div className={styles.panel}>
          <input
            autoFocus
            className={styles.search}
            placeholder="Search..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={onKeyDown}
          />
          <div className={styles.results}>
            {filtered.map((id, i) => (
              <div
                key={id}
                className={i === active ? styles.optionActive : styles.option}
                onMouseDown={(e) => {
                  e.preventDefault();
                  commit(id);
                }}
                onMouseEnter={() => setActive(i)}
                ref={
                  i === active ? (el) => el?.scrollIntoView({ block: "nearest" }) : undefined
                }
              >
                <img
                  className={styles.optionIcon}
                  src={iconFor(id)}
                  loading="lazy"
                  alt=""
                  onError={fallback}
                />
                <span className={styles.optionLabel}>{label(id)}</span>
              </div>
            ))}
            {filtered.length === 0 && <div className={styles.empty}>No matches</div>}
          </div>
        </div>
      )}
    </div>
  );
}
