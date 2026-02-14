/// Scanline effect: subtle horizontal lines that drift slowly.

/// Determine if a given row should have a scanline overlay at a given tick.
/// Returns an opacity value (0.0 = no scanline, up to 0.3 = subtle darkening).
pub fn scanline_opacity(row: u16, tick: f32) -> f32 {
    // Every other row gets a faint scanline
    if row % 2 == 0 {
        return 0.0;
    }
    // Gentle breathing effect
    let phase = (tick * 0.5 + row as f32 * 0.1).sin();
    let opacity = 0.08 + phase.abs() * 0.07;
    opacity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_rows_no_scanline() {
        assert_eq!(scanline_opacity(0, 0.0), 0.0);
        assert_eq!(scanline_opacity(2, 1.0), 0.0);
        assert_eq!(scanline_opacity(100, 5.0), 0.0);
    }

    #[test]
    fn test_odd_rows_have_scanline() {
        let o = scanline_opacity(1, 0.0);
        assert!(o > 0.0 && o < 0.5);
    }
}
