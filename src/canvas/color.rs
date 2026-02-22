/// Terminal colors for Braille canvas rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermColor {
    /// No explicit color (uses terminal default).
    Default,
    /// Blue (ANSI 34).
    Blue,
    /// Red (ANSI 31).
    Red,
    /// Green (ANSI 32).
    Green,
    /// Yellow (ANSI 33).
    Yellow,
    /// Cyan (ANSI 36).
    Cyan,
    /// Magenta (ANSI 35).
    Magenta,
    /// White (ANSI 37).
    White,
}

/// Default palette for auto color cycling across data series.
pub const PALETTE: [TermColor; 7] = [
    TermColor::Blue,
    TermColor::Red,
    TermColor::Green,
    TermColor::Yellow,
    TermColor::Cyan,
    TermColor::Magenta,
    TermColor::White,
];

impl TermColor {
    /// Parse a color name string into a TermColor.
    ///
    /// Supports: "red", "green", "blue", "yellow", "cyan", "magenta", "white", "default".
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "red" => Some(TermColor::Red),
            "green" => Some(TermColor::Green),
            "blue" => Some(TermColor::Blue),
            "yellow" => Some(TermColor::Yellow),
            "cyan" => Some(TermColor::Cyan),
            "magenta" | "purple" => Some(TermColor::Magenta),
            "white" => Some(TermColor::White),
            "default" | "none" => Some(TermColor::Default),
            _ => None,
        }
    }

    /// Returns the ANSI escape sequence to set this foreground color.
    pub fn ansi_fg(self) -> &'static str {
        match self {
            TermColor::Default => "",
            TermColor::Blue => "\x1b[34m",
            TermColor::Red => "\x1b[31m",
            TermColor::Green => "\x1b[32m",
            TermColor::Yellow => "\x1b[33m",
            TermColor::Cyan => "\x1b[36m",
            TermColor::Magenta => "\x1b[35m",
            TermColor::White => "\x1b[37m",
        }
    }

    /// Returns the ANSI reset escape sequence.
    pub fn ansi_reset() -> &'static str {
        "\x1b[0m"
    }

    fn to_rgb_flags(self) -> u8 {
        match self {
            TermColor::Default => 0b000,
            TermColor::Red => 0b100,
            TermColor::Green => 0b010,
            TermColor::Blue => 0b001,
            TermColor::Yellow => 0b110,
            TermColor::Magenta => 0b101,
            TermColor::Cyan => 0b011,
            TermColor::White => 0b111,
        }
    }

    fn from_rgb_flags(flags: u8) -> Self {
        match flags & 0b111 {
            0b000 => TermColor::Default,
            0b100 => TermColor::Red,
            0b010 => TermColor::Green,
            0b001 => TermColor::Blue,
            0b110 => TermColor::Yellow,
            0b101 => TermColor::Magenta,
            0b011 => TermColor::Cyan,
            _ => TermColor::White,
        }
    }

    /// Additively mixes two colors by OR-ing their 3-bit RGB flags.
    ///
    /// `Default` acts as identity (no color contribution).
    pub fn mix(self, other: TermColor) -> TermColor {
        TermColor::from_rgb_flags(self.to_rgb_flags() | other.to_rgb_flags())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_has_seven_colors() {
        assert_eq!(PALETTE.len(), 7);
    }

    #[test]
    fn default_has_no_escape() {
        assert_eq!(TermColor::Default.ansi_fg(), "");
    }

    #[test]
    fn colors_produce_ansi_codes() {
        assert!(TermColor::Blue.ansi_fg().starts_with("\x1b["));
        assert!(TermColor::Red.ansi_fg().starts_with("\x1b["));
    }

    #[test]
    fn reset_is_nonempty() {
        assert!(!TermColor::ansi_reset().is_empty());
    }

    #[test]
    fn mix_primaries_to_secondaries() {
        assert_eq!(TermColor::Red.mix(TermColor::Blue), TermColor::Magenta);
        assert_eq!(TermColor::Red.mix(TermColor::Green), TermColor::Yellow);
        assert_eq!(TermColor::Green.mix(TermColor::Blue), TermColor::Cyan);
    }

    #[test]
    fn mix_is_commutative() {
        assert_eq!(
            TermColor::Blue.mix(TermColor::Red),
            TermColor::Red.mix(TermColor::Blue)
        );
        assert_eq!(
            TermColor::Green.mix(TermColor::Red),
            TermColor::Red.mix(TermColor::Green)
        );
    }

    #[test]
    fn mix_default_is_identity() {
        for &c in &PALETTE {
            assert_eq!(TermColor::Default.mix(c), c);
            assert_eq!(c.mix(TermColor::Default), c);
        }
    }

    #[test]
    fn mix_self_is_idempotent() {
        for &c in &PALETTE {
            assert_eq!(c.mix(c), c);
        }
    }

    #[test]
    fn mix_white_absorbs_all() {
        for &c in &PALETTE {
            assert_eq!(TermColor::White.mix(c), TermColor::White);
        }
    }

    #[test]
    fn mix_three_primaries_to_white() {
        let mixed = TermColor::Red.mix(TermColor::Green).mix(TermColor::Blue);
        assert_eq!(mixed, TermColor::White);
    }
}
