import { useState } from "react";
import type { BotwState } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import styles from "./BotwStatsForm.module.css";

interface Props {
  state: BotwState;
  onError: (message: string) => void;
}

function NumberField({
  label,
  value,
  onCommit,
}: {
  label: string;
  value: number;
  onCommit: (val: number) => Promise<void>;
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
        onBlur={() => {
          onCommit(Number(text));
        }}
      />
    </label>
  );
}

export default function BotwStatsForm({ state, onError }: Props) {
  const commit = (fn: (val: number) => Promise<void>) => async (val: number) => {
    try {
      await fn(val);
    } catch (err) {
      onError((err as api.ShellError).message ?? String(err));
    }
  };

  return (
    <div>
      <SectionHeading title="Stats" motif={<BotwMotif />} />
      <NumberField label="Rupees" value={state.rupees} onCommit={commit(api.setRupees)} />
      <NumberField label="Mons (Amiibo)" value={state.mons} onCommit={commit(api.setMons)} />
      <NumberField label="Max Hearts" value={state.max_hearts} onCommit={commit(api.setMaxHearts)} />
      <NumberField
        label="Max Stamina"
        value={state.max_stamina}
        onCommit={commit(api.setMaxStaminaBotw)}
      />
      <NumberField
        label="Relic - Gerudo"
        value={state.relic_gerudo}
        onCommit={commit(api.setRelicGerudo)}
      />
      <NumberField
        label="Relic - Goron"
        value={state.relic_goron}
        onCommit={commit(api.setRelicGoron)}
      />
      <NumberField label="Relic - Rito" value={state.relic_rito} onCommit={commit(api.setRelicRito)} />
      <NumberField
        label="Korok Seed Counter"
        value={state.korok_seed_counter}
        onCommit={commit(api.setKorokSeedCounter)}
      />
      <NumberField
        label="Defeated Hinox Counter"
        value={state.defeated_hinox_counter}
        onCommit={commit(api.setDefeatedHinoxCounter)}
      />
      <NumberField
        label="Defeated Talus Counter"
        value={state.defeated_talus_counter}
        onCommit={commit(api.setDefeatedTalusCounter)}
      />
      <NumberField
        label="Defeated Molduga Counter"
        value={state.defeated_molduga_counter}
        onCommit={commit(api.setDefeatedMoldugaCounter)}
      />
      <NumberField
        label="Playtime (seconds)"
        value={state.playtime_seconds}
        onCommit={commit(api.setPlaytimeSeconds)}
      />
      {state.motorcycle !== null && (
        <label className={styles.field}>
          Master Cycle Zero unlocked:{" "}
          <input
            type="checkbox"
            checked={state.motorcycle}
            onChange={(e) =>
              api.setMotorcycle(e.target.checked).catch((err) => onError(String(err)))
            }
          />
        </label>
      )}
      <SectionHeading level="h4" title="Map" />
      <label className={styles.field}>
        Map:{" "}
        <input
          className={styles.input}
          type="text"
          defaultValue={state.map}
          onBlur={(e) => api.setMap(e.target.value).catch((err) => onError(String(err)))}
        />
      </label>
      <label className={styles.field}>
        Map Type:{" "}
        <input
          className={styles.input}
          type="text"
          defaultValue={state.map_type}
          onBlur={(e) => api.setMapType(e.target.value).catch((err) => onError(String(err)))}
        />
      </label>
      <SectionHeading level="h4" title="Positions" />
      <PositionField
        label="Player Position"
        value={state.player_position}
        onCommit={commit3(api.setPlayerPosition, onError)}
      />
      <PositionField
        label="Horse Position"
        value={state.horse_position}
        onCommit={commit3(api.setHorsePosition, onError)}
      />
    </div>
  );
}

function commit3(
  fn: (x: number, y: number, z: number) => Promise<void>,
  onError: (message: string) => void,
) {
  return async (x: number, y: number, z: number) => {
    try {
      await fn(x, y, z);
    } catch (err) {
      onError(String(err));
    }
  };
}

function PositionField({
  label,
  value,
  onCommit,
}: {
  label: string;
  value: [number, number, number];
  onCommit: (x: number, y: number, z: number) => Promise<void>;
}) {
  const [xs, setXs] = useState(String(value[0]));
  const [ys, setYs] = useState(String(value[1]));
  const [zs, setZs] = useState(String(value[2]));
  const commit = () => onCommit(Number(xs), Number(ys), Number(zs));
  return (
    <div className={styles.field}>
      {label}:{" "}
      <input className={styles.input} type="number" value={xs} onChange={(e) => setXs(e.target.value)} onBlur={commit} />{" "}
      <input className={styles.input} type="number" value={ys} onChange={(e) => setYs(e.target.value)} onBlur={commit} />{" "}
      <input className={styles.input} type="number" value={zs} onChange={(e) => setZs(e.target.value)} onBlur={commit} />
    </div>
  );
}
