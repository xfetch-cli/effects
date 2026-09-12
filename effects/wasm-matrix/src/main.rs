//! `wasm-matrix`: a Matrix-style reveal effect compiled to WebAssembly.
//!
//! The content settles from random glyphs into the final text, with an
//! intensity curve that scrambles more at the start. The same logic runs
//! natively (`cargo run`) and as a `wasm32-wasip1` guest; ANSI escape
//! sequences stay intact thanks to `xfetch-effects-lib`.

use std::time::Duration;
use xfetch_effect_api::{EffectFrame, read_effect_request, with_timeout, write_effect_frames};
use xfetch_effects_lib::{lcg, rebuild, tokenize, visible_indices};

/// Frame generation is CPU-bound; keep a generous budget for long durations.
const BUDGET: Duration = Duration::from_secs(10);

/// Glyph alphabet for the rain; blocks and digits read well in any terminal.
const GLYPHS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '#', '%', '&', '*', '+', '=', '█', '▓', '▒',
    '░', '▀', '▄', '▌', '▐',
];

/// Picks a deterministic glyph from the mixer state.
fn glyph(state: &mut u64) -> char {
    GLYPHS[(lcg(state) as usize) % GLYPHS.len()]
}

/// Scrambles one line: each visible character resolves once the progress
/// threshold passes its position, otherwise renders as a random glyph.
fn matrix_line(line: &str, progress: f64, frame: u64, seed: u64) -> String {
    if progress >= 1.0 {
        return line.to_string();
    }

    let segs = tokenize(line);
    let total = visible_indices(&segs).len().max(1) as f64;
    // Reveal a little ahead of the progress curve so text settles smoothly.
    let reveal_threshold = progress * total * 1.1;

    // Mix the frame and the line index so rows resolve independently.
    let mut state = seed
        .wrapping_add(frame.wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add(0x2545_F491_4F6C_DD1D);

    rebuild(&segs, |visible, ch| {
        if (visible as f64) < reveal_threshold {
            ch
        } else {
            glyph(&mut state)
        }
    })
}

fn main() {
    let frames: Vec<EffectFrame> = match with_timeout(BUDGET, || {
        let request = match read_effect_request() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        let duration_ms = request.args.duration_ms.unwrap_or(900).max(1);
        let fps = request.args.fps.unwrap_or(30).max(1);
        let frame_count = ((duration_ms * fps) / 1000).max(1);
        let frame_delay = (1000.0 / fps as f64) as u64;

        let mut frames = Vec::with_capacity(frame_count as usize + 1);
        for frame in 0..=frame_count {
            let progress = frame as f64 / frame_count as f64;
            let lines = request
                .lines
                .iter()
                .enumerate()
                .map(|(index, line)| matrix_line(line, progress, frame, index as u64))
                .collect();
            let delay = if frame == frame_count { 1 } else { frame_delay };
            frames.push(EffectFrame::new(delay, lines));
        }
        frames
    }) {
        Ok(frames) => frames,
        Err(_) => {
            eprintln!("wasm-matrix: timed out generating frames");
            std::process::exit(1);
        }
    };

    if let Err(err) = write_effect_frames(frames) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_frame_settles_to_original() {
        let line = "\u{1b}[32mos\u{1b}[0m: Arch";
        assert_eq!(matrix_line(line, 1.0, 5, 0), line);
    }

    #[test]
    fn escapes_are_preserved() {
        let line = "\u{1b}[32mabcdef\u{1b}[0m";
        for frame in 0..20 {
            let out = matrix_line(line, 0.2, frame, frame);
            assert!(out.contains("\u{1b}[32m"), "frame {frame}");
            assert!(out.contains("\u{1b}[0m"), "frame {frame}");
        }
    }

    #[test]
    fn scramble_uses_the_glyph_alphabet() {
        let out = matrix_line("abcdef", 0.0, 1, 0);
        assert!(out.chars().all(|c| GLYPHS.contains(&c)));
    }
}
