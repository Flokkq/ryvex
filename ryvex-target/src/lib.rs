extern crate alloc;

pub mod key;
pub mod std;
pub mod term;

mod macros;

pub mod r#impl;
pub use r#impl as target;
