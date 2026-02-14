use crate::cli::Args;
use crate::effects::shooting_star::ShootingStarManager;
use crate::effects::starfield::Starfield;
use crate::theme::Theme;

/// Application state.
pub struct App {
    pub theme: Theme,
    pub time_format: String,
    pub show_date: bool,
    pub blink: bool,
    pub blink_visible: bool,
    pub effects_enabled: bool,
    pub starfield: Starfield,
    pub shooting_stars: ShootingStarManager,
    pub fps: u32,
    pub star_density: u32,
    pub tick_count: u64,
    pub elapsed: f32,
    pub demo_mode: bool,
    pub should_quit: bool,
    pub width: u16,
    pub height: u16,
}

impl App {
    /// Create from CLI args and initial terminal size.
    pub fn new(args: &Args, width: u16, height: u16) -> Self {
        let mut theme = Theme::by_name(&args.theme);
        if let Some(ref c) = args.color {
            theme = theme.with_accent_override(c);
        }

        let time_format = args.effective_format();

        Self {
            theme,
            time_format,
            show_date: args.date,
            blink: args.blink,
            blink_visible: true,
            effects_enabled: !args.no_effects,
            starfield: Starfield::new(width, height, args.stars, args.seed),
            shooting_stars: ShootingStarManager::new(width, height, args.seed),
            fps: args.fps,
            star_density: args.stars,
            tick_count: 0,
            elapsed: 0.0,
            demo_mode: args.demo,
            should_quit: false,
            width,
            height,
        }
    }

    /// Advance one frame. `dt` = seconds since last frame.
    pub fn tick(&mut self, dt: f32) {
        self.tick_count += 1;
        self.elapsed += dt;

        // Blink colon every 0.5s
        if self.blink {
            self.blink_visible = (self.elapsed * 2.0) as u64 % 2 == 0;
        }

        if self.effects_enabled {
            self.starfield.tick(dt);
            self.shooting_stars.tick(dt);
        }
    }

    /// Handle terminal resize.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.starfield.resize(width, height, self.star_density);
        self.shooting_stars.resize(width, height);
    }
}
