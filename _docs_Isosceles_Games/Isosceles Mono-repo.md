# Isosceles - Mono-repo

**Collection of games made with ECS-model of the Bevy Engine.** 

### Current scope: 
* `custom-plugins-bevy`  - Rust lib ~ acting as collection of custom plugins to support the mono-repo games.
* `v-26-2026` - Scoped in-dev game of the repo (orthographic, fixed-camera).
* `isosceles-cli` - light-weight CLI tool to support fast game dev.
* `isosceles-desktop` - tauri-based desktop app ~ possibly expand into an editor for Bevy use-cases.

### Build from source:

Install Rust (https://rust-lang.org/tools/install/):
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Package based runs: 
```shell
# install dependencies
cargo build

# run v-26-2026 package
cargo run -p v-26-2026

# run tauri isosceles-desktop-system app.
TAURI_APP_PATH=isosceles-desktop-system cargo tauri dev

# system-wide debug version install for CLI.
cargo install --path isosceles-cli --debug
```

