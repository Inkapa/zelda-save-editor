import styles from "./WavyFrame.module.css";

// A hand-drawn-looking wavy rectangle outline, used as the border of the tab buttons instead of a
// straight box. Drawn in a normalized viewBox and stretched to the button with
// preserveAspectRatio="none"; the stroke stays crisp via vector-effect non-scaling-stroke, and the
// waves stretch a little per button width, which reads as organic rather than rigid.
const W = 100;
const H = 40;
const PAD = 4;
const AMP = 1.6;

function edge(
  n: number,
  waves: number,
  point: (p: number, offset: number) => [number, number],
  startAt: number,
): [number, number][] {
  const pts: [number, number][] = [];
  for (let i = startAt; i <= n; i++) {
    const p = i / n;
    const offset = AMP * Math.sin(p * waves * 2 * Math.PI);
    pts.push(point(p, offset));
  }
  return pts;
}

function wavyRectPath(): string {
  const iw = W - 2 * PAD;
  const ih = H - 2 * PAD;
  const pts: [number, number][] = [
    ...edge(28, 7, (p, o) => [PAD + p * iw, PAD + o], 0), // top
    ...edge(8, 2, (p, o) => [W - PAD + o, PAD + p * ih], 1), // right
    ...edge(28, 7, (p, o) => [W - PAD - p * iw, H - PAD + o], 1), // bottom
    ...edge(8, 2, (p, o) => [PAD + o, H - PAD - p * ih], 1), // left
  ];
  return "M" + pts.map(([x, y]) => `${x.toFixed(1)},${y.toFixed(1)}`).join(" L") + " Z";
}

const PATH = wavyRectPath();

export default function WavyFrame() {
  return (
    <svg
      className={styles.frame}
      viewBox={`0 0 ${W} ${H}`}
      preserveAspectRatio="none"
      aria-hidden="true"
    >
      <path d={PATH} vectorEffect="non-scaling-stroke" />
    </svg>
  );
}
