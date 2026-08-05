import styles from "./CloseConfirmModal.module.css";

interface Props {
  onSave: () => void;
  onDiscard: () => void;
  onCancel: () => void;
  message?: string;
}

export default function CloseConfirmModal({
  onSave,
  onDiscard,
  onCancel,
  message = "You have unsaved edits. Save them before closing?",
}: Props) {
  return (
    <div className={styles.overlay}>
      <div className={styles.panel}>
        <h2 className={styles.title}>Unsaved changes</h2>
        <p className={styles.message}>{message}</p>
        <div className={styles.actions}>
          <button className={styles.button} onClick={onSave}>
            Save
          </button>
          <button className={styles.button} onClick={onDiscard}>
            Discard
          </button>
          <button className={styles.button} onClick={onCancel}>
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
