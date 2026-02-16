/// Big-digit font for the clock display.
///
/// Each glyph is 5 lines tall and a fixed width (varies per character).
/// Uses Unicode block characters for a clean 7-segment-like look.

/// Height of each glyph in rows.
pub const GLYPH_HEIGHT: usize = 5;

/// Width of a digit glyph in columns.
pub const DIGIT_WIDTH: usize = 5;

/// Width of the colon glyph in columns.
pub const COLON_WIDTH: usize = 1;

/// Width of a space glyph.
pub const SPACE_WIDTH: usize = 3;

/// Returns the glyph lines for a single character.
/// Each line is a string slice of fixed width.
pub fn glyph(ch: char) -> &'static [&'static str] {
    match ch {
        '0' => &[
            "█▀▀▀█",
            "█   █",
            "█   █",
            "█   █",
            "█▄▄▄█",
        ],
        '1' => &[
            "  ▀█ ",
            "   █ ",
            "   █ ",
            "   █ ",
            "   █ ",
        ],
        '2' => &[
            "█▀▀▀█",
            "    █",
            "█▀▀▀█",
            "█    ",
            "█▄▄▄█",
        ],
        '3' => &[
            "█▀▀▀█",
            "    █",
            " ▀▀▀█",
            "    █",
            "█▄▄▄█",
        ],
        '4' => &[
            "█   █",
            "█   █",
            "█▀▀▀█",
            "    █",
            "    █",
        ],
        '5' => &[
            "█▀▀▀█",
            "█    ",
            "█▀▀▀█",
            "    █",
            "█▄▄▄█",
        ],
        '6' => &[
            "█▀▀▀█",
            "█    ",
            "█▀▀▀█",
            "█   █",
            "█▄▄▄█",
        ],
        '7' => &[
            "█▀▀▀█",
            "    █",
            "    █",
            "    █",
            "    █",
        ],
        '8' => &[
            "█▀▀▀█",
            "█   █",
            "█▀▀▀█",
            "█   █",
            "█▄▄▄█",
        ],
        '9' => &[
            "█▀▀▀█",
            "█   █",
            "█▀▀▀█",
            "    █",
            "█▄▄▄█",
        ],
        ':' => &[
            " ",
            "█",
            " ",
            "█",
            " ",
        ],
        ' ' => &[
            "   ",
            "   ",
            "   ",
            "   ",
            "   ",
        ],
        'A' | 'a' => &[
            "█▀▀▀█",
            "█   █",
            "█▀▀▀█",
            "█   █",
            "█   █",
        ],
        'P' | 'p' => &[
            "█▀▀▀█",
            "█   █",
            "█▀▀▀█",
            "█    ",
            "█    ",
        ],
        'M' | 'm' => &[
            "█   █",
            "██ ██",
            "█ █ █",
            "█   █",
            "█   █",
        ],
        _ => &[
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ],
    }
}

/// Compute the display width (in columns) of a character.
pub fn char_width(ch: char) -> usize {
    match ch {
        '0'..='9' | 'A'..='Z' | 'a'..='z' => DIGIT_WIDTH,
        ':' => COLON_WIDTH,
        ' ' => SPACE_WIDTH,
        _ => DIGIT_WIDTH,
    }
}

/// Render a time string into a 2D grid of characters.
/// Returns a Vec of GLYPH_HEIGHT strings, each being the full width of the rendered text.
/// When `hide_colons` is true, colon positions are rendered as blank spaces of the same width,
/// preserving the overall layout.
pub fn render_time_string(s: &str, hide_colons: bool) -> Vec<String> {
    let mut lines: Vec<String> = vec![String::new(); GLYPH_HEIGHT];

    for (i, ch) in s.chars().enumerate() {
        if hide_colons && ch == ':' {
            // Render blank space with the same width as a colon glyph
            for line in lines.iter_mut() {
                for _ in 0..COLON_WIDTH {
                    line.push(' ');
                }
            }
        } else {
            let g = glyph(ch);
            for (row, line) in lines.iter_mut().enumerate() {
                if row < g.len() {
                    line.push_str(g[row]);
                }
            }
        }
        // Add 1-col gap between characters
        if i + 1 < s.len() {
            for line in lines.iter_mut() {
                line.push(' ');
            }
        }
    }

    lines
}

/// Compute total display width for a string.
pub fn rendered_width(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    let char_widths: usize = s.chars().map(char_width).sum();
    let gaps = s.len().saturating_sub(1); // 1-col gap between each pair
    char_widths + gaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_height() {
        for ch in "0123456789: ".chars() {
            let g = glyph(ch);
            assert_eq!(g.len(), GLYPH_HEIGHT, "glyph '{ch}' should have {GLYPH_HEIGHT} lines");
        }
    }

    #[test]
    fn test_render_time_string_height() {
        let lines = render_time_string("12:34", false);
        assert_eq!(lines.len(), GLYPH_HEIGHT);
    }

    #[test]
    fn test_digit_glyph_width_consistency() {
        // Every line of a digit glyph should have the same display width
        for d in '0'..='9' {
            let g = glyph(d);
            for (i, line) in g.iter().enumerate() {
                // Count Unicode grapheme widths (each block char is 1 column wide in our font)
                let w: usize = line.chars().count();
                assert_eq!(
                    w, DIGIT_WIDTH,
                    "digit '{d}' line {i} has width {w}, expected {DIGIT_WIDTH}"
                );
            }
        }
    }

    #[test]
    fn test_rendered_width() {
        // "12:34" = 5 chars, widths: 5+5+1+5+5 = 21, gaps = 4 → 25
        assert_eq!(rendered_width("12:34"), 25);
    }

    #[test]
    fn test_render_specific_digit() {
        let g = glyph('0');
        assert_eq!(g[0], "█▀▀▀█");
        assert_eq!(g[4], "█▄▄▄█");
    }

    #[test]
    fn test_hide_colons_preserves_width() {
        let visible = render_time_string("12:34:56", false);
        let hidden = render_time_string("12:34:56", true);
        for row in 0..GLYPH_HEIGHT {
            assert_eq!(
                visible[row].chars().count(),
                hidden[row].chars().count(),
                "row {row} width differs between visible and hidden colons"
            );
        }
    }
}
