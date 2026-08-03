import { useEffect, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { OpenResult } from "./api";
import * as api from "./api";
import {
  getRecentFiles,
  recordRecentFile,
  removeRecentFile,
  type RecentFile,
} from "./api/recentFiles";
import BotwView from "./components/botw/BotwView";
import TotkView from "./components/totk/TotkView";
import CaptionView from "./components/totk/CaptionView";
import ErrorBanner from "./components/ErrorBanner";
import CloseConfirmModal from "./components/CloseConfirmModal";
import Toast from "./components/Toast";
import { useThemeAttributes } from "./theme/useThemeAttributes";
import BotwMotif from "./theme/motifs/BotwMotif";
import TotkMotif from "./theme/motifs/TotkMotif";
import SectionWave from "./theme/SectionWave";
import styles from "./App.module.css";

const GAME_SUFFIX: Record<OpenResult["kind"], string> = {
  botw: " (BOTW)",
  totk: " (TOTK)",
  caption: " (TOTK)",
};

function App() {
  const [loaded, setLoaded] = useState<OpenResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [savedToast, setSavedToast] = useState(false);
  const [confirmClose, setConfirmClose] = useState(false);
  const [recents, setRecents] = useState<RecentFile[]>(getRecentFiles);
  const dirty = api.useDirty();
  const editable = loaded?.kind === "botw" || loaded?.kind === "totk";
  useThemeAttributes(loaded?.kind === "caption" ? "totk" : loaded?.kind ?? "neutral");

  useEffect(() => {
    const suffix = loaded ? GAME_SUFFIX[loaded.kind] : "";
    getCurrentWindow().setTitle(`Zelda Save Editor${suffix}${dirty ? " *" : ""}`);
  }, [loaded, dirty]);

  useEffect(() => {
    const win = getCurrentWindow();
    const unlisten = win.onCloseRequested((event) => {
      if (api.isDirty()) {
        event.preventDefault();
        setConfirmClose(true);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const recordIfEditable = async (result: OpenResult) => {
    if (result.kind !== "botw" && result.kind !== "totk") return;
    const path = await api.currentPath();
    if (!path) return;
    const name = path.split(/[\\/]/).pop() ?? path;
    setRecents(recordRecentFile({ path, kind: result.kind, name }));
  };

  const handleOpen = async () => {
    try {
      const result = await api.openSave();
      setLoaded(result);
      setError(null);
      await recordIfEditable(result);
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleOpenRecent = async (file: RecentFile) => {
    try {
      const result = await api.openSaveAt(file.path);
      setLoaded(result);
      setError(null);
      await recordIfEditable(result);
    } catch (err) {
      // The file may have been moved or deleted; drop it from the list.
      setError((err as api.ShellError).message ?? String(err));
      setRecents(removeRecentFile(file.path));
    }
  };

  const handleSave = async () => {
    try {
      await api.saveSave();
      setSavedToast(true);
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleSaveAs = async () => {
    try {
      await api.saveAs();
      setSavedToast(true);
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const refreshCurrent = async () => {
    if (!loaded || loaded.kind === "caption") return;
    try {
      if (loaded.kind === "botw") {
        setLoaded({ kind: "botw", state: await api.getBotwState() });
      } else {
        setLoaded({ kind: "totk", state: await api.getTotkState() });
      }
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const refreshCurrentRef = useRef(refreshCurrent);
  refreshCurrentRef.current = refreshCurrent;

  // Every mutating command (any edit, anywhere in the app) fires this after it succeeds, so the
  // UI always reflects the true backend state. Without it, a controlled input like a <select>
  // has nothing but the stale prop to render, so it visually snaps back right after the user
  // picks a new value.
  useEffect(() => {
    return api.onStateChanged(() => refreshCurrentRef.current());
  }, []);

  const closeForReal = () => {
    getCurrentWindow().destroy();
  };

  return (
    <div className={styles.app}>
      <h1 className={styles.title}>
        Zelda Save Editor
        {loaded && <span className={styles.gameSuffix}>{GAME_SUFFIX[loaded.kind]}</span>}
      </h1>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />
      <div className={styles.toolbar}>
        <button className={styles.button} onClick={handleOpen}>
          Open...
        </button>
        <button className={styles.button} onClick={handleSave} disabled={!editable}>
          Save
        </button>
        <button className={styles.button} onClick={handleSaveAs} disabled={!editable}>
          Save As...
        </button>
        <button className={styles.button} onClick={refreshCurrent} disabled={!editable}>
          Refresh
        </button>
      </div>
      {loaded?.kind === "botw" && (
        <BotwView state={loaded.state} onError={setError} onRefresh={refreshCurrent} />
      )}
      {loaded?.kind === "totk" && (
        <TotkView state={loaded.state} onError={setError} onRefresh={refreshCurrent} />
      )}
      {loaded?.kind === "caption" && <CaptionView info={loaded.info} />}
      {!loaded && (
        <div className={styles.hero}>
          <div className={styles.heroMotifs}>
            <BotwMotif />
            <TotkMotif />
          </div>
          <p className={styles.heroTagline}>Breath of the Wild &amp; Tears of the Kingdom save editor</p>
          <div className={styles.heroWave}>
            <SectionWave />
          </div>
          <button className={styles.heroButton} onClick={handleOpen}>
            Open a save file...
          </button>
          <p className={styles.heroHint}>game_data.sav, progress.sav, or caption.sav</p>
          {recents.length > 0 && (
            <div className={styles.recents}>
              <p className={styles.recentsTitle}>Recent</p>
              {recents.map((file) => (
                <button
                  key={file.path}
                  className={styles.recentItem}
                  onClick={() => handleOpenRecent(file)}
                  title={file.path}
                >
                  <span className={styles.recentName}>{file.name}</span>
                  <span className={styles.recentKind}>{file.kind.toUpperCase()}</span>
                  <span className={styles.recentPath}>{file.path}</span>
                </button>
              ))}
            </div>
          )}
        </div>
      )}
      {savedToast && <Toast message="Saved" onDismiss={() => setSavedToast(false)} />}
      {confirmClose && (
        <CloseConfirmModal
          onSave={async () => {
            try {
              await api.saveSave();
              closeForReal();
            } catch (err) {
              setError((err as api.ShellError).message ?? String(err));
              setConfirmClose(false);
            }
          }}
          onDiscard={closeForReal}
          onCancel={() => setConfirmClose(false)}
        />
      )}
    </div>
  );
}

export default App;
