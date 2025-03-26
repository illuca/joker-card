use std::sync::atomic::{ AtomicBool, Ordering };

static EXPLAIN_MODE: AtomicBool = AtomicBool::new(false);

pub fn init(explain: bool) {
    EXPLAIN_MODE.store(explain, Ordering::SeqCst);
}

#[inline]
pub fn explain_enabled() -> bool {
    EXPLAIN_MODE.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! explain {
    ($($arg:tt)*) => {
        if crate::config::explain_enabled() {
            println!($($arg)*);
        }
    };
}
