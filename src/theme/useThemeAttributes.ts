import { useEffect, useState } from "react";

export type Game = "botw" | "totk" | "neutral";

function readScheme(): "light" | "dark" {
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

export function useThemeAttributes(game: Game): void {
  const [scheme, setScheme] = useState(readScheme);

  useEffect(() => {
    const mql = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = () => setScheme(readScheme());
    mql.addEventListener("change", onChange);
    return () => mql.removeEventListener("change", onChange);
  }, []);

  useEffect(() => {
    document.documentElement.dataset.game = game;
  }, [game]);

  useEffect(() => {
    document.documentElement.dataset.scheme = scheme;
  }, [scheme]);
}
