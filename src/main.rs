mod app;
mod cli;
mod effects;
mod font;
mod render;
mod theme;
mod util;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use cli::Args;

fn main() -> Result<()> {
    // Install panic hook that restores terminal before printing backtrace.
    color_eyre::install()?;

    let args = Args::parse();

    // Optional file logging (keep guard alive for duration of program).
    let _log_guard = if let Some(ref log_path) = args.log {
        Some(util::init_logging(log_path)?)
    } else {
        None
    };

    tracing::info!("stardial starting with args: {:?}", args);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Ensure terminal is restored on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let size = terminal.size()?;
    let mut app = App::new(&args, size.width, size.height);

    let result = run_loop(&mut terminal, &mut app);

    // Restore terminal (always, even on error)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    tracing::info!("stardial exiting");
    result
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let frame_duration = Duration::from_secs_f64(1.0 / app.fps as f64);
    let mut last_frame = Instant::now();

    loop {
        if app.should_quit {
            return Ok(());
        }

        // Compute dt
        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        // Update state
        app.tick(dt);

        // Render
        terminal.draw(|frame| {
            render::draw(frame, app);
        })?;

        // Handle events with timeout to maintain fps
        let timeout = frame_duration.saturating_sub(now.elapsed());
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('c')
                        if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) =>
                    {
                        app.should_quit = true;
                    }
                    _ => {}
                },
                Event::Resize(w, h) => {
                    app.resize(w, h);
                }
                _ => {}
            }
        }
    }
}
