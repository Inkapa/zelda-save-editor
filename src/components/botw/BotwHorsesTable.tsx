import { useState } from "react";
import type { BotwHorse } from "../../api";
import * as api from "../../api";

interface Props {
  horses: BotwHorse[];
  onError: (message: string) => void;
}

function HorseRow({
  index,
  horse,
  onError,
}: {
  index: number;
  horse: BotwHorse;
  onError: (message: string) => void;
}) {
  const [name, setName] = useState(horse.name ?? "");
  const [saddle, setSaddle] = useState(horse.saddle ?? "");
  const [reins, setReins] = useState(horse.reins ?? "");
  const [horseType, setHorseType] = useState(horse.horse_type);
  const hasNamedSlots = horse.name !== null;

  return (
    <tr>
      <td>{index}</td>
      <td>
        <input
          value={horseType}
          onChange={(e) => setHorseType(e.target.value)}
          onBlur={() => api.setHorseType(index, horseType).catch((err) => onError(String(err)))}
        />
      </td>
      <td>
        {hasNamedSlots && (
          <input
            value={name}
            onChange={(e) => setName(e.target.value)}
            onBlur={() => api.setHorseName(index, name).catch((err) => onError(String(err)))}
          />
        )}
      </td>
      <td>
        {hasNamedSlots && (
          <input
            value={saddle}
            onChange={(e) => setSaddle(e.target.value)}
            onBlur={() => api.setHorseSaddle(index, saddle).catch((err) => onError(String(err)))}
          />
        )}
      </td>
      <td>
        {hasNamedSlots && (
          <input
            value={reins}
            onChange={(e) => setReins(e.target.value)}
            onBlur={() => api.setHorseReins(index, reins).catch((err) => onError(String(err)))}
          />
        )}
      </td>
    </tr>
  );
}

export default function BotwHorsesTable({ horses, onError }: Props) {
  return (
    <div>
      <h3>Horses</h3>
      <table>
        <thead>
          <tr>
            <th>#</th>
            <th>Type</th>
            <th>Name</th>
            <th>Saddle</th>
            <th>Reins</th>
          </tr>
        </thead>
        <tbody>
          {horses.map((horse, i) => (
            <HorseRow key={i} index={i} horse={horse} onError={onError} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
