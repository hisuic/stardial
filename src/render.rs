use chrono::Local;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
    Frame,
};

use crate::app::App;
use crate::effects::scanline;
use crate::effects::shooting_star::ShootingStarManager;
use crate::effects::starfield::Starfield;
use crate::font;

/// Render the full frame.
pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let clock_widget = ClockWidget { app };
    frame.render_widget(clock_widget, area);
}

struct ClockWidget<'a> {
    app: &'a App,
}

impl<'a> Widget for ClockWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app = self.app;

        // Clear background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(' ');
                    cell.set_style(Style::default().bg(app.theme.bg));
                }
            }
        }

        // Render starfield
        if app.effects_enabled {
            render_starfield(buf, area, app);
            render_shooting_stars(buf, area, app);
        }

        // Render big clock digits
        let now = Local::now();
        let time_str = now.format(&app.time_format).to_string();

        let hide_colons = app.blink && !app.blink_visible;
        let lines = font::render_time_string(&time_str, hide_colons);
        let text_width = font::rendered_width(&time_str) as u16;
        let text_height = font::GLYPH_HEIGHT as u16;

        // Center the clock, slightly above center
        let cx = area.x + area.width.saturating_sub(text_width) / 2;
        let cy = area.y + area.height.saturating_sub(text_height + 2) / 2;

        // Draw digit glyphs
        for (row, line) in lines.iter().enumerate() {
            let y = cy + row as u16;
            if y >= area.y + area.height {
                break;
            }
            for (col, ch) in line.chars().enumerate() {
                let x = cx + col as u16;
                if x >= area.x + area.width {
                    break;
                }
                if ch != ' ' {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_char(ch);
                        let color = glyph_color(ch, &app.theme);
                        cell.set_style(Style::default().fg(color).bg(app.theme.bg));
                    }
                }
            }
        }

        // Render date line below clock
        if app.show_date {
            let date_str = now.format("%Y-%m-%d %A").to_string();
            let date_x = area.x + area.width.saturating_sub(date_str.len() as u16) / 2;
            let date_y = cy + text_height + 1;
            if date_y < area.y + area.height {
                for (i, ch) in date_str.chars().enumerate() {
                    let x = date_x + i as u16;
                    if x < area.x + area.width {
                        if let Some(cell) = buf.cell_mut((x, date_y)) {
                            cell.set_char(ch);
                            cell.set_style(
                                Style::default().fg(app.theme.date_color).bg(app.theme.bg),
                            );
                        }
                    }
                }
            }
        }

        // Scanline overlay (last pass)
        if app.effects_enabled {
            render_scanlines(buf, area, app);
        }
    }
}

/// Map glyph characters to appropriate colors.
fn glyph_color(ch: char, theme: &crate::theme::Theme) -> Color {
    match ch {
        '█' => theme.accent,
        '▀' | '▄' => theme.secondary,
        _ => theme.accent,
    }
}

fn render_starfield(buf: &mut Buffer, area: Rect, app: &App) {
    for star in &app.starfield.stars {
        let x = star.x as u16;
        let y = star.y as u16;
        if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
            let brightness = Starfield::star_brightness(star);
            let color =
                Starfield::star_color(brightness, app.theme.star_bright, app.theme.star_dim);
            let ch = Starfield::star_char(star);
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char(ch);
                cell.set_style(Style::default().fg(color).bg(app.theme.bg));
            }
        }
    }
}

fn render_shooting_stars(buf: &mut Buffer, area: Rect, app: &App) {
    for star in &app.shooting_stars.active {
        let positions = ShootingStarManager::trail_positions(star);
        for (i, (x, y, ch)) in positions.iter().enumerate() {
            if *x >= area.x
                && *x < area.x + area.width
                && *y >= area.y
                && *y < area.y + area.height
            {
                // Fade trail: head is bright, tail is dim
                let fade = 1.0 - (i as f32 / positions.len() as f32);
                let (r, g, b) = match app.theme.shooting_star {
                    Color::Rgb(r, g, b) => (r, g, b),
                    _ => (200, 220, 255),
                };
                let color = Color::Rgb(
                    (r as f32 * fade) as u8,
                    (g as f32 * fade) as u8,
                    (b as f32 * fade) as u8,
                );
                if let Some(cell) = buf.cell_mut((*x, *y)) {
                    cell.set_char(*ch);
                    cell.set_style(Style::default().fg(color).bg(app.theme.bg));
                }
            }
        }
    }
}

fn render_scanlines(buf: &mut Buffer, area: Rect, app: &App) {
    for y in area.y..area.y + area.height {
        let opacity = scanline::scanline_opacity(y, app.elapsed);
        if opacity > 0.01 {
            let (sr, sg, sb) = match app.theme.scanline {
                Color::Rgb(r, g, b) => (r, g, b),
                _ => (0, 0, 0),
            };
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    // Darken the existing bg slightly toward scanline color
                    if let Color::Rgb(br, bg_c, bb) = cell.bg {
                        let nr = lerp(br, sr, opacity);
                        let ng = lerp(bg_c, sg, opacity);
                        let nb = lerp(bb, sb, opacity);
                        cell.set_style(cell.style().bg(Color::Rgb(nr, ng, nb)));
                    }
                }
            }
        }
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let v = a as f32 + (b as f32 - a as f32) * t;
    v.clamp(0.0, 255.0) as u8
}

/// Render to a string buffer for headless/snapshot testing.
#[cfg(test)]
pub fn render_to_string(app: &App, width: u16, height: u16) -> String {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            draw(frame, app);
        })
        .unwrap();

    let buf = terminal.backend().buffer().clone();
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            let cell = &buf[(x, y)];
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Args;

    fn test_args() -> Args {
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
            no_effects: true,
            stars: 0,
            seed: Some(42),
            demo: false,
            log: None,
        }
    }

    #[test]
    fn test_headless_render_contains_digits() {
        let args = test_args();
        let app = App::new(&args, 80, 24);
        let output = render_to_string(&app, 80, 24);
        // Should contain block characters (digits are rendered)
        assert!(
            output.contains('█'),
            "rendered output should contain block chars"
        );
    }

    #[test]
    fn test_headless_render_with_date() {
        let mut args = test_args();
        args.date = true;
        let app = App::new(&args, 80, 24);
        let output = render_to_string(&app, 80, 24);
        assert!(output.contains('█'));
    }

    #[test]
    fn test_render_small_terminal() {
        // Should not panic even with tiny terminal
        let args = test_args();
        let app = App::new(&args, 20, 5);
        let _output = render_to_string(&app, 20, 5);
    }
}
