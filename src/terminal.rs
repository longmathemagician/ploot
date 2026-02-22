/// Terminal size detection (pure Rust, no dependencies).
///
/// Returns `(columns, rows)` or `None` if detection fails.
pub fn terminal_size() -> Option<(usize, usize)> {
    // Try TIOCGWINSZ ioctl on Unix
    #[cfg(unix)]
    {
        use std::mem::MaybeUninit;

        #[repr(C)]
        #[derive(Copy, Clone)]
        struct Winsize {
            ws_row: u16,
            ws_col: u16,
            ws_xpixel: u16,
            ws_ypixel: u16,
        }

        // TIOCGWINSZ value varies by platform
        #[cfg(target_os = "linux")]
        const TIOCGWINSZ: u64 = 0x5413;
        #[cfg(target_os = "macos")]
        const TIOCGWINSZ: u64 = 0x40087468;

        unsafe extern "C" {
            fn ioctl(fd: i32, request: u64, ...) -> i32;
        }

        unsafe {
            let mut ws = MaybeUninit::<Winsize>::uninit();
            // Try stdout (fd 1), then stderr (fd 2)
            for fd in [1i32, 2] {
                let ret = ioctl(fd, TIOCGWINSZ, ws.as_mut_ptr());
                if ret == 0 {
                    let ws = ws.assume_init();
                    if ws.ws_col > 0 && ws.ws_row > 0 {
                        return Some((ws.ws_col as usize, ws.ws_row as usize));
                    }
                }
            }
        }
    }

    // Fallback: check COLUMNS and LINES environment variables
    if let (Ok(cols), Ok(rows)) = (std::env::var("COLUMNS"), std::env::var("LINES"))
        && let (Ok(c), Ok(r)) = (cols.parse::<usize>(), rows.parse::<usize>())
        && c > 0
        && r > 0
    {
        return Some((c, r));
    }

    None
}

/// Returns a reasonable default terminal size (80x24) if detection fails.
pub fn terminal_size_or_default() -> (usize, usize) {
    terminal_size().unwrap_or((80, 24))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_reasonable() {
        let (w, h) = terminal_size_or_default();
        assert!(w >= 20);
        assert!(h >= 5);
    }
}
