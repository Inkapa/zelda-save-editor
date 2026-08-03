import { useState, type SyntheticEvent } from "react";
import { totkUnknownIconUrl } from "./totkIcons";
import styles from "./ItemIcon.module.css";

function fallback(e: SyntheticEvent<HTMLImageElement>) {
  e.currentTarget.onerror = null;
  e.currentTarget.src = totkUnknownIconUrl;
}

/** Item thumbnail that pops a larger preview near the cursor while hovered, so a tiny 48px
 * sprite can be inspected without opening anything. */
export default function ItemIcon({ url, size = 48 }: { url: string; size?: number }) {
  const [at, setAt] = useState<{ x: number; y: number } | null>(null);
  return (
    <>
      <img
        className={styles.icon}
        style={{ width: size, height: size }}
        src={url}
        alt=""
        onError={fallback}
        onMouseEnter={(e) => setAt({ x: e.clientX, y: e.clientY })}
        onMouseMove={(e) => setAt({ x: e.clientX, y: e.clientY })}
        onMouseLeave={() => setAt(null)}
      />
      {at && (
        <div className={styles.preview} style={{ left: at.x + 18, top: at.y + 18 }}>
          <img className={styles.previewImg} src={url} alt="" onError={fallback} />
        </div>
      )}
    </>
  );
}
