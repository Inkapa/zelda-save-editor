# Contributing

This project is still in early development (see the [README](README.md#status--roadmap) for
current status), so the most useful contributions right now are:

- Bug reports against real save files (please don't attach save files containing personal
  progress you're not comfortable sharing publicly)
- Fixes to the save-format engine in `crates/save-engine`, especially anything verified against
  an actual `.sav` file
- Discussion on open issues before large changes, since the format research this project relies
  on is easy to get subtly wrong

## Development

```bash
npm install
cargo test -p save-engine      # engine unit + integration tests
cargo test -p zelda-save-shell # Tauri IPC layer tests
npm run tauri dev              # run the desktop app
```

## Guidelines

- Save-format facts (hash values, offsets, field sizes) should be verified against a real save
  file or the source data they're derived from, not assumed from documentation alone
- New engine functionality should include a test against a real fixture in
  `crates/save-engine/tests/fixtures/`, not just a synthetic example
- Keep commit messages focused on the *why*, not just a restatement of the diff

## Reporting issues

Open a GitHub issue with:
- What you were trying to do
- What game and save-file version you're working with (if known)
- The exact error or unexpected behavior, and if possible the offset/field involved
