//! Die quietly on SIGPIPE, like a well-behaved Unix tool.
//!
//! Rust ignores SIGPIPE at startup, which turns a closed pipe
//! (`scaffold list | head -1`) into a panic. Restoring the default
//! handler makes the process exit silently instead. No dependencies:
//! one `signal(2)` call through the system libc.

/// Restore the default SIGPIPE disposition. No-op off Unix.
pub fn restore() {
    #[cfg(unix)]
    unsafe {
        unsafe extern "C" {
            fn signal(signum: i32, handler: usize) -> usize;
        }
        const SIGPIPE: i32 = 13;
        const SIG_DFL: usize = 0;
        signal(SIGPIPE, SIG_DFL);
    }
}
