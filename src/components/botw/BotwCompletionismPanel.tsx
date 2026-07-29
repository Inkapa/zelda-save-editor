import { useState } from "react";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import styles from "./BotwCompletionismPanel.module.css";

interface Props {
  onError: (message: string) => void;
  onUnlocked: () => void;
}

function UnlockButton({
  label,
  run,
  onError,
  onUnlocked,
}: {
  label: string;
  run: () => Promise<number>;
  onError: (message: string) => void;
  onUnlocked: () => void;
}) {
  const [result, setResult] = useState<number | null>(null);
  const handleClick = async () => {
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

export default function BotwCompletionismPanel({ onError, onUnlocked }: Props) {
  return (
    <div>
      <SectionHeading title="Completionism" motif={<BotwMotif />} />
      <UnlockButton label="Unlock All Koroks" run={api.unlockAllKoroks} onError={onError} onUnlocked={onUnlocked} />
      <UnlockButton
        label="Unlock All Defeated Hinox"
        run={api.unlockAllDefeatedHinox}
        onError={onError}
        onUnlocked={onUnlocked}
      />
      <UnlockButton
        label="Unlock All Defeated Talus"
        run={api.unlockAllDefeatedTalus}
        onError={onError}
        onUnlocked={onUnlocked}
      />
      <UnlockButton
        label="Unlock All Defeated Molduga"
        run={api.unlockAllDefeatedMolduga}
        onError={onError}
        onUnlocked={onUnlocked}
      />
      <UnlockButton
        label="Unlock All Locations"
        run={api.unlockAllLocations}
        onError={onError}
        onUnlocked={onUnlocked}
      />
    </div>
  );
}
