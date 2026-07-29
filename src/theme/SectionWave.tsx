import { useEffect, useRef } from "react";
import { subscribe } from "./waveClock";

const WIDTH = 400;
const HEIGHT = 24;
const AMPLITUDE = 7;
const FREQUENCY = 10;
const SPEED = 0.012;
const STEPS = 80;

function triangleWave(x: number): number {
  const raw = (2 / Math.PI) * Math.asin(Math.sin(x));
  return Math.round(raw * 7) / 7;
}

function buildPoints(t: number): string {
  const dtAngle = t * SPEED * 60;
  const parts: string[] = [];
  for (let i = 0; i <= STEPS; i++) {
    const progress = i / STEPS;
    const angle = progress * FREQUENCY * 2 * Math.PI + dtAngle;
    const y = (HEIGHT / 2 + AMPLITUDE * triangleWave(angle)).toFixed(2);
    parts.push(`${(progress * WIDTH).toFixed(1)},${y}`);
  }
  return parts.join(" ");
}

export default function SectionWave() {
  const lineRef = useRef<SVGPolylineElement>(null);

  useEffect(() => {
    const el = lineRef.current;
    if (!el) return;
    el.setAttribute("points", buildPoints(0));
    return subscribe((t) => el.setAttribute("points", buildPoints(t)));
  }, []);

  return (
    <svg
      width="100%"
      height={HEIGHT}
      viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
      preserveAspectRatio="none"
      style={{ display: "block" }}
    >
      <polyline ref={lineRef} fill="none" stroke="var(--accent)" strokeWidth={1.5} strokeLinejoin="miter" />
    </svg>
  );
}
