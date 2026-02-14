use fastrand::Rng;

/// A shooting star that streaks across the screen.
#[derive(Debug, Clone)]
pub struct ShootingStar {
    /// Current head position
    pub x: f32,
    pub y: f32,
    /// Velocity (cells/sec)
    pub vx: f32,
    pub vy: f32,
    /// Trail length in cells
    pub trail_len: u8,
    /// Remaining lifetime in seconds
    pub life: f32,
}

/// Manager for occasional shooting star effects.
#[derive(Debug, Clone)]
pub struct ShootingStarManager {
    pub active: Vec<ShootingStar>,
    cooldown: f32,
    rng: Rng,
    width: u16,
    height: u16,
}

/// Characters for the shooting star trail (head → tail).
pub const TRAIL_CHARS: [char; 4] = ['━', '─', '╌', '·'];

impl ShootingStarManager {
    pub fn new(width: u16, height: u16, seed: Option<u64>) -> Self {
        let mut rng = match seed {
            Some(s) => Rng::with_seed(s.wrapping_add(0xDEAD)),
            None => Rng::new(),
        };
        Self {
            active: Vec::new(),
            cooldown: 3.0 + rng.f32() * 5.0, // first one after 3-8 seconds
            rng,
            width,
            height,
        }
    }

    /// Advance the shooting star animation.
    pub fn tick(&mut self, dt: f32) {
        // Update existing
        self.active.retain_mut(|s| {
            s.x += s.vx * dt;
            s.y += s.vy * dt;
            s.life -= dt;
            s.life > 0.0
                && s.x > -(s.trail_len as f32)
                && s.x < self.width as f32 + 10.0
                && s.y > -2.0
                && s.y < self.height as f32 + 2.0
        });

        // Spawn new
        self.cooldown -= dt;
        if self.cooldown <= 0.0 {
            self.spawn();
            // Next one in 5-15 seconds (rare)
            self.cooldown = 5.0 + self.rng.f32() * 10.0;
        }
    }

    fn spawn(&mut self) {
        let w = self.width as f32;
        let h = self.height as f32;
        // Start from right side, streak left-ish
        let x = w * 0.6 + self.rng.f32() * w * 0.4;
        let y = self.rng.f32() * h * 0.5; // upper half
        let speed = 20.0 + self.rng.f32() * 30.0;
        let angle = 0.1 + self.rng.f32() * 0.3; // slight downward angle
        self.active.push(ShootingStar {
            x,
            y,
            vx: -speed,
            vy: speed * angle,
            trail_len: 4 + self.rng.u8(0..4),
            life: 1.5 + self.rng.f32(),
        });
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Get trail positions for rendering (from head to tail).
    pub fn trail_positions(star: &ShootingStar) -> Vec<(u16, u16, char)> {
        let mut positions = Vec::new();
        let dx = if star.vx.abs() > 0.01 {
            -star.vx / star.vx.abs()
        } else {
            1.0
        };
        let dy = if star.vy.abs() > 0.01 {
            -star.vy / star.vy.abs()
        } else {
            0.0
        };

        for i in 0..star.trail_len {
            let tx = star.x + dx * i as f32;
            let ty = star.y + dy * i as f32 * 0.3; // compressed vertical
            if tx >= 0.0 && ty >= 0.0 {
                let ch = TRAIL_CHARS[(i as usize).min(TRAIL_CHARS.len() - 1)];
                positions.push((tx as u16, ty as u16, ch));
            }
        }
        positions
    }
}
