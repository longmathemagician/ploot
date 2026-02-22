/// Heckbert's "nice number" algorithm.
/// Returns a "nice" number approximately equal to `x`.
/// If `round` is true, rounds to the nearest nice number; otherwise, takes the ceiling.
pub fn nice_number(x: f64, round: bool) -> f64 {
    let exp = x.abs().log10().floor();
    let frac = x / 10.0_f64.powf(exp);

    let nice_frac = if round {
        if frac < 1.5 {
            1.0
        } else if frac < 3.0 {
            2.0
        } else if frac < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if frac <= 1.0 {
        1.0
    } else if frac <= 2.0 {
        2.0
    } else if frac <= 5.0 {
        5.0
    } else {
        10.0
    };

    nice_frac * 10.0_f64.powf(exp)
}

/// A set of computed tick marks for an axis.
pub struct TickSet {
    pub min: f64,
    pub max: f64,
    pub spacing: f64,
    pub ticks: Vec<(f64, String)>,
}

/// Generates nicely-spaced tick marks for the range `[data_min, data_max]`.
///
/// `max_ticks` is the maximum desired number of ticks (typically 5–10).
pub fn generate_ticks(data_min: f64, data_max: f64, max_ticks: usize) -> TickSet {
    if max_ticks < 2 {
        return TickSet {
            min: data_min,
            max: data_max,
            spacing: data_max - data_min,
            ticks: vec![
                (data_min, format_tick(data_min)),
                (data_max, format_tick(data_max)),
            ],
        };
    }

    let range = nice_number(data_max - data_min, true);
    let spacing = nice_number(range / (max_ticks - 1) as f64, true);

    let tick_min = (data_min / spacing).floor() * spacing;
    let tick_max = (data_max / spacing).ceil() * spacing;

    let mut ticks = Vec::new();
    let mut val = tick_min;
    // Guard against infinite loops from floating point issues
    let max_iterations = max_ticks * 3;
    let mut iter = 0;
    while val <= tick_max + spacing * 0.5 * f64::EPSILON && iter < max_iterations {
        ticks.push((val, format_tick(val)));
        val += spacing;
        iter += 1;
    }

    TickSet {
        min: tick_min,
        max: tick_max,
        spacing,
        ticks,
    }
}

/// Formats a tick value, stripping unnecessary trailing zeros.
fn format_tick(val: f64) -> String {
    // Snap very small values to zero
    if val.abs() < 1e-10 {
        return "0".to_string();
    }

    let abs = val.abs();
    if abs >= 1e6 || (abs < 1e-3 && abs > 0.0) {
        // Scientific notation for very large/small values
        let s = format!("{val:.2e}");
        return s;
    }

    // Determine decimal places from magnitude
    let s = if abs >= 100.0 {
        format!("{val:.0}")
    } else if abs >= 1.0 {
        format!("{val:.1}")
    } else {
        format!("{val:.2}")
    };

    // Strip trailing zeros after decimal point
    if s.contains('.') {
        let s = s.trim_end_matches('0');
        let s = s.trim_end_matches('.');
        s.to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_number_known_values() {
        assert!((nice_number(12.0, true) - 10.0).abs() < 1e-9);
        assert!((nice_number(35.0, true) - 50.0).abs() < 1e-9);
        assert!((nice_number(72.0, true) - 100.0).abs() < 1e-9);
        assert!((nice_number(0.23, true) - 0.2).abs() < 1e-9);
    }

    #[test]
    fn ticks_symmetric_range() {
        let ts = generate_ticks(-10.0, 10.0, 5);
        assert!(ts.ticks.len() >= 3);
        // Should include 0
        assert!(ts.ticks.iter().any(|(v, _)| v.abs() < 1e-9));
    }

    #[test]
    fn ticks_small_range() {
        let ts = generate_ticks(0.0, 1.0, 5);
        assert!(ts.ticks.len() >= 3);
        assert!(ts.spacing > 0.0);
    }

    #[test]
    fn ticks_large_range() {
        let ts = generate_ticks(0.0, 1000.0, 6);
        assert!(ts.spacing >= 100.0);
    }

    #[test]
    fn format_strips_trailing_zeros() {
        assert_eq!(format_tick(2.0), "2");
        assert_eq!(format_tick(0.5), "0.5");
        assert_eq!(format_tick(0.0), "0");
    }

    #[test]
    fn format_scientific_for_large() {
        let s = format_tick(1e7);
        assert!(s.contains('e'));
    }

    #[test]
    fn ticks_cover_data_range() {
        let ts = generate_ticks(3.0, 17.0, 5);
        assert!(ts.min <= 3.0);
        assert!(ts.max >= 17.0);
    }
}
