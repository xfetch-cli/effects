//! Shared ANSI-safe text helpers for xfetch effects.
//!
//! Rendered content lines carry ANSI escape sequences (colors, icons). These
//! helpers tokenize a line so escape sequences stay byte-identical and only
//! *visible* characters are manipulated — a partial escape sequence would make
//! the terminal interpret it (e.g. as a device-attributes query) and leak
//! garbage onto the screen.

/// Glyphs used to "encrypt" the not-yet-revealed part of a line.
pub const SCRAMBLE_GLYPHS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789#@%&*$+=";

/// Deterministic position counter (LCG) for scramble output.
pub fn lcg(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state >> 33
}

/// One unit of a rendered line: either a whole ANSI escape sequence (kept
/// verbatim) or a single visible character.
#[derive(Debug)]
pub enum Seg {
    Esc(String),
    Ch(char),
}

/// Splits a rendered line into escape sequences and visible characters.
/// Escape sequences are consumed whole (`ESC [ ... final`), so they can never
/// be broken apart by effects.
pub fn tokenize(line: &str) -> Vec<Seg> {
    let mut segs = Vec::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            let mut esc = String::new();
            esc.push(c);
            if chars.peek() == Some(&'[') {
                esc.push(chars.next().expect("peeked '['"));
                // Consume parameters until the final byte (0x40..=0x7E).
                while let Some(&n) = chars.peek() {
                    esc.push(n);
                    chars.next();
                    if ('\x40'..='\x7e').contains(&n) {
                        break;
                    }
                }
            }
            segs.push(Seg::Esc(esc));
        } else {
            segs.push(Seg::Ch(c));
        }
    }
    segs
}

/// Indexes of the visible (`Ch`) segments, in order.
pub fn visible_indices(segs: &[Seg]) -> Vec<usize> {
    segs.iter()
        .enumerate()
        .filter_map(|(i, s)| matches!(s, Seg::Ch(_)).then_some(i))
        .collect()
}

/// Rebuilds a line from its segments, applying a transform to the visible
/// characters in order. `apply(index_in_visible, char)` returns the char to
/// emit for that position.
pub fn rebuild<F>(segs: &[Seg], mut apply: F) -> String
where
    F: FnMut(usize, char) -> char,
{
    let mut out = String::new();
    let mut seen = 0usize;
    for seg in segs {
        match seg {
            Seg::Esc(esc) => out.push_str(esc),
            Seg::Ch(c) => {
                out.push(apply(seen, *c));
                seen += 1;
            }
        }
    }
    out
}

/// Reveals the *visible* characters of `line` up to `progress` (0.0..=1.0);
/// the rest are scrambled with `seed`. Escape sequences stay unchanged.
pub fn reveal(line: &str, progress: f64, seed: u64) -> String {
    let segs = tokenize(line);
    let indices = visible_indices(&segs);
    let shown = ((indices.len() as f64) * progress).round() as usize;

    let mut state = seed;
    rebuild(&segs, |seen, c| {
        if seen < shown {
            c
        } else {
            let glyph = SCRAMBLE_GLYPHS[(lcg(&mut state) as usize) % SCRAMBLE_GLYPHS.len()];
            glyph as char
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_reveal_returns_original_line() {
        let line = "\u{1b}[32mICON\u{1b}[0m value \u{1b}[1m123\u{1b}[0m";
        assert_eq!(reveal(line, 1.0, 0), line);
    }

    #[test]
    fn tokenize_keeps_escape_sequences_whole() {
        let line = "\u{1b}[38;2;1;2;3mhi\u{1b}[0m!";
        let segs = tokenize(line);
        assert!(matches!(&segs[0], Seg::Esc(e) if e == "\u{1b}[38;2;1;2;3m"));
        assert!(matches!(&segs[1], Seg::Ch('h')));
        assert!(matches!(&segs[2], Seg::Ch('i')));
        assert!(matches!(&segs[3], Seg::Esc(e) if e == "\u{1b}[0m"));
        assert!(matches!(&segs[4], Seg::Ch('!')));
    }

    #[test]
    fn reveal_never_breaks_escapes() {
        let line = "\u{1b}[32mabcdef\u{1b}[0m";
        for seed in 0..20 {
            let out = reveal(line, 0.3, seed);
            assert!(out.contains("\u{1b}[32m"));
            assert!(out.contains("\u{1b}[0m"));
        }
    }

    #[test]
    fn scramble_uses_only_glyph_alphabet() {
        let line = "\u{1b}[31mABCDE\u{1b}[0m";
        let out = reveal(line, 0.0, 7);
        let visible: String = out
            .chars()
            .filter(|c| *c != '\u{1b}' && !"\u{1b}[;m0123456789".contains(*c))
            .collect();
        assert!(
            visible
                .chars()
                .all(|c| SCRAMBLE_GLYPHS.contains(&(c as u8))),
            "scrambled chars must come from the glyph set, got '{visible}'"
        );
    }

    #[test]
    fn rebuild_applies_transform_in_visible_order() {
        let segs = tokenize("\u{1b}[0mA B\u{1b}[0m");
        let out = rebuild(&segs, |_, c| c.to_ascii_lowercase());
        assert_eq!(out, "\u{1b}[0ma b\u{1b}[0m");
    }
}
