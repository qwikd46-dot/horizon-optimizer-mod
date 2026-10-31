// Settings click hook: called when a settings button is clicked.

fn horizon_optimizer_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::horizon_optimizer as m;

    if mod_id != m::ID {
        return;
    }

    let set = |key: &str, val: &str| game.native_set_setting(m::ID, key, val);
    let get = |key: &str| game.native_setting(m::ID, key);

    match button {
        0 => {
            let current = m::show_fps(get);
            set("show_fps", if current { "false" } else { "true" });
        }
        1 => {
            let current = m::show_avg_time(get);
            set("show_avg_time", if current { "false" } else { "true" });
        }
        2 => {
            let current = m::show_last_time(get);
            set("show_last_time", if current { "false" } else { "true" });
        }
        3 => {
            let current = m::optimize_ticks(get);
            set("optimize_ticks", if current { "false" } else { "true" });
        }
        4 => {
            let current = m::optimize_frames(get);
            set("optimize_frames", if current { "false" } else { "true" });
        }
        5 => {
            let current = m::target_fps(get);
            let next = match current {
                0 => 60,
                60 => 120,
                120 => 144,
                144 => 240,
                240 => 0,
                _ => 0,
            };
            set("target_fps", &next.to_string());
        }
        6 => {
            let current = m::chunk_lod(get);
            set("chunk_lod", if current { "false" } else { "true" });
        }
        7 => {
            let current = m::chunk_preload(get);
            set("chunk_preload", if current { "false" } else { "true" });
        }
        8 => {
            let current = m::chunk_fast(get);
            set("chunk_fast", if current { "false" } else { "true" });
        }
        _ => {}
    }
}