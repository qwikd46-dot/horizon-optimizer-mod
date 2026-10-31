// Horizon Optimizer - Performance optimization mod for Horizon (Rust-based Minecraft)
//
// This module is compiled into the game as `crate::game::horizon_optimizer`.
// It provides FPS monitoring, frame timing, tick counting, chunk optimization, and settings.
//
// State lives in thread_local! cells for what changes; the window's game runs on
// one thread, so its hooks see the same state.

use std::cell::Cell;
use std::time::Instant;

pub const ID: &str = "horizon_optimizer";
pub const NAME: &str = "Horizon Optimizer";
pub const VERSION: &str = "0.3.0";
pub const DESCRIPTION: &str = "Performance optimization: FPS counter (smoothed), frame timing, tick stats, chunk optimization settings.";

// ---- Settings keys ----
const KEY_SHOW_FPS: &str = "show_fps";
const KEY_SHOW_AVG_TIME: &str = "show_avg_time";
const KEY_SHOW_LAST_TIME: &str = "show_last_time";
const KEY_OPTIMIZE_TICKS: &str = "optimize_ticks";
const KEY_OPTIMIZE_FRAMES: &str = "optimize_frames";
const KEY_TARGET_FPS: &str = "target_fps";
// ---- Chunk handling settings ----
const KEY_CHUNK_RENDER_DIST: &str = "chunk_render_dist";
const KEY_CHUNK_LOD: &str = "chunk_lod";
const KEY_CHUNK_PRELOAD: &str = "chunk_preload";
const KEY_CHUNK_FAST: &str = "chunk_fast";
// ---- Chunk/RSS settings ----
const KEY_CHUNK_RENDER_DIST: &str = "chunk_render_dist";
const KEY_CHUNK_LOD: &str = "chunk_lod";
const KEY_CHUNK_PRELOAD: &str = "chunk_preload";
const KEY_CHUNK_FAST: &str = "chunk_fast";

// ---- Thread-local state ----

thread_local! {
    // Frame timing
    pub static LAST_FRAME_TIME: Cell<u128> = const { Cell::new(0) };
    pub static FPS_COUNT: Cell<u32> = const { Cell::new(0) };
    pub static FPS_LAST_UPDATE: Cell<Option<Instant>> = const { Cell::new(None) };
    pub static CURRENT_FPS: Cell<u32> = const { Cell::new(0) };
    pub static SMOOTH_FPS: Cell<u32> = const { Cell::new(0) };

    // Tick counting
    pub static TICK_COUNT: Cell<u64> = const { Cell::new(0) };
    pub static LAST_TICK_TIME: Cell<u128> = const { Cell::new(0) };

    // Frame time history (last 60 frames for averaging)
    pub static FRAME_TIMES: Cell<[u128; 60]> = const { Cell::new([0; 60]) };
    pub static FRAME_INDEX: Cell<usize> = const { Cell::new(0) };
}

// ---- Settings accessors ----

pub fn get_bool(get: impl Fn(&str) -> Option<String>, key: &str, default: bool) -> bool {
    match get(key) {
        Some(v) => v.trim().to_lowercase() == "true",
        None => default,
    }
}

pub fn show_fps(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_SHOW_FPS, true)
}

pub fn show_avg_time(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_SHOW_AVG_TIME, true)
}

pub fn show_last_time(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_SHOW_LAST_TIME, true)
}

pub fn optimize_ticks(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_OPTIMIZE_TICKS, true)
}

pub fn optimize_frames(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_OPTIMIZE_FRAMES, true)
}

pub fn target_fps(get: impl Fn(&str) -> Option<String>) -> u32 {
    get(KEY_TARGET_FPS)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

// ---- Chunk optimization accessors ----
pub fn chunk_render_dist(get: impl Fn(&str) -> Option<String>) -> u32 {
    get(KEY_CHUNK_RENDER_DIST).and_then(|s| s.parse().ok()).unwrap_or(12)
}

pub fn chunk_lod(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_CHUNK_LOD, true)
}

pub fn chunk_preload(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_CHUNK_PRELOAD, true)
}

pub fn chunk_fast(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_CHUNK_FAST, true)
}

// ---- Chunk optimization accessors ----

pub fn chunk_render_dist(get: impl Fn(&str) -> Option<String>) -> u32 {
    get(KEY_CHUNK_RENDER_DIST).and_then(|s| s.parse().ok()).unwrap_or(12)
}

pub fn chunk_lod(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_CHUNK_LOD, true)
}

pub fn chunk_preload(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_CHUNK_PRELOAD, true)
}

pub fn chunk_fast(get: impl Fn(&str) -> Option<String>) -> bool {
    get_bool(get, KEY_CHUNK_FAST, true)
}

pub fn set_setting<W: Fn(&str, &str)>(set: W, key: &str, val: &str) {
    set(key, val);
}

// ---- Helpers ----

pub fn format_duration(us: u128) -> String {
    if us < 1_000 {
        format!("{}µs", us)
    } else if us < 1_000_000 {
        format!("{:.1}ms", us as f64 / 1_000.0)
    } else {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    }
}

pub fn format_fps(fps: u32) -> String {
    if fps >= 60 {
        format!("§a{} FPS", fps)
    } else if fps >= 30 {
        format!("§e{} FPS", fps)
    } else {
        format!("§c{} FPS", fps)
    }
}

pub fn avg_frame_time() -> u128 {
    FRAME_TIMES.with(|times| {
        let idx = FRAME_INDEX.with(|i| i.get());
        let arr = times.get();
        let sum: u128 = arr.iter().sum();
        if idx > 0 { sum / idx as u128 } else { 0 }
    })
}

// ---- Unit tests ----
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(1000), "1.0ms");
        assert_eq!(format_duration(1_000_000), "1.00s");
    }
    #[test]
    fn test_format_fps_colors() {
        assert!(format_fps(60).contains("§a"));
        assert!(format_fps(10).contains("§c"));
    }
}
