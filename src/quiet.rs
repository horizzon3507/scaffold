//! The global `--quiet` switch: non-error output can be silenced.

use std::sync::atomic::{AtomicBool, Ordering};

static QUIET: AtomicBool = AtomicBool::new(false);

/// Set at startup from the `--quiet` flag.
pub fn init(quiet: bool) {
    QUIET.store(quiet, Ordering::Relaxed);
}

/// Whether non-error output should be suppressed.
pub fn is_quiet() -> bool {
    QUIET.load(Ordering::Relaxed)
}

/// Print a line unless quiet mode is on. Errors use `eprintln!` and are
/// never silenced.
#[macro_export]
macro_rules! out {
    ($($arg:tt)*) => {
        if !$crate::quiet::is_quiet() {
            println!($($arg)*);
        }
    };
}
