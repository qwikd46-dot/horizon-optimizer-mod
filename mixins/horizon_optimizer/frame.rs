// Frame hook: called once per frame.
// Records frame timing, computes FPS, and applies frame rate limiting if configured.

fn horizon_optimizer_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::horizon_optimizer as m;
    use std::time::Instant;

    // Only track when the mod is enabled
    if !game.native_on(m::ID) {
        return;
    }

    let elapsed_us = elapsed.as_micros();

    // Store frame time in rolling buffer
    m::FRAME_TIMES.with(|times| {
        let idx = m::FRAME_INDEX.with(|i| {
            let current = i.get();
            let next = (current + 1) % 60;
            i.set(next);
            current
        });
        let mut t = times.get();
        t[idx] = elapsed_us;
        times.set(t);
    });

    // Update FPS counter
    m::FPS_COUNT.with(|count| {
        m::FPS_LAST_UPDATE.with(|last| {
            let now = Instant::now();
            let last_time = last.get();
            let should_reset = match last_time {
                Some(t) => now.duration_since(t).as_secs_f64() >= 1.0,
                None => true,
            };
            if should_reset {
                m::CURRENT_FPS.set(count.get());
                count.set(0);
                last.set(Some(now));
            }
        });
        count.set(count.get() + 1);
    });

    // Store last frame time
    m::LAST_FRAME_TIME.set(elapsed_us);
}