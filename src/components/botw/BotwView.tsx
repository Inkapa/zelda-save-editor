import type { BotwState } from "../../api";
import BotwStatsForm from "./BotwStatsForm";
import { BotwCategoryTable } from "./BotwItemsTable";
import BotwHorsesTable from "./BotwHorsesTable";
import BotwCompletionismPanel from "./BotwCompletionismPanel";
import TabShell from "../../theme/TabShell";
import { HeartIcon, SwordIcon, BowIcon, ShieldIcon, ShirtIcon, LeafIcon, AppleIcon, ChestIcon, HorseshoeIcon, StarIcon } from "../../theme/icons";
import styles from "./BotwView.module.css";

interface Props {
  state: BotwState;
  onError: (message: string) => void;
  onRefresh: () => void;
}

export default function BotwView({ state, onError, onRefresh }: Props) {
  return (
    <div className={styles.view}>
      <TabShell
        tabs={[
          {
            id: "stats",
            label: "Stats",
            icon: <HeartIcon />,
            content: <BotwStatsForm state={state} onError={onError} />,
          },
          {
            id: "weapons",
            label: "Weapons",
            icon: <SwordIcon />,
            content: (
              <BotwCategoryTable
                title="Weapons"
                valueLabel="Durability"
                items={state.items}
                category="weapon"
                modifiers={state.weapon_modifiers}
                onError={onError}
              />
            ),
          },
          {
            id: "bows",
            label: "Bows",
            icon: <BowIcon />,
            content: (
              <BotwCategoryTable
                title="Bows"
                valueLabel="Durability"
                items={state.items}
                category="bow"
                modifiers={state.bow_modifiers}
                onError={onError}
              />
            ),
          },
          {
            id: "shields",
            label: "Shields",
            icon: <ShieldIcon />,
            content: (
              <BotwCategoryTable
                title="Shields"
                valueLabel="Durability"
                items={state.items}
                category="shield"
                modifiers={state.shield_modifiers}
                onError={onError}
              />
            ),
          },
          {
            id: "armor",
            label: "Armor",
            icon: <ShirtIcon />,
            content: (
              <BotwCategoryTable
                title="Armor"
                valueLabel="Dye Color"
                items={state.items}
                category="armor"
                onError={onError}
              />
            ),
          },
          {
            id: "materials",
            label: "Materials",
            icon: <LeafIcon />,
            content: (
              <BotwCategoryTable
                title="Materials"
                valueLabel="Quantity"
                items={state.items}
                category="material"
                onError={onError}
              />
            ),
          },
          {
            id: "food",
            label: "Food",
            icon: <AppleIcon />,
            content: (
              <BotwCategoryTable title="Food" valueLabel="Quantity" items={state.items} category="food" onError={onError} />
            ),
          },
          {
            id: "key_items",
            label: "Key Items",
            icon: <ChestIcon />,
            content: (
              <BotwCategoryTable
                title="Key Items"
                valueLabel="Quantity"
                items={state.items}
                category="key_item"
                onError={onError}
              />
            ),
          },
          {
            id: "horses",
            label: "Horses",
            icon: <HorseshoeIcon />,
            content: <BotwHorsesTable horses={state.horses} onError={onError} />,
          },
          {
            id: "completionism",
            label: "Completionism",
            icon: <StarIcon />,
            content: <BotwCompletionismPanel onError={onError} onUnlocked={onRefresh} />,
          },
        ]}
      />
    </div>
  );
}
