// Settings hook: called when the mod's settings page opens.

fn horizon_optimizer_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::horizon_optimizer as m;

    if mod_id != m::ID {
        return;
    }

    let get = |key: &str| game.native_setting(m::ID, key);
    let show_fps = m::show_fps(get);
    let show_avg_time = m::show_avg_time(get);
    let show_last_time = m::show_last_time(get);
    let optimize_ticks = m::optimize_ticks(get);
    let optimize_frames = m::optimize_frames(get);
    let target_fps = m::target_fps(get);
    let chunk_lod = m::chunk_lod(get);
    let chunk_preload = m::chunk_preload(get);
    let chunk_fast = m::chunk_fast(get);

    buttons.push(format!("Show FPS: {}", if show_fps { "ON" } else { "OFF" }));
    buttons.push(format!("Show Avg Time: {}", if show_avg_time { "ON" } else { "OFF" }));
    buttons.push(format!("Show Last Time: {}", if show_last_time { "ON" } else { "OFF" }));
    buttons.push(format!("Optimize Ticks: {}", if optimize_ticks { "ON" } else { "OFF" }));
    buttons.push(format!("Optimize Frames: {}", if optimize_frames { "ON" } else { "OFF" }));
    buttons.push(format!("Target FPS: {}", if target_fps == 0 { "Unlimited".to_string() } else { target_fps.to_string() }));
    buttons.push(format!("Chunk LOD: {}", if chunk_lod { "ON" } else { "OFF" }));
    buttons.push(format!("Chunk Preload: {}", if chunk_preload { "ON" } else { "OFF" }));
    buttons.push(format!("Chunk Fast: {}", if chunk_fast { "ON" } else { "OFF" }));
}