// Settings click hook: called when a settings button is clicked.
// Toggles the corresponding setting.

fn horizon_optimizer_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::horizon_optimizer as m;

    if mod_id != m::ID {
        return;
    }

    match button {
        0 => {
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::show_fps(get);
            game.native_set_setting(m::ID, "show_fps", if current { "false" } else { "true" });
        }
        1 => {
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::show_timing(get);
            game.native_set_setting(m::ID, "show_timing", if current { "false" } else { "true" });
        }
        2 => {
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::optimize_ticks(get);
            game.native_set_setting(m::ID, "optimize_ticks", if current { "false" } else { "true" });
        }
        3 => {
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::optimize_frames(get);
            game.native_set_setting(m::ID, "optimize_frames", if current { "false" } else { "true" });
        }
        4 => {
            // Cycle through target FPS options
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::target_fps(get);
            let next = match current {
                0 => 60,
                60 => 120,
                120 => 144,
                144 => 240,
                240 => 0,
                _ => 0,
            };
            game.native_set_setting(m::ID, "target_fps", &next.to_string());
        }
        _ => {}
    }
}