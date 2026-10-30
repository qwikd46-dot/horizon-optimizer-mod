// Horizon Optimizer - Performance optimization mod for Horizon (Rust-based Minecraft)
//
// This module is compiled into the game as `crate::game::horizon_optimizer`.
// It provides FPS monitoring, frame timing, tick counting, and optimization settings.
//
// State lives in thread_local! cells for what changes; the window's game runs on
// one thread, so its hooks see the same state.

use std::cell::Cell;
use std::time::Instant;

pub const ID: &str = "horizon_optimizer";
pub const NAME: &str = "Horizon Optimizer";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Performance monitoring and optimization: FPS counter, frame timing, tick stats, and optimization settings.";

// ---- Settings keys ----
const KEY_SHOW_FPS: &str = "show_fps";
const KEY_SHOW_TIMING: &str = "show_timing";
const KEY_OPTIMIZE_TICKS: &str = "optimize_ticks";
const KEY_OPTIMIZE_FRAMES: &str = "optimize_frames";
const KEY_TARGET_FPS: &str = "target_fps";

// ---- Thread-local state ----

thread_local! {
    // Frame timing
    pub static FRAME_START: Cell<Option<Instant>> = const { Cell::new(None) };
    pub static LAST_FRAME_TIME: Cell<u128> = const { Cell::new(0) };
    pub static FPS_COUNT: Cell<u32> = const { Cell::new(0) };
    pub static FPS_LAST_UPDATE: Cell<Option<Instant>> = const { Cell::new(None) };
    pub static CURRENT_FPS: Cell<u32> = const { Cell::new(0) };

    // Tick counting
    pub static TICK_COUNT: Cell<u64> = const { Cell::new(0) };
    pub static LAST_TICK_TIME: Cell<u128> = const { Cell::new(0) };

    // Frame time history (last 60 frames for averaging)
    pub static FRAME_TIMES: Cell<[u128; 60]> = const { Cell::new([0; 60]) };
    pub static FRAME_INDEX: Cell<usize> = const { Cell::new(0) };

    // Optimization state
    pub static OPTIMIZE_ENABLED: Cell<bool> = const { Cell::new(true) };
}

// ---- Helper functions ----

/// Get the saved "show_fps" setting (default: true).
pub fn show_fps(get: impl Fn(&str) -> Option<String>) -> bool {
    get(KEY_SHOW_FPS).map(|s| s != "false").unwrap_or(true)
}

/// Get the saved "show_timing" setting (default: false).
pub fn show_timing(get: impl Fn(&str) -> Option<String>) -> bool {
    get(KEY_SHOW_TIMING).map(|s| s == "true").unwrap_or(false)
}

/// Get the saved "optimize_ticks" setting (default: true).
pub fn optimize_ticks(get: impl Fn(&str) -> Option<String>) -> bool {
    get(KEY_OPTIMIZE_TICKS).map(|s| s != "false").unwrap_or(true)
}

/// Get the saved "optimize_frames" setting (default: true).
pub fn optimize_frames(get: impl Fn(&str) -> Option<String>) -> bool {
    get(KEY_OPTIMIZE_FRAMES).map(|s| s != "false").unwrap_or(true)
}

/// Get the saved "target_fps" setting (default: 0 = unlimited).
pub fn target_fps(get: impl Fn(&str) -> Option<String>) -> u32 {
    get(KEY_TARGET_FPS)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Format a duration in microseconds to a human-readable string.
pub fn format_duration(us: u128) -> String {
    if us < 1_000 {
        format!("{}µs", us)
    } else if us < 1_000_000 {
        format!("{:.1}ms", us as f64 / 1_000.0)
    } else {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    }
}

/// Format FPS with color coding.
pub fn format_fps(fps: u32) -> String {
    if fps >= 60 {
        format!("§a{} FPS", fps)
    } else if fps >= 30 {
        format!("§e{} FPS", fps)
    } else {
        format!("§c{} FPS", fps)
    }
}

/// Compute average frame time from the rolling buffer.
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
    fn test_format_duration_microseconds() {
        assert_eq!(format_duration(500), "500µs");
        assert_eq!(format_duration(999), "999µs");
    }

    #[test]
    fn test_format_duration_milliseconds() {
        assert_eq!(format_duration(1000), "1.0ms");
        assert_eq!(format_duration(5500), "5.5ms");
        assert_eq!(format_duration(16666), "16.7ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(1_000_000), "1.00s");
        assert_eq!(format_duration(2_500_000), "2.50s");
    }

    #[test]
    fn test_format_fps_colors() {
        assert!(format_fps(60).contains("§a"));
        assert!(format_fps(30).contains("§e"));
        assert!(format_fps(10).contains("§c"));
    }
}