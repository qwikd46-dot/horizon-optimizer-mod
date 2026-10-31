// HUD hook: called when the HUD is drawn (not under F1).
// Draws the FPS counter (smoothed), frame timing, and optimization overlay.
// The box uses a FIXED width so only the numbers change, never the box position.

fn horizon_optimizer_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::horizon_optimizer as m;
    use horizon_client::extras::Corner;

    if !game.native_on(m::ID) {
        return;
    }
    if !game.native_playing() {
        return;
    }
    if !game.native_window() {
        return;
    }

    let get  = |key: &str| game.native_setting(m::ID, key);
    let show_fps       = m::show_fps(get);
    let show_avg_time  = m::show_avg_time(get);
    let show_last_time = m::show_last_time(get);

    if !show_fps && !show_avg_time && !show_last_time {
        return;
    }

    // Always reserve enough width for the longest possible line.  This keeps
    // the box stationary (only the digits inside it change).
    let fixed_w: f32 = 160.0;

    // Find the first available slot in the top-right corner, below any other
    // (this is what keeps it anchored).
    let (x, y) = corners.place(Corner::TopRight, fixed_w, 14.0);

    // Draw the stable background box.
    game.native_fill(batch, [x, y, fixed_w, 14.0], 0x7800_0000);

    // Now compose the text.  All lines share the same box; nothing recomputes
    // the width each frame, so the position never moves.
    let mut t_y = y + 2.0;

    if show_fps {
        let fps = m::SMOOTH_FPS.with(|s| s.get());
        if fps > 0 {
            game.native_text(batch, &m::format_fps(fps), [x + 2.0, t_y], 0xFFFFFF, true);
            t_y += 1.0; // tiny stagger so lines overlap cleanly
        }
    }
    if show_avg_time {
        let avg = m::avg_frame_time();
        if avg > 0 {
            let s = format!("§7avg: {}", m::format_duration(avg));
            game.native_text(batch, &s, [x + 2.0, t_y], 0xFFFFFF, true);
            t_y += 1.0;
        }
    }
    if show_last_time {
        let last = m::LAST_FRAME_TIME.with(|t| t.get());
        if last > 0 {
            let s = format!("§7last: {}", m::format_duration(last));
            game.native_text(batch, &s, [x + 2.0, t_y], 0xFFFFFF, true);
            t_y += 1.0;
        }
    }
    // Hide the y offset from the compiler
    let _ = t_y;
}