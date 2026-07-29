import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import UnlockButton from "../UnlockButton";

interface Props {
  onError: (message: string) => void;
  onUnlocked: () => void;
}

export default function BotwCompletionismPanel({ onError, onUnlocked }: Props) {
  return (
    <div>
      <SectionHeading title="Completionism" motif={<BotwMotif />} />
      <UnlockButton label="Unlock All Koroks" run={api.unlockAllKoroks} onError={onError} onUnlocked={onUnlocked} />
      <UnlockButton
        label="Unlock All Defeated Hinox"
        run={api.unlockAllDefeatedHinox}
        onError={onError}
        onUnlocked={onUnlocked}
      />
      <UnlockButton
        label="Unlock All Defeated Talus"
        run={api.unlockAllDefeatedTalus}
        onError={onError}
        onUnlocked={onUnlocked}
      />
      <UnlockButton
        label="Unlock All Defeated Molduga"
        run={api.unlockAllDefeatedMolduga}
        onError={onError}
        onUnlocked={onUnlocked}
      />
      <UnlockButton
        label="Unlock All Locations"
        run={api.unlockAllLocations}
        onError={onError}
        onUnlocked={onUnlocked}
      />
    </div>
  );
}
