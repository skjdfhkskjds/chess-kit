mod map;
mod stack;

pub use map::{HashFn, HashKey, Map, EvictionPolicy, Value, ValuePriority};
pub use stack::{Copyable, Stack};
