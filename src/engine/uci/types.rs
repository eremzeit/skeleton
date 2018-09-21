use std::sync::atomic::{AtomicBool, Ordering};
use std::str::{SplitWhitespace};

pub type Flag = Arc<AtomicBool>;
pub type Params<'a> = SplitWhitespace<'a>;
pub type ClockTime = f64;
pub type ClockIncTime = f64;
