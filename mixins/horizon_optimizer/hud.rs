// HUD hook: called when the HUD is drawn (not under F1).
// Draws the FPS counter, frame timing, and optimization overlay.

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

    // Only show when playing
    if !game.native_playing() {
        return;
    }

    // Only show on the window's game
    if !game.native_window() {
        return;
    }

    let get = |key: &str| game.native_setting(m::ID, key);
    let show_fps = m::show_fps(get);
    let show_timing = m::show_timing(get);

    if !show_fps && !show_timing {
        return;
    }

    // Build the text lines
    let mut lines: Vec<String> = Vec::new();

    if show_fps {
        let fps = m::CURRENT_FPS.with(|c| c.get());
        lines.push(m::format_fps(fps));
    }

    if show_timing {
        let avg = m::avg_frame_time();
        lines.push(format!("§7Avg: {}§r", m::format_duration(avg)));

        let last = m::LAST_FRAME_TIME.with(|t| t.get());
        lines.push(format!("§7Last: {}§r", m::format_duration(last)));
    }

    // Place in top-right corner
    if !lines.is_empty() {
        let text = lines.join(" §7| ");
        let w = game.native_text_width(&text) + 4.0;
        let (x, y) = corners.place(Corner::TopRight, w, 12.0);
        game.native_fill(batch, [x, y, w, 12.0], 0x7800_0000);
        game.native_text(batch, &text, [x + 2.0, y + 2.0], 0xFFFFFF, true);
    }
}