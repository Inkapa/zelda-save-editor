import type { BotwState } from "../../api";
import BotwStatsForm from "./BotwStatsForm";
import BotwItemsTable from "./BotwItemsTable";
import BotwModifiersTable from "./BotwModifiersTable";
import BotwHorsesTable from "./BotwHorsesTable";

interface Props {
  state: BotwState;
  onError: (message: string) => void;
}

export default function BotwView({ state, onError }: Props) {
  return (
    <div>
      <BotwStatsForm state={state} onError={onError} />
      <BotwItemsTable items={state.items} onError={onError} />
      <BotwModifiersTable
        weaponModifiers={state.weapon_modifiers}
        bowModifiers={state.bow_modifiers}
        shieldModifiers={state.shield_modifiers}
        onError={onError}
      />
      <BotwHorsesTable horses={state.horses} onError={onError} />
    </div>
  );
}
