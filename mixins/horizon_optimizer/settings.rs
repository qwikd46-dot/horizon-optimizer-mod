// Settings hook: called when the mod's settings page opens.
// Pushes the labels of the settings buttons.

fn horizon_optimizer_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::horizon_optimizer as m;

    if mod_id != m::ID {
        return;
    }

    let get = |key: &str| game.native_setting(m::ID, key);
    let show_fps = m::show_fps(get);
    let show_timing = m::show_timing(get);
    let optimize_ticks = m::optimize_ticks(get);
    let optimize_frames = m::optimize_frames(get);
    let target_fps = m::target_fps(get);

    buttons.push(format!("Show FPS: {}", if show_fps { "ON" } else { "OFF" }));
    buttons.push(format!("Show Timing: {}", if show_timing { "ON" } else { "OFF" }));
    buttons.push(format!("Optimize Ticks: {}", if optimize_ticks { "ON" } else { "OFF" }));
    buttons.push(format!("Optimize Frames: {}", if optimize_frames { "ON" } else { "OFF" }));
    buttons.push(format!("Target FPS: {}", if target_fps == 0 { "Unlimited".to_string() } else { target_fps.to_string() }));
}