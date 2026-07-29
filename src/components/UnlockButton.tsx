import { useState } from "react";
import * as api from "../api";
import styles from "./UnlockButton.module.css";

interface Props {
  label: string;
  run: () => Promise<number>;
  onError: (message: string) => void;
  onUnlocked: () => void;
  /** When set, clicking the button first shows a native confirm dialog with this message.
   * Declining does nothing (no command call, no error). Used for edits that are riskier or
   * less common than a typical field edit, e.g. TOTK's GUID mass-unlocks, which grow the
   * save file rather than overwriting an already-reserved slot. */
  confirmMessage?: string;
}

export default function UnlockButton({ label, run, onError, onUnlocked, confirmMessage }: Props) {
  const [result, setResult] = useState<number | null>(null);
  const handleClick = async () => {
    if (confirmMessage && !window.confirm(confirmMessage)) {
      return;
    }
    try {
      const count = await run();
      setResult(count);
      onUnlocked();
    } catch (err) {
      onError((err as api.ShellError).message ?? String(err));
    }
  };
  return (
    <div className={styles.row}>
      <button className={styles.button} onClick={handleClick}>
        {label}
      </button>{" "}
      {result !== null && <span>unlocked {result} new</span>}
    </div>
  );
}
