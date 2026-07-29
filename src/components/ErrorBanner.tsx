interface Props {
  message: string | null;
  onDismiss: () => void;
}

export default function ErrorBanner({ message, onDismiss }: Props) {
  if (!message) return null;
  return (
    <div style={{ background: "#fee", border: "1px solid #c00", padding: 8, marginBottom: 8 }}>
      {message}{" "}
      <button onClick={onDismiss}>Dismiss</button>
    </div>
  );
}
