import { useState } from "react";
import type { TotkState } from "../../api";
import * as api from "../../api";
import TotkPouchTables from "./TotkPouchTables";
import TotkHorsesTable from "./TotkHorsesTable";
import TotkAutoBuildTable from "./TotkAutoBuildTable";
import TotkMapPinsTable from "./TotkMapPinsTable";
import TotkMapMarkersTable from "./TotkMapMarkersTable";
import TotkTeleportersTable from "./TotkTeleportersTable";
import TotkCompletionismPanel from "./TotkCompletionismPanel";
import SectionHeading from "../../theme/SectionHeading";
import TotkMotif from "../../theme/motifs/TotkMotif";
import styles from "./TotkView.module.css";

interface Props {
  state: TotkState;
  onError: (message: string) => void;
  onRefresh: () => void;
}

function NumberField({
  label,
  value,
  onCommit,
  onError,
}: {
  label: string;
  value: number;
  onCommit: (val: number) => Promise<void>;
  onError: (message: string) => void;
}) {
  const [text, setText] = useState(String(value));
  return (
    <label className={styles.field}>
      {label}:{" "}
      <input
        className={styles.input}
        type="number"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onBlur={() => onCommit(Number(text)).catch((err) => onError(String(err)))}
      />
    </label>
  );
}

export default function TotkView({ state, onError, onRefresh }: Props) {
  const [posX, setPosX] = useState(String(state.save_pos[0]));
  const [posY, setPosY] = useState(String(state.save_pos[1]));
  const [posZ, setPosZ] = useState(String(state.save_pos[2]));
  const commitPos = () =>
    api
      .setSavePos(Number(posX), Number(posY), Number(posZ))
      .catch((err) => onError(String(err)));

  return (
    <div className={styles.view}>
      <SectionHeading title="Stats" motif={<TotkMotif />} />
      <NumberField label="Max Life" value={state.max_life} onCommit={api.setMaxLife} onError={onError} />
      <NumberField
        label="Current Rupees"
        value={state.current_rupees}
        onCommit={api.setCurrentRupees}
        onError={onError}
      />
      <NumberField
        label="Max Stamina"
        value={state.max_stamina}
        onCommit={api.setMaxStaminaTotk}
        onError={onError}
      />
      <NumberField
        label="Max Energy"
        value={state.max_energy}
        onCommit={api.setMaxEnergy}
        onError={onError}
      />
      <NumberField label="Playtime" value={state.playtime} onCommit={api.setPlaytime} onError={onError} />
      <NumberField
        label="Horse Inn Member Point"
        value={state.horse_inn_member_point}
        onCommit={api.setHorseInnMemberPoint}
        onError={onError}
      />

      <SectionHeading level="h4" title="Save Position" />
      <div className={styles.row}>
        <input className={styles.input} type="number" value={posX} onChange={(e) => setPosX(e.target.value)} onBlur={commitPos} />{" "}
        <input className={styles.input} type="number" value={posY} onChange={(e) => setPosY(e.target.value)} onBlur={commitPos} />{" "}
        <input className={styles.input} type="number" value={posZ} onChange={(e) => setPosZ(e.target.value)} onBlur={commitPos} />
      </div>

      <SectionHeading level="h4" title="Current Checkpoint" />
      <input
        className={styles.input}
        type="text"
        defaultValue={state.sequence_current_banc}
        onBlur={(e) => api.setSequenceCurrentBanc(e.target.value).catch((err) => onError(String(err)))}
      />

      <SectionHeading level="h4" title="Pouch Slot Counts" />
      <NumberField
        label="Weapon"
        value={state.pouch_weapon_valid_num}
        onCommit={api.setPouchWeaponValidNum}
        onError={onError}
      />
      <NumberField
        label="Bow"
        value={state.pouch_bow_valid_num}
        onCommit={api.setPouchBowValidNum}
        onError={onError}
      />
      <NumberField
        label="Shield"
        value={state.pouch_shield_valid_num}
        onCommit={api.setPouchShieldValidNum}
        onError={onError}
      />

      <TotkPouchTables
        weapons={state.pouch_weapons}
        bows={state.pouch_bows}
        shields={state.pouch_shields}
        armor={state.armor}
        arrows={state.arrows}
        materials={state.materials}
        keyItems={state.key_items}
        devices={state.devices}
        food={state.food}
        onError={onError}
      />

      <TotkHorsesTable horses={state.horses} onError={onError} />

      <TotkAutoBuildTable autobuilds={state.autobuilds} onError={onError} />

      <TotkMapPinsTable mapPins={state.map_pins} onError={onError} />

      <TotkMapMarkersTable mapMarkers={state.map_markers} onError={onError} />

      <TotkTeleportersTable teleporters={state.teleporters} onError={onError} />

      <TotkCompletionismPanel state={state} onError={onError} onUnlocked={onRefresh} />
    </div>
  );
}
