// Frame hook: called once per frame.
// Records frame timing, computes FPS (smoothed 1/sec).

fn horizon_optimizer_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::horizon_optimizer as m;
    use std::time::Instant;

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

    // Update FPS counter (smoothed - 1 update per second)
    m::FPS_COUNT.with(|count| {
        m::FPS_LAST_UPDATE.with(|last| {
            let now = Instant::now();
            let should_reset = match last.get() {
                Some(t) => now.duration_since(t).as_secs_f64() >= 1.0,
                None => true,
            };
            if should_reset {
                let old_count = count.get();
                m::CURRENT_FPS.set(old_count);
                // Smooth the displayed FPS (80% current, 20% prev)
                m::SMOOTH_FPS.with(|s| {
                    let prev = s.get();
                    let new = ((old_count as f32 * 0.8) + (prev as f32 * 0.2)) as u32;
                    if new == 0 { s.set(old_count.max(1)); } else { s.set(new.max(1)); }
                });
                count.set(0);
                last.set(Some(now));
            }
        });
        count.set(count.get() + 1);
    });

    // Store last frame time (raw, for instant display)
    m::LAST_FRAME_TIME.set(elapsed_us);
}