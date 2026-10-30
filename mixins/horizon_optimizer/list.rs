// Mod list hook: called when Options > Mods lists the native mods.
// Pushes the mod's row with its id, title, description, and keys.

fn horizon_optimizer_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::horizon_optimizer as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![
            ["toggle".into(), "Toggle FPS overlay".into(), "key.keyboard.f6".into()],
            ["timing".into(), "Toggle timing overlay".into(), "key.keyboard.f7".into()],
        ],
    });
}