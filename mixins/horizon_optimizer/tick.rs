// Tick hook: called once per tick (20 times per second).
// Counts ticks, tracks tick timing, and can apply tick optimizations.

fn horizon_optimizer_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::horizon_optimizer as m;
    use std::time::Instant;

    if !game.native_on(m::ID) {
        return;
    }

    // Increment tick counter
    m::TICK_COUNT.with(|count| {
        count.set(count.get() + 1);
    });

    // Track tick timing
    let now = Instant::now();
    m::LAST_TICK_TIME.set(now.elapsed().as_micros());

    // Tick optimization: if enabled, we could skip some processing
    let get = |key: &str| game.native_setting(m::ID, key);
    if m::optimize_ticks(get) {
        // In a real optimization mod, this is where we'd hook into
        // entity AI, block ticking, etc. The hook points for that
        // would be separate injection points.
    }
}