import type { BotwState } from "../../api";
import BotwStatsForm from "./BotwStatsForm";
import { BotwCategoryTable } from "./BotwItemsTable";
import BotwHorsesTable from "./BotwHorsesTable";
import BotwCompletionismPanel from "./BotwCompletionismPanel";
import TabShell from "../../theme/TabShell";
import { LeafIcon, ChestIcon } from "../../theme/icons";
import {
  HomeSprite,
  SwordSprite,
  BowSprite,
  ShieldSprite,
  ShirtSprite,
  AppleSprite,
  StarSprite,
  HorseSprite,
} from "../../theme/TabSprite";
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
            icon: <HomeSprite />,
            content: <BotwStatsForm state={state} onError={onError} />,
          },
          {
            id: "weapons",
            label: "Weapons",
            icon: <SwordSprite />,
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
            icon: <BowSprite />,
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
            icon: <ShieldSprite />,
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
            icon: <ShirtSprite />,
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
            icon: <AppleSprite />,
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
            icon: <HorseSprite />,
            content: <BotwHorsesTable horses={state.horses} onError={onError} />,
          },
          {
            id: "completionism",
            label: "Completionism",
            icon: <StarSprite />,
            content: <BotwCompletionismPanel onError={onError} onUnlocked={onRefresh} />,
          },
        ]}
      />
    </div>
  );
}
