use ratatui::style::Color;

/// A color theme for stardial.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    /// Primary accent color for digits
    pub accent: Color,
    /// Secondary color for colon / dim elements
    pub secondary: Color,
    /// Date line color
    pub date_color: Color,
    /// Star color (bright)
    pub star_bright: Color,
    /// Star color (dim)
    pub star_dim: Color,
    /// Shooting star color
    pub shooting_star: Color,
    /// Scanline tint
    pub scanline: Color,
    /// Background color (usually near-black)
    pub bg: Color,
}

impl Theme {
    pub fn by_name(name: &str) -> Self {
        match name {
            "nebula" => Self::nebula(),
            "luna" => Self::luna(),
            "solar" => Self::solar(),
            _ => Self::void(),
        }
    }

    /// Deep space — cool cyan/blue on black
    pub fn void() -> Self {
        Self {
            name: "void",
            accent: Color::Rgb(0, 255, 255),
            secondary: Color::Rgb(0, 160, 180),
            date_color: Color::Rgb(80, 120, 140),
            star_bright: Color::Rgb(220, 230, 255),
            star_dim: Color::Rgb(60, 70, 90),
            shooting_star: Color::Rgb(180, 220, 255),
            scanline: Color::Rgb(0, 40, 50),
            bg: Color::Rgb(0, 0, 0),
        }
    }

    /// Purple-pink nebula haze
    pub fn nebula() -> Self {
        Self {
            name: "nebula",
            accent: Color::Rgb(255, 106, 193),
            secondary: Color::Rgb(180, 80, 160),
            date_color: Color::Rgb(140, 80, 130),
            star_bright: Color::Rgb(255, 200, 240),
            star_dim: Color::Rgb(90, 50, 80),
            shooting_star: Color::Rgb(255, 150, 220),
            scanline: Color::Rgb(40, 0, 30),
            bg: Color::Rgb(5, 0, 10),
        }
    }

    /// Soft silver-blue moonlight
    pub fn luna() -> Self {
        Self {
            name: "luna",
            accent: Color::Rgb(200, 210, 240),
            secondary: Color::Rgb(140, 150, 180),
            date_color: Color::Rgb(100, 110, 140),
            star_bright: Color::Rgb(240, 240, 255),
            star_dim: Color::Rgb(50, 55, 70),
            shooting_star: Color::Rgb(210, 220, 255),
            scanline: Color::Rgb(15, 15, 25),
            bg: Color::Rgb(2, 2, 8),
        }
    }

    /// Warm golden sun
    pub fn solar() -> Self {
        Self {
            name: "solar",
            accent: Color::Rgb(255, 200, 50),
            secondary: Color::Rgb(200, 150, 40),
            date_color: Color::Rgb(160, 120, 50),
            star_bright: Color::Rgb(255, 240, 200),
            star_dim: Color::Rgb(80, 70, 40),
            shooting_star: Color::Rgb(255, 220, 100),
            scanline: Color::Rgb(30, 20, 0),
            bg: Color::Rgb(5, 2, 0),
        }
    }

    /// Override the accent color from a user-supplied string.
    pub fn with_accent_override(mut self, color_str: &str) -> Self {
        if let Some(c) = parse_color(color_str) {
            self.accent = c;
        }
        self
    }
}

/// Parse a color from a name or #RRGGBB hex string.
pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    // Hex
    if let Some(hex) = s.strip_prefix('#') {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Color::Rgb(r, g, b));
        }
    }
    // Named colors
    match s.to_lowercase().as_str() {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        "cyan" => Some(Color::Cyan),
        "magenta" => Some(Color::Magenta),
        "yellow" => Some(Color::Yellow),
        "white" => Some(Color::White),
        "black" => Some(Color::Black),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_color("#ff6ac1"), Some(Color::Rgb(255, 106, 193)));
        assert_eq!(parse_color("#000000"), Some(Color::Rgb(0, 0, 0)));
        assert_eq!(parse_color("#ffffff"), Some(Color::Rgb(255, 255, 255)));
    }

    #[test]
    fn test_parse_named_color() {
        assert_eq!(parse_color("red"), Some(Color::Red));
        assert_eq!(parse_color("Cyan"), Some(Color::Cyan));
    }

    #[test]
    fn test_parse_invalid_color() {
        assert_eq!(parse_color("notacolor"), None);
        assert_eq!(parse_color("#xyz"), None);
    }

    #[test]
    fn test_theme_by_name() {
        let t = Theme::by_name("nebula");
        assert_eq!(t.name, "nebula");
        let t = Theme::by_name("unknown");
        assert_eq!(t.name, "void"); // fallback
    }

    #[test]
    fn test_accent_override() {
        let t = Theme::void().with_accent_override("#ff0000");
        assert_eq!(t.accent, Color::Rgb(255, 0, 0));
    }
}
