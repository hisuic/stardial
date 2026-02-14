use clap::Parser;

/// stardial — a space-anime themed terminal clock
///
/// A tty-clock superset with starfield backgrounds, themes, and gentle effects.
///
/// Examples:
///   stardial                     # default 24h clock
///   stardial --twelve            # 12-hour format
///   stardial --date --blink      # show date, blink colon
///   stardial --theme nebula      # nebula color theme
///   stardial --no-effects        # plain big clock, no stars
///   stardial --color '#ff6ac1'   # custom accent color
///   stardial --stars 80 --fps 24 # dense stars, 24fps
#[derive(Parser, Debug, Clone)]
#[command(name = "stardial", version, about, long_about = None)]
pub struct Args {
    /// Time format string (chrono syntax)
    #[arg(long, default_value = "%H:%M:%S")]
    pub format: String,

    /// Use 12-hour format (overrides --format)
    #[arg(long, conflicts_with = "twentyfour")]
    pub twelve: bool,

    /// Use 24-hour format (default)
    #[arg(long)]
    pub twentyfour: bool,

    /// Show date line below clock
    #[arg(long)]
    pub date: bool,

    /// Hide seconds from display
    #[arg(long)]
    pub no_seconds: bool,

    /// Blink the colon separator
    #[arg(long)]
    pub blink: bool,

    /// Target frames per second (1-60)
    #[arg(long, default_value_t = 30, value_parser = clap::value_parser!(u32).range(1..=60))]
    pub fps: u32,

    /// Color theme
    #[arg(long, default_value = "void", value_parser = ["void", "nebula", "luna", "solar"])]
    pub theme: String,

    /// Override primary accent color (name or #hex)
    #[arg(long)]
    pub color: Option<String>,

    /// Disable all background effects
    #[arg(long)]
    pub no_effects: bool,

    /// Star density (0-100)
    #[arg(long, default_value_t = 40, value_parser = clap::value_parser!(u32).range(0..=100))]
    pub stars: u32,

    /// Random seed for deterministic visuals
    #[arg(long)]
    pub seed: Option<u64>,

    /// Run a scripted demo loop
    #[arg(long)]
    pub demo: bool,

    /// Write debug logs to file
    #[arg(long, value_name = "FILE")]
    pub log: Option<String>,
}

impl Args {
    /// Resolve the effective time format string based on flags.
    pub fn effective_format(&self) -> String {
        if self.twelve {
            if self.no_seconds {
                "%I:%M %p".to_string()
            } else {
                "%I:%M:%S %p".to_string()
            }
        } else if self.no_seconds {
            "%H:%M".to_string()
        } else {
            self.format.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twelve_hour_format() {
        let args = Args {
            twelve: true,
            no_seconds: false,
            format: "%H:%M:%S".into(),
            ..default_args()
        };
        assert_eq!(args.effective_format(), "%I:%M:%S %p");
    }

    #[test]
    fn test_twelve_no_seconds() {
        let args = Args {
            twelve: true,
            no_seconds: true,
            format: "%H:%M:%S".into(),
            ..default_args()
        };
        assert_eq!(args.effective_format(), "%I:%M %p");
    }

    #[test]
    fn test_twentyfour_no_seconds() {
        let args = Args {
            twelve: false,
            no_seconds: true,
            format: "%H:%M:%S".into(),
            ..default_args()
        };
        assert_eq!(args.effective_format(), "%H:%M");
    }

    #[test]
    fn test_default_format_passthrough() {
        let args = default_args();
        assert_eq!(args.effective_format(), "%H:%M:%S");
    }

    fn default_args() -> Args {
        Args {
            format: "%H:%M:%S".into(),
            twelve: false,
            twentyfour: false,
            date: false,
            no_seconds: false,
            blink: false,
            fps: 30,
            theme: "void".into(),
            color: None,
            no_effects: false,
            stars: 40,
            seed: None,
            demo: false,
            log: None,
        }
    }
}
