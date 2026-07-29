import { useState } from "react";
import type { TotkState } from "../../api";
import * as api from "../../api";

interface Props {
  state: TotkState;
  onError: (message: string) => void;
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
    <label style={{ display: "block", marginBottom: 4 }}>
      {label}:{" "}
      <input
        type="number"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onBlur={() => onCommit(Number(text)).catch((err) => onError(String(err)))}
      />
    </label>
  );
}

export default function TotkView({ state, onError }: Props) {
  const [posX, setPosX] = useState(String(state.save_pos[0]));
  const [posY, setPosY] = useState(String(state.save_pos[1]));
  const [posZ, setPosZ] = useState(String(state.save_pos[2]));
  const commitPos = () =>
    api
      .setSavePos(Number(posX), Number(posY), Number(posZ))
      .catch((err) => onError(String(err)));

  return (
    <div>
      <h3>Stats</h3>
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

      <h3>Save Position</h3>
      <div>
        <input type="number" value={posX} onChange={(e) => setPosX(e.target.value)} onBlur={commitPos} />{" "}
        <input type="number" value={posY} onChange={(e) => setPosY(e.target.value)} onBlur={commitPos} />{" "}
        <input type="number" value={posZ} onChange={(e) => setPosZ(e.target.value)} onBlur={commitPos} />
      </div>

      <h3>Current Checkpoint</h3>
      <input
        type="text"
        defaultValue={state.sequence_current_banc}
        onBlur={(e) => api.setSequenceCurrentBanc(e.target.value).catch((err) => onError(String(err)))}
      />

      <h3>Pouch Slot Counts</h3>
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
    </div>
  );
}
