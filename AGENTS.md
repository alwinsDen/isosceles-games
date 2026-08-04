# AGENTS.md

Bevy game monorepo (`isosceles-games`). Rust workspace, edition 2024, resolver "3".

## Commands
- Build/check workspace: `cargo build` / `cargo check`
- Run the game: `cargo run -p v-26-2026`
- Check the plugin lib: `cargo check -p custom-plugins-bevy`
- Check the CLI: `cargo check -p isosceles-cli`
- Run the Tauri desktop app: `cargo tauri dev` (run from `isosceles-desktop/`, not root — Trunk + Tauri use relative paths)
- No tests, lint, or CI config exist in this repo.

## Critical: local Bevy checkout
- `bevy = { path = "../bevy", default-features = false }` (root `Cargo.toml`). The repo compiles against a **local** bevy checkout at `../bevy` (version `0.20.0-dev`), NOT crates.io. Never bump bevy to a released version; keep the local checkout up to date.
- Code targets bevy's unreleased snapshot/ECS-scene API: `bsn_list!`, `SceneList`, `asset_value`, `template_value`, `SpawnListSystem`. Released Bevy tutorials/docs (0.17–0.19) will not compile here; consult `../bevy` source for the real API.
- Bevy builds from source in debug, so compiles are slow. The `[profile.dev] opt-level` tuning in `v-26-2026/Cargo.toml` is intentionally commented out.

## Layout
- `custom-plugins-bevy/` — lib crate aggregating reusable plugins; expose new plugins from `src/lib.rs`.
- `v-26-2026/` — active game binary. Entrypoint `src/main.rs`: `dev_pointer_grip` gizmo grid + `opening_scene` (spawned via `bsn_list!`), then `HelloPlugin`.
- `isosceles-cli/` — ratatui TUI launcher (runner search). Workspace member, standalone crate.
- `isosceles-desktop/` — Leptos CSR + Trunk + Tauri v2 desktop app. `src-tauri/` is a workspace member; the Leptos crate at root is WASM-only (excluded from workspace via `exclude`).
- `src/scene/` and `src/utils/` follow a `mod.rs` + child-module pattern; add scenes/utils there.
- `_docs_Isosceles_Games/` — Obsidian notes vault mirroring the README; not build-relevant.

## Conventions
- Conventional commit messages (e.g. `feat:`, `fix:`).
