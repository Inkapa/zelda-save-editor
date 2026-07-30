const COMMON = { fill: "none", stroke: "var(--accent)", strokeWidth: 1.4, strokeLinecap: "round" as const, strokeLinejoin: "round" as const };

export function HeartIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M10 17 C4 12.5 1 8.8 1 5.6 A4 4 0 0 1 10 5.6 A4 4 0 0 1 19 5.6 C19 8.8 16 12.5 10 17 Z" {...COMMON} />
    </svg>
  );
}

export function BagIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M6 8 Q6 3 10 3 Q14 3 14 8" {...COMMON} />
      <path d="M4 8 L16 8 L15 18 L5 18 Z" {...COMMON} />
    </svg>
  );
}

export function HorseshoeIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M5 18 L5 11 A5 5 0 0 1 15 11 L15 18" {...COMMON} />
      <circle cx="5" cy="16.5" r="0.9" fill="var(--accent)" stroke="none" />
      <circle cx="15" cy="16.5" r="0.9" fill="var(--accent)" stroke="none" />
    </svg>
  );
}

export function GearIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <polygon points="10,2 16,5.5 16,13.5 10,17 4,13.5 4,5.5" {...COMMON} />
      <circle cx="10" cy="9.5" r="3" {...COMMON} />
    </svg>
  );
}

export function PinIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M10 2 C6.5 2 4 4.7 4 8 C4 12 10 18 10 18 C10 18 16 12 16 8 C16 4.7 13.5 2 10 2 Z" {...COMMON} />
      <circle cx="10" cy="8" r="2" fill="var(--accent)" stroke="none" />
    </svg>
  );
}

export function StarIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path
        d="M10 2 L12.2 7.6 L18 8.2 L13.6 12 L15 17.8 L10 14.6 L5 17.8 L6.4 12 L2 8.2 L7.8 7.6 Z"
        {...COMMON}
      />
    </svg>
  );
}

export function SearchIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <circle cx="8.5" cy="8.5" r="5.5" {...COMMON} />
      <line x1="12.5" y1="12.5" x2="18" y2="18" {...COMMON} />
    </svg>
  );
}

export function SparkIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path
        d="M10 2 L11.2 8.8 L18 10 L11.2 11.2 L10 18 L8.8 11.2 L2 10 L8.8 8.8 Z"
        fill="var(--accent)"
        stroke="none"
      />
    </svg>
  );
}

export function SwordIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <line x1="4" y1="16" x2="15" y2="5" {...COMMON} />
      <line x1="10" y1="4" x2="16" y2="10" {...COMMON} />
      <line x1="3" y1="12" x2="8" y2="17" {...COMMON} />
      <line x1="3" y1="17" x2="5" y2="15" {...COMMON} />
    </svg>
  );
}

export function BowIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M6 2 A11 11 0 0 0 6 18" {...COMMON} />
      <line x1="6" y1="2" x2="6" y2="18" {...COMMON} strokeDasharray="1.5 1.6" />
      <line x1="2" y1="10" x2="17" y2="10" {...COMMON} />
    </svg>
  );
}

export function ShieldIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M10 2 L17 5 V9.5 C17 14 14 17 10 18.5 C6 17 3 14 3 9.5 V5 Z" {...COMMON} />
    </svg>
  );
}

export function ShirtIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path
        d="M7 2 L4 4.5 L2 7.5 L4.5 9.5 L6 8 V18 H14 V8 L15.5 9.5 L18 7.5 L16 4.5 L13 2 C13 3.5 11.5 4.5 10 4.5 C8.5 4.5 7 3.5 7 2 Z"
        {...COMMON}
      />
    </svg>
  );
}

export function AppleIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path
        d="M10 7 C6 7 4 9.5 4 12.5 C4 15.5 6.5 18 8.5 18 C9.2 18 9.6 17.6 10 17.6 C10.4 17.6 10.8 18 11.5 18 C13.5 18 16 15.5 16 12.5 C16 9.5 14 7 10 7 Z"
        {...COMMON}
      />
      <path d="M10 7 C10 5 9 4 9 4" {...COMMON} />
      <path d="M10 4.5 C11 4 12 4.5 12 4.5" {...COMMON} />
    </svg>
  );
}

export function ArrowIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <line x1="3" y1="17" x2="17" y2="3" {...COMMON} />
      <path d="M9 3 L17 3 L17 11" {...COMMON} />
    </svg>
  );
}

export function BlueprintIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <rect x="3" y="3" width="14" height="14" {...COMMON} />
      <line x1="3" y1="8" x2="17" y2="8" {...COMMON} />
      <line x1="8" y1="3" x2="8" y2="17" {...COMMON} />
      <circle cx="12.5" cy="12.5" r="1.1" fill="var(--accent)" stroke="none" />
    </svg>
  );
}

export function LeafIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M4 16 C4 8 10 3 17 3 C17 10 12 16 4 16 Z" {...COMMON} />
      <line x1="4" y1="16" x2="12" y2="8" {...COMMON} />
    </svg>
  );
}

export function ChestIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M3 8 L3 16 L17 16 L17 8 Z" {...COMMON} />
      <path d="M3 8 C3 5 5 4 10 4 C15 4 17 5 17 8" {...COMMON} />
      <circle cx="10" cy="11" r="1.1" fill="var(--accent)" stroke="none" />
    </svg>
  );
}
