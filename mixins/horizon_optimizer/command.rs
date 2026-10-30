// Command hook: called when the player types or clicks a command.
// Handles the mod's commands: /optfps, /optstats, /optreset.

fn horizon_optimizer_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::horizon_optimizer as m;

    if !game.native_on(m::ID) {
        return;
    }

    let trimmed = command.trim_start_matches('/');
    let args: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = args.first().copied().unwrap_or("");

    match cmd {
        "optfps" => {
            *ran = true;
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::show_fps(get);
            game.native_set_setting(m::ID, "show_fps", if current { "false" } else { "true" });
            game.native_chat(&format!(
                "§a[Horizon Optimizer] §rFPS overlay {}",
                if !current { "§aenabled" } else { "§cdisabled" }
            ));
        }
        "opttiming" => {
            *ran = true;
            let get = |k: &str| game.native_setting(m::ID, k);
            let current = m::show_timing(get);
            game.native_set_setting(m::ID, "show_timing", if current { "false" } else { "true" });
            game.native_chat(&format!(
                "§a[Horizon Optimizer] §rTiming overlay {}",
                if !current { "§aenabled" } else { "§cdisabled" }
            ));
        }
        "optstats" => {
            *ran = true;
            let (fps, slowest, one_percent) = game.native_frame_rate();
            let ticks = m::TICK_COUNT.with(|c| c.get());
            let avg = m::avg_frame_time();
            game.native_chat(&format!(
                "§a[Horizon Optimizer] §rStats: §e{}§r FPS, §e{}§r 1% low, §e{}§r slowest, §e{}§r ticks, §e{}§r avg frame",
                fps, one_percent, slowest, ticks, m::format_duration(avg)
            ));
        }
        "optreset" => {
            *ran = true;
            game.native_set_setting(m::ID, "show_fps", "true");
            game.native_set_setting(m::ID, "show_timing", "false");
            game.native_set_setting(m::ID, "optimize_ticks", "true");
            game.native_set_setting(m::ID, "optimize_frames", "true");
            game.native_set_setting(m::ID, "target_fps", "0");
            game.native_chat("§a[Horizon Optimizer] §rSettings reset to defaults");
        }
        "opttarget" => {
            *ran = true;
            if args.len() < 2 {
                game.native_chat("§a[Horizon Optimizer] §rUsage: /opttarget <fps> (0 = unlimited)");
                return;
            }
            match args[1].parse::<u32>() {
                Ok(fps) => {
                    game.native_set_setting(m::ID, "target_fps", &fps.to_string());
                    game.native_chat(&format!(
                        "§a[Horizon Optimizer] §rTarget FPS set to {}",
                        if fps == 0 { "unlimited".to_string() } else { fps.to_string() }
                    ));
                }
                Err(_) => {
                    game.native_chat("§a[Horizon Optimizer] §rInvalid FPS value");
                }
            }
        }
        _ => {}
    }
}