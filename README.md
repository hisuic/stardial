# stardial

A space-anime themed terminal clock — a strict superset of tty-clock, with starfield backgrounds, themes, and gentle effects.

## Quickstart

```bash
cargo run --release
```

Press `q` or `Esc` to quit.

## Features

- Big block-character digits (7-segment style)
- 12/24-hour format, optional seconds, blinking colon
- Date display
- 4 color themes: **void** (cyan), **nebula** (pink), **luna** (silver), **solar** (gold)
- Custom accent color via `--color '#hex'` or named colors
- Animated starfield background with parallax layers
- Shooting star effect (rare, tasteful)
- Scanline overlay for CRT/anime feel
- Deterministic visuals with `--seed`
- Handles terminal resize gracefully
- Panic-safe terminal restoration
- Optional debug logging to file

## Usage

```bash
# Default 24h clock with void theme
stardial

# 12-hour with date
stardial --twelve --date

# Nebula theme, blinking colon, no seconds
stardial --theme nebula --blink --no-seconds

# Custom color, dense stars
stardial --color '#ff6ac1' --stars 80

# Plain clock, no effects
stardial --no-effects

# Deterministic starfield for screenshots
stardial --seed 42 --fps 24

# Debug logging
stardial --log /tmp/stardial.log
```

## Themes

| Theme  | Vibe              | Accent Color |
|--------|-------------------|--------------|
| void   | Deep space cyan   | `#00ffff`    |
| nebula | Purple-pink haze  | `#ff6ac1`    |
| luna   | Silver moonlight  | `#c8d2f0`    |
| solar  | Golden sun        | `#ffc832`    |

## Controls

| Key       | Action |
|-----------|--------|
| `q`/`Esc` | Quit   |
| `Ctrl-C`  | Quit   |

## Performance

- Default 30 FPS with adaptive frame timing
- Starfield uses simple float math, no allocations per frame
- Ratatui's diff-based rendering minimizes terminal writes
- CPU usage typically <1% on modern hardware

## Building

```bash
cargo build --release
```

Binary at `target/release/stardial`.

## Arch Linux (AUR) Packaging

A template PKGBUILD is provided at `pkg/PKGBUILD`. To build:

```bash
cd pkg
makepkg -si
```

### Dependencies

- Runtime: none (static binary)
- Build: `rust` (or `cargo`)

### Generating .SRCINFO

```bash
cd pkg
makepkg --printsrcinfo > .SRCINFO
```

## License

MIT
