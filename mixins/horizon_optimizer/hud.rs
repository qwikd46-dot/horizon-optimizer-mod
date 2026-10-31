// HUD hook: called when the HUD is drawn (not under F1).
// Draws the FPS counter (smoothed), frame timing, and optimization overlay.

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

    let get = |key: &str| game.native_setting(m::ID, key);
    let show_fps = m::show_fps(get);
    let show_avg_time = m::show_avg_time(get);
    let show_last_time = m::show_last_time(get);

    if !show_fps && !show_avg_time && !show_last_time {
        return;
    }

    let mut lines: Vec<String> = Vec::new();

    if show_fps {
        // Use smoothed FPS to prevent jitter
        let fps = m::SMOOTH_FPS.with(|s| s.get());
        if fps > 0 {
            lines.push(m::format_fps(fps));
        }
    }

    if show_avg_time {
        let avg = m::avg_frame_time();
        if avg > 0 {
            lines.push(format!("§7avg: {}§r", m::format_duration(avg)));
        }
    }

    if show_last_time {
        let last = m::LAST_FRAME_TIME.with(|t| t.get());
        if last > 0 {
            lines.push(format!("§7last: {}§r", m::format_duration(last)));
        }
    }

    if !lines.is_empty() {
        let text = lines.join(" §7| ");
        let w = game.native_text_width(&text) + 4.0;
        let (x, y) = corners.place(Corner::TopRight, w, 12.0);
        game.native_fill(batch, [x, y, w, 12.0], 0x7800_0000);
        game.native_text(batch, &text, [x + 2.0, y + 2.0], 0xFFFFFF, true);
    }
}