# Zelda AIO Save Editor

A unified save editor for *The Legend of Zelda: Breath of the Wild* and *Tears of the Kingdom*,
built as a Rust save-parsing core with a Tauri desktop shell.

> **Status: early development.** The save-format engine and its desktop IPC wiring are functional
> and tested against real save files, but no editor UI has been built yet — see
> [Status & roadmap](#status--roadmap) below.

## Features

### Breath of the Wild
- Player stats (hearts, stamina, rupees, mon, playtime), positions, and current map
- Full inventory: weapons, bows, shields, armor, materials, food, and key items, with automatic
  item categorization
- Weapon/bow/shield modifier slots
- Horses (name, saddle, reins, type)
- Completionism mass-unlock: koroks, defeated hinox/talus/molduga, visited locations

### Tears of the Kingdom
- Player stats, save position, and current checkpoint
- Full pouch inventory: weapons, bows, shields, armor, arrows, materials, key items, Zonai
  devices, and food (including fused-weapon and recipe data)
- Horses, including bond, stats, and coat/mane/eye coloring
- Completionism counts: shrines, koroks, defeated bosses, visited locations (read-only, matching
  the absence of a mass-unlock feature for these categories in-game)
- AutoBuild: all 30 saved Ultrahand schematics, including camera framing and favorite status

## Project layout

```
crates/save-engine/   headless Rust library: binary parsing, hash-table resolution, and typed
                       read/write accessors for both games' save formats. No UI dependencies.
src-tauri/             Tauri v2 backend: converts save-engine's types to serializable DTOs and
                       exposes them as IPC commands, plus native file open/save dialogs.
src/                    React + TypeScript frontend (Vite). Not yet built out beyond scaffolding.
docs/                   Design specs and implementation plans for each shipped slice of work.
```

Everything under `crates/save-engine` builds and tests independently of Tauri, so the parsing
logic can be reused (or audited) without pulling in a desktop toolchain.

## Getting started

**Prerequisites:** a recent [Rust toolchain](https://rustup.rs/), [Node.js](https://nodejs.org/)
18+, and the [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your
platform.

```bash
npm install

# Run the engine's own test suite (no Tauri/Node required)
cargo test -p save-engine

# Run the desktop shell's IPC layer tests
cargo test -p zelda-save-shell

# Launch the desktop app in development mode
npm run tauri dev
```

Target platforms are Windows, Linux, and Android.

## Status & roadmap

The save-format core is complete for both games' most commonly edited data (see Features above),
and every shipped engine capability is wired through to the Tauri IPC layer with a typed
command per read/write operation. What's still ahead:

- Editor UI — the frontend currently has no real screens, only the IPC scaffolding
- A visual design pass
- Responsive layout across desktop and mobile form factors
- Packaging and release automation (installers, Android APK/AAB, CI)
- BOTW Master Mode, TOTK map icons/pins, and a handful of other lower-priority save fields

## Credits

The BOTW and TOTK save formats implemented here — hash tables, field offsets, and encoding
details — were reverse-engineered by studying
[marcrobledo/savegame-editors](https://github.com/marcrobledo/savegame-editors) (MIT licensed),
whose browser-based `zelda-botw` and `zelda-totk` tools were the original prior art for editing
these save files. This project is an independent Rust reimplementation built on top of that
research, not a fork of its code.

## Disclaimer

This project is not affiliated with or endorsed by Nintendo. It edits save files for personal
and educational use; always keep a backup of your save data before editing it.

## License

MIT — see [LICENSE](LICENSE).
