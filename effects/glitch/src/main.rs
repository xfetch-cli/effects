//! Glitch intro effect for xfetch.
//!
//! The content stutters through scrambled glyphs with random "corruption
//! bursts" (frames where the whole line collapses) before settling on the real
//! text. Deterministic; ANSI escape sequences stay intact via
//! `xfetch-effects-lib`.
//!
//! Protocol: `xfetch-effect-api` (`EffectRequest` in, `EffectResponse` out).

use std::time::Duration;
use xfetch_effect_api::{EffectFrame, read_effect_request, with_timeout, write_effect_frames};
use xfetch_effects_lib::{lcg, rebuild, tokenize, visible_indices};

/// Frame generation is CPU-bound; the user-controlled duration can demand many
/// frames, so the budget is generous.
const BUDGET: Duration = Duration::from_secs(10);

/// Scramble alphabet: alphanumerics + symbols + ASCII block elements, so the
/// corruption looks like a broken terminal instead of plain text.
const GLYPHS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '#', '@',
    '%', '&', '*', '$', '+', '=', '█', '▓', '▒', '░', '▀', '▄', '▌', '▐', '╬', '╪', '╫',
];

fn glitch_char(state: &mut u64) -> char {
    GLYPHS[(lcg(state) as usize) % GLYPHS.len()]
}

/// Mixes the frame index into the scramble seed so each frame's noise differs.
fn mix(state: &mut u64, frame: u64) {
    *state ^= frame.wrapping_mul(0x9E3779B97F4A7C15);
    *state = state.wrapping_add(0x2545F4914F6CDD1D);
}

/// One line of glitch. Until the very end, the visible characters flicker
/// wildly: `shown` bounces within a progress-shrinking cap, whole lines
/// collapse to full scramble on "burst" frames, already-revealed characters
/// occasionally flip back to a random glyph, and the line is sometimes
/// displaced horizontally ("slice") so rows stop aligning. The final frame
/// (`progress >= 1.0`) returns the line unchanged.
///
/// `line_seed` differs per line so each row corrupts independently instead of
/// showing the same noise.
fn glitch_line(line: &str, progress: f64, frame: u64, line_seed: u64) -> String {
    if progress >= 1.0 {
        return line.to_string();
    }
    let segs = tokenize(line);
    let indices = visible_indices(&segs);
    let n = indices.len();
    if n == 0 {
        return line.to_string();
    }

    let mut state = frame;
    mix(&mut state, frame);
    mix(&mut state, line_seed);

    // Progress cap shrinks to zero as we approach the end; `shown` bounces
    // anywhere inside it, so earlier frames are dominated by random noise.
    let cap = (n as f64 * progress).round() as usize;
    // Aggressive: bursts (full-line collapse) hit roughly every 3 frames.
    let burst = lcg(&mut state).is_multiple_of(3);
    let shown = if burst {
        0
    } else {
        lcg(&mut state) as usize % (cap + 1)
    };

    let base = rebuild(&segs, |seen, c| {
        if seen < shown && !lcg(&mut state).is_multiple_of(5) {
            c
        } else {
            glitch_char(&mut state)
        }
    });

    // Horizontal "slice": occasionally push the whole line right a few columns
    // so the rows no longer align — the classic displaced-lines glitch look.
    let slice = progress < 0.95 && lcg(&mut state).is_multiple_of(4);
    if slice {
        let offset = (lcg(&mut state) as usize % 5) + 1;
        format!("{}{}", " ".repeat(offset), base)
    } else {
        base
    }
}

/// Full frame: like `glitch_line` per line, plus a "corrupted row" — a random
/// line is dropped (renders blank) on burst frames.
fn glitch_frame(lines: &[String], progress: f64, frame: u64) -> Vec<String> {
    let mut out: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| glitch_line(line, progress, frame, i as u64))
        .collect();
    if !out.is_empty() && progress < 0.9 {
        let mut state = frame;
        mix(&mut state, frame);
        if lcg(&mut state).is_multiple_of(6) {
            let idx = (lcg(&mut state) as usize) % out.len();
            out[idx] = String::new();
        }
    }
    out
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

        let duration_ms = request.args.duration_ms.unwrap_or(800).max(1);
        let fps = request.args.fps.unwrap_or(30).max(1);
        let frame_count = ((duration_ms * fps) / 1000).max(1);
        let frame_delay = (1000.0 / fps as f64) as u64;

        let mut frames = Vec::with_capacity(frame_count as usize + 1);
        for i in 0..=frame_count {
            let progress = i as f64 / frame_count as f64;
            let lines = glitch_frame(&request.lines, progress, i);
            // Hold the final (settled) frame briefly before the core takes over.
            let delay = if i == frame_count { 1 } else { frame_delay };
            frames.push(EffectFrame::new(delay, lines));
        }
        frames
    }) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("glitch: timed out generating frames");
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
        let line = "\u{1b}[32mICON\u{1b}[0m value 123";
        assert_eq!(glitch_line(line, 1.0, 5, 0), line);
    }

    #[test]
    fn escapes_are_preserved() {
        let line = "\u{1b}[32mabcdef\u{1b}[0m";
        for frame in 0..30 {
            let out = glitch_line(line, 0.3, frame, frame);
            assert!(out.contains("\u{1b}[32m"), "frame {frame}");
            assert!(out.contains("\u{1b}[0m"), "frame {frame}");
        }
    }

    #[test]
    fn scramble_uses_only_glyph_alphabet() {
        let line = "\u{1b}[31mABCDE\u{1b}[0m";
        let out = glitch_line(line, 0.0, 1, 0);
        // Leading spaces are the horizontal "slice" displacement.
        let out = out.trim_start_matches(' ');
        let visible: String = out
            .chars()
            .filter(|c| *c != '\u{1b}' && !"\u{1b}[;m0123456789".contains(*c))
            .collect();
        assert!(
            visible.chars().all(|c| GLYPHS.contains(&c)),
            "scrambled chars must come from the glyph set, got '{visible}'"
        );
    }

    #[test]
    fn glyph_set_includes_ascii_blocks() {
        for block in ['█', '▓', '▒', '░', '▀', '▄'] {
            assert!(GLYPHS.contains(&block), "missing block '{block}'");
        }
    }
}
