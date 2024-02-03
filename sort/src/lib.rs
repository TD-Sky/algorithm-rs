#![feature(maybe_uninit_write_slice)]

mod bubble;
mod heap;
mod insertion;
mod merge;
mod quick;
mod selection;
mod shell;

pub use self::bubble::bubble;
pub use self::heap::heap;
pub use self::insertion::insertion;
pub use self::merge::msort;
pub use self::quick::qsort;
pub use self::selection::selection;
pub use self::shell::shell;
