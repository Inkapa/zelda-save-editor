import { useState } from "react";
import type { OpenResult } from "./api";
import * as api from "./api";
import BotwView from "./components/botw/BotwView";
import TotkView from "./components/totk/TotkView";
import ErrorBanner from "./components/ErrorBanner";
import { useThemeAttributes } from "./theme/useThemeAttributes";
import styles from "./App.module.css";

function App() {
  const [loaded, setLoaded] = useState<OpenResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  useThemeAttributes(loaded?.kind ?? "neutral");

  const handleOpen = async () => {
    try {
      const result = await api.openSave();
      setLoaded(result);
      setError(null);
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleSave = async () => {
    try {
      await api.saveSave();
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleSaveAs = async () => {
    try {
      await api.saveAs();
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const refreshCurrent = async () => {
    if (!loaded) return;
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

  return (
    <div className={styles.app}>
      <h1 className={styles.title}>Zelda Save Editor</h1>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />
      <div className={styles.toolbar}>
        <button className={styles.button} onClick={handleOpen}>
          Open...
        </button>
        <button className={styles.button} onClick={handleSave} disabled={!loaded}>
          Save
        </button>
        <button className={styles.button} onClick={handleSaveAs} disabled={!loaded}>
          Save As...
        </button>
        <button className={styles.button} onClick={refreshCurrent} disabled={!loaded}>
          Refresh
        </button>
      </div>
      {loaded?.kind === "botw" && (
        <BotwView state={loaded.state} onError={setError} onRefresh={refreshCurrent} />
      )}
      {loaded?.kind === "totk" && (
        <TotkView state={loaded.state} onError={setError} onRefresh={refreshCurrent} />
      )}
      {!loaded && <p className={styles.empty}>Open a save file to begin.</p>}
    </div>
  );
}

export default App;
