# stardial

A space-anime themed terminal clock — a strict superset of tty-clock, with starfield backgrounds, themes, and gentle effects.

## Quickstart

```bash
cargo run --release
```

Press `q` or `Esc` to quit.

## Features

- Big block-character digits (7-segment style) with adjustable size (`--size 1`-`5`)
- 12/24-hour format, optional seconds, blinking colon
- Custom time format via `--format` (chrono syntax)
- Date display
- 4 color themes: **void** (cyan), **nebula** (pink), **luna** (silver), **solar** (gold)
- Custom accent color via `--color '#hex'` or named colors
- Animated starfield background with parallax layers
- Shooting star effect (rare, tasteful)
- Scanline overlay for CRT/anime feel
- Deterministic visuals with `--seed`
- Scripted demo loop with `--demo`
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

# Larger clock face (2x)
stardial --size 2

# Custom time format (chrono syntax)
stardial --format '%H:%M'

# Plain clock, no effects
stardial --no-effects

# Deterministic starfield for screenshots
stardial --seed 42 --fps 24

# Run scripted demo loop
stardial --demo

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

## Installation

### From source

```bash
make
sudo make install
```

This installs:
- `/usr/bin/stardial`
- `/usr/share/man/man1/stardial.1`
- `/usr/share/licenses/stardial/LICENSE`

To customize the prefix:

```bash
make install PREFIX=/usr/local DESTDIR=/tmp/pkg
```

To uninstall:

```bash
sudo make uninstall
```

### Arch Linux (AUR)

```bash
cd pkg
makepkg -si
```

#### Clean chroot build (recommended for verification)

```bash
# Install devtools if not present
sudo pacman -S devtools

# Build in a clean chroot
cd pkg
extra-x86_64-build
```

#### Verify with namcap

```bash
cd pkg
namcap PKGBUILD
namcap stardial-*.pkg.tar.zst
```

### Dependencies

- Runtime: `gcc-libs` (glibc/libgcc — dynamically linked)
- Build: `cargo`

## Man page

```bash
man stardial
```

## Releases

Tagged releases follow semver: `v0.1.0`, `v0.2.0`, etc.

## License

MIT — see [LICENSE](LICENSE).

## Links

- Repository: <https://github.com/hisuic/stardial>
