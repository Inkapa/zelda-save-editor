import type { BotwState } from "../../api";
import BotwStatsForm from "./BotwStatsForm";
import BotwItemsTable from "./BotwItemsTable";
import BotwModifiersTable from "./BotwModifiersTable";
import BotwHorsesTable from "./BotwHorsesTable";
import BotwCompletionismPanel from "./BotwCompletionismPanel";
import styles from "./BotwView.module.css";

interface Props {
  state: BotwState;
  onError: (message: string) => void;
  onRefresh: () => void;
}

export default function BotwView({ state, onError, onRefresh }: Props) {
  return (
    <div className={styles.view}>
      <BotwStatsForm state={state} onError={onError} />
      <BotwItemsTable items={state.items} onError={onError} />
      <BotwModifiersTable
        weaponModifiers={state.weapon_modifiers}
        bowModifiers={state.bow_modifiers}
        shieldModifiers={state.shield_modifiers}
        onError={onError}
      />
      <BotwHorsesTable horses={state.horses} onError={onError} />
      <BotwCompletionismPanel onError={onError} onUnlocked={onRefresh} />
    </div>
  );
}
