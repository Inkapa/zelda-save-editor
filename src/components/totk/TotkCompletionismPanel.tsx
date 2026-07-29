import type { TotkState } from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import TotkMotif from "../../theme/motifs/TotkMotif";

interface Props {
  state: TotkState;
}

export default function TotkCompletionismPanel({ state }: Props) {
  return (
    <div>
      <SectionHeading title="Completionism" motif={<TotkMotif />} />
      <p>Shrines found: {state.shrines_found}</p>
      <p>Shrines cleared: {state.shrines_cleared}</p>
      <p>Koroks hidden: {state.koroks_hidden}</p>
      <p>Koroks carried: {state.koroks_carried}</p>
      <p>Locations visited: {state.locations_visited}</p>
      <p>Hinox defeated: {state.defeated_hinox}</p>
      <p>Talus defeated: {state.defeated_talus}</p>
      <p>Molduga defeated: {state.defeated_molduga}</p>
    </div>
  );
}
