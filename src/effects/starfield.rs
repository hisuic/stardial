use fastrand::Rng;
use ratatui::style::Color;

/// A single star with position and parallax layer.
#[derive(Debug, Clone)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    /// Parallax layer: 0 = far (dim, slow), 2 = near (bright, fast)
    pub layer: u8,
    /// Twinkle phase offset
    pub phase: f32,
}

/// Manages the starfield background effect.
#[derive(Debug, Clone)]
pub struct Starfield {
    pub stars: Vec<Star>,
    rng: Rng,
    width: u16,
    height: u16,
}

/// Characters used for stars at different layers.
const STAR_CHARS: [char; 3] = ['·', '∘', '✦'];

impl Starfield {
    /// Create a new starfield filling the given dimensions.
    /// `density` is 0-100 controlling how many stars to generate.
    pub fn new(width: u16, height: u16, density: u32, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => Rng::with_seed(s),
            None => Rng::new(),
        };

        let mut sf = Self {
            stars: Vec::new(),
            rng,
            width,
            height,
        };
        sf.populate(density);
        sf
    }

    /// Populate stars based on density (0-100).
    fn populate(&mut self, density: u32) {
        let area = self.width as u32 * self.height as u32;
        // At density=100, ~1 star per 8 cells; at density=0, no stars
        let count = (area * density) / 800;

        self.stars.clear();
        self.stars.reserve(count as usize);

        for _ in 0..count {
            self.stars.push(Star {
                x: self.rng.f32() * self.width as f32,
                y: self.rng.f32() * self.height as f32,
                layer: (self.rng.u8(0..6) / 2).min(2), // weighted toward far layers
                phase: self.rng.f32() * std::f32::consts::TAU,
            });
        }
    }

    /// Advance the starfield animation by one tick.
    /// `dt` is delta time in seconds.
    pub fn tick(&mut self, dt: f32) {
        let w = self.width as f32;
        for star in &mut self.stars {
            // Parallax speed: far=0.2, mid=0.5, near=1.0 cells/sec
            let speed = match star.layer {
                0 => 0.2,
                1 => 0.5,
                _ => 1.0,
            };
            star.x -= speed * dt;
            if star.x < 0.0 {
                star.x += w;
                star.y = self.rng.f32() * self.height as f32;
            }
            // Advance twinkle phase
            star.phase += dt * 2.0;
            if star.phase > std::f32::consts::TAU {
                star.phase -= std::f32::consts::TAU;
            }
        }
    }

    /// Resize the starfield. Re-populates if dimensions changed.
    pub fn resize(&mut self, width: u16, height: u16, density: u32) {
        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            self.populate(density);
        }
    }

    /// Get the character and brightness for a star.
    pub fn star_char(star: &Star) -> char {
        STAR_CHARS[star.layer as usize]
    }

    /// Compute a star's brightness (0.0-1.0) including twinkle.
    pub fn star_brightness(star: &Star) -> f32 {
        let base = match star.layer {
            0 => 0.3,
            1 => 0.6,
            _ => 0.9,
        };
        // Gentle twinkle: ±20% variation
        let twinkle = star.phase.sin() * 0.2;
        (base + twinkle).clamp(0.0, 1.0)
    }

    /// Interpolate between dim and bright star colors based on brightness.
    pub fn star_color(brightness: f32, bright: Color, dim: Color) -> Color {
        let (br, bg, bb) = color_rgb(bright);
        let (dr, dg, db) = color_rgb(dim);
        let t = brightness;
        Color::Rgb(
            lerp_u8(dr, br, t),
            lerp_u8(dg, bg, t),
            lerp_u8(db, bb, t),
        )
    }
}

fn color_rgb(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (128, 128, 128),
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    let v = a as f32 + (b as f32 - a as f32) * t;
    v.clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_seed() {
        let sf1 = Starfield::new(80, 24, 50, Some(42));
        let sf2 = Starfield::new(80, 24, 50, Some(42));
        assert_eq!(sf1.stars.len(), sf2.stars.len());
        for (a, b) in sf1.stars.iter().zip(sf2.stars.iter()) {
            assert_eq!(a.x, b.x);
            assert_eq!(a.y, b.y);
            assert_eq!(a.layer, b.layer);
        }
    }

    #[test]
    fn test_zero_density_no_stars() {
        let sf = Starfield::new(80, 24, 0, Some(1));
        assert!(sf.stars.is_empty());
    }

    #[test]
    fn test_star_brightness_range() {
        let star = Star {
            x: 0.0,
            y: 0.0,
            layer: 2,
            phase: 0.0,
        };
        let b = Starfield::star_brightness(&star);
        assert!((0.0..=1.0).contains(&b));
    }

    #[test]
    fn test_tick_wraps_stars() {
        let mut sf = Starfield::new(80, 24, 50, Some(42));
        // Tick a lot — stars should wrap, not go negative
        for _ in 0..1000 {
            sf.tick(0.1);
        }
        for star in &sf.stars {
            assert!(star.x >= 0.0, "star x should be non-negative after wrap");
        }
    }

    #[test]
    fn test_resize_repopulates() {
        let mut sf = Starfield::new(80, 24, 50, Some(42));
        let old_count = sf.stars.len();
        sf.resize(160, 48, 50);
        // Larger area should produce more stars
        assert!(sf.stars.len() > old_count);
    }
}
