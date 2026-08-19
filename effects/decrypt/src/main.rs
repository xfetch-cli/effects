//! Decrypt intro effect for xfetch.
//!
//! Each content line is revealed from scrambled glyphs to its real text over
//! `duration_ms` at `fps`. Deterministic; the scramble seed is derived from the
//! frame index. ANSI escape sequences are kept intact via `xfetch-effects-lib`.
//!
//! Protocol: `xfetch-effect-api` (`EffectRequest` in, `EffectResponse` out).

use std::time::Duration;
use xfetch_effect_api::{EffectFrame, read_effect_request, with_timeout, write_effect_frames};
use xfetch_effects_lib::reveal;

/// Frame generation is CPU-bound; the user-controlled duration can demand many
/// frames, so the budget is generous.
const BUDGET: Duration = Duration::from_secs(10);

fn main() {
    let frames: Vec<EffectFrame> = match with_timeout(BUDGET, || {
        let request = match read_effect_request() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        let duration_ms = request.args.duration_ms.unwrap_or(1500).max(1);
        let fps = request.args.fps.unwrap_or(30).max(1);
        let frame_count = ((duration_ms * fps) / 1000).max(1);
        let frame_delay = (1000.0 / fps as f64) as u64;

        let mut frames = Vec::with_capacity(frame_count as usize + 1);
        for i in 0..=frame_count {
            let progress = i as f64 / frame_count as f64;
            let lines: Vec<String> = request
                .lines
                .iter()
                .map(|line| reveal(line, progress, i))
                .collect();
            // Hold the final (revealed) frame briefly before the core takes over.
            let delay = if i == frame_count { 1 } else { frame_delay };
            frames.push(EffectFrame::new(delay, lines));
        }
        frames
    }) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("decrypt: timed out generating frames");
            std::process::exit(1);
        }
    };

    if let Err(err) = write_effect_frames(frames) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
