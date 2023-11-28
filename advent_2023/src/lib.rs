mod err_utils;
pub use err_utils::{OptionEmptyError, OptionUtils};

mod file_utils;
pub use file_utils::read_aoc_lines_impl;

mod point;
pub use point::Point;

mod grid;
pub use grid::{Coord, Grid};

// Commonly used containers
pub use std::collections::{HashSet, HashMap};