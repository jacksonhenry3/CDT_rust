pub mod slab;
pub mod cdt;
pub mod r#move;

use std::ops::{Index, IndexMut};

use rand::Rng;
use rand::seq::SliceRandom;
pub use slab::Slab;