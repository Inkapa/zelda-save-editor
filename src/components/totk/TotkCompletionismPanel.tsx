import type { TotkState } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import TotkMotif from "../../theme/motifs/TotkMotif";
import UnlockButton from "../UnlockButton";

interface Props {
  state: TotkState;
  onError: (message: string) => void;
  onUnlocked: () => void;
}

const GUID_UNLOCK_CONFIRM =
  "This adds entries to your save's internal tracking data and grows the file slightly, a less common kind of edit than others in this tool. Continue?";

export default function TotkCompletionismPanel({ state, onError, onUnlocked }: Props) {
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
      <p>Bubbuls defeated: {state.defeated_bubbuls}</p>
      <p>Sage wills found: {state.sage_wills_found}</p>
      <p>Old maps found: {state.old_maps_found}</p>
      <p>Addison completed: {state.addison_completed}</p>
      <UnlockButton
        label="Unlock All Bubbuls"
        run={api.unlockAllBubbuls}
        onError={onError}
        onUnlocked={onUnlocked}
        confirmMessage={GUID_UNLOCK_CONFIRM}
      />
      <UnlockButton
        label="Unlock All Sage Wills"
        run={api.unlockAllSageWills}
        onError={onError}
        onUnlocked={onUnlocked}
        confirmMessage={GUID_UNLOCK_CONFIRM}
      />
      <UnlockButton
        label="Unlock All Addison"
        run={api.unlockAllAddison}
        onError={onError}
        onUnlocked={onUnlocked}
        confirmMessage={GUID_UNLOCK_CONFIRM}
      />
    </div>
  );
}
