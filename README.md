# Horizon Optimizer

A native optimization mod for **Horizon** (the Rust-based Minecraft client).

This mod provides performance monitoring and optimization controls that Horizon doesn't include out of the box.

## Features

- **FPS Counter** — Real-time FPS display with color coding (green/yellow/red)
- **Frame Timing** — Average and last frame time display
- **Tick Counter** — Tracks game tick count
- **Performance Stats** — FPS, 1% lows, slowest frame, and tick count via `/optstats`
- **Optimization Toggles** — Enable/disable tick and frame optimizations
- **Target FPS Limiter** — Set a target FPS cap (0 = unlimited)
- **Settings Page** — Full config via Options > Mods > Horizon Optimizer > Settings
- **Keyboard Shortcuts** — F6 to toggle FPS, F7 to toggle timing overlay

## Commands

| Command | Description |
|---------|-------------|
| `/optfps` | Toggle FPS overlay |
| `/opttiming` | Toggle timing overlay |
| `/optstats` | Show performance stats in chat |
| `/optreset` | Reset all settings to defaults |
| `/opttarget <fps>` | Set target FPS (0 = unlimited) |

## Installation

1. Install the Horizon launcher (0.2.23 or later)
2. Have Minecraft Java Edition 26.3 prepared
3. In the launcher, go to Mods and add this mod
4. Build the game (first build takes 20-30 minutes)
5. Play with the built instance

## How It Works

Horizon is written in Rust and compiles mods directly into the game binary. This mod hooks into Horizon's native hook points:
- `native_frame` — Frame timing and FPS calculation
- `native_tick` — Tick counting and tick optimization
- `native_hud` — Drawing the FPS/timing overlay
- `native_key` — Keyboard shortcut handling
- `native_command` — Command processing
- `native_mods` — Mod listing in Options > Mods
- `native_settings` — Settings page
- `native_settings_click` — Settings interaction

## License

MIT