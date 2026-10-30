// Key hook: called when a key or mouse button goes down or up.
// Reacts to the mod's toggle key.

fn horizon_optimizer_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::horizon_optimizer as m;

    if !game.native_on(m::ID) {
        return;
    }

    // Only react to key presses, not releases
    if !pressed {
        return;
    }

    // Get the configured toggle key (default: F6)
    let toggle_key = game.native_mod_key(m::ID, "toggle", "key.keyboard.f6");

    if key == toggle_key {
        // Toggle the FPS overlay
        let get = |k: &str| game.native_setting(m::ID, k);
        let current = m::show_fps(get);
        game.native_set_setting(m::ID, "show_fps", if current { "false" } else { "true" });
        let new_state = !current;
        game.native_chat(&format!(
            "§a[Horizon Optimizer] §rFPS overlay {}",
            if new_state { "§aenabled" } else { "§cdisabled" }
        ));
    }

    // Toggle timing overlay with F7
    let timing_key = game.native_mod_key(m::ID, "timing", "key.keyboard.f7");
    if key == timing_key {
        let get = |k: &str| game.native_setting(m::ID, k);
        let current = m::show_timing(get);
        game.native_set_setting(m::ID, "show_timing", if current { "false" } else { "true" });
        let new_state = !current;
        game.native_chat(&format!(
            "§a[Horizon Optimizer] §rTiming overlay {}",
            if new_state { "§aenabled" } else { "§cdisabled" }
        ));
    }
}