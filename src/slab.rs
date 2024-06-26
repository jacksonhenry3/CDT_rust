use crate::Direction;
use itertools::Itertools;
use std::{
    fmt,
    ops::{Deref, Not},
    sync::Arc,
};

/// Represents a slab, which is a sequence of 1s and 0s.
/// The 1s represent upwards pointing triangles and the 0s represent downwards pointing triangles.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct Slab {
    pub data: Arc<[bool]>,
}

impl Slab {
    /// Creates a new slab with the given data.
    pub fn new(data: Vec<bool>) -> Slab {
        Slab {
            data: Arc::from(data),
        }
    }

    /// Returns the length of the slab.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Returns the count of true values in the slab.
    pub fn count_true(&self) -> usize {
        self.data.iter().filter(|&b| *b).count()
    }

    /// Returns the count of false values in the slab.
    pub fn count_false(&self) -> usize {
        self.data.iter().filter(|&b| !*b).count()
    }

    /// Returns a string representation of the slab.
    /// Upwards pointing triangles are represented by '^' and downwards pointing triangles are represented by 'v'.
    pub fn string(&self) -> String {
        let mut result = String::new();
        for value in self.data.iter() {
            if *value {
                result.push('^');
            } else {
                result.push('v');
            }
        }
        result
    }

    /// Returns the index of the triangle at the given space index.
    pub fn get_triangle_index(&self, space_index: usize) -> usize {
        if self[space_index] {
            (self)[..space_index].iter().filter(|&b| *b).count()
        } else {
            (self)[..space_index].iter().filter(|&b| !*b).count()
        }
    }

    /// Returns the spatial index of the triangle in the slab with the given triangle index and type.
    pub fn get_triangle_in_slab_by_index(
        &self,
        triangle_index: usize,
        triangle_type: bool,
    ) -> usize {
        let mut sum = 0;
        for i in 0..self.len() {
            if self[i] == triangle_type {
                sum += 1;
            }
            if sum == triangle_index + 1 {
                return i;
            }
        }
        println!("triangle index: {}", triangle_index);
        println!("triangle type: {}", triangle_type);
        println!("slab: {}", self);
        panic!("triangle index out of bounds");
    }

    /// Checks if the triangle at the given index is a boundary triangle on the specified side.
    pub fn is_boundary(&self, index: usize, side: Direction) -> bool {
        let triangle_index = self.get_triangle_index(index);
        match side {
            Direction::Left => triangle_index == 0,
            Direction::Right => triangle_index == self.count_true() - 1,
        }
    }
}

impl Deref for Slab {
    type Target = [bool];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Not for Slab {
    type Output = Self;
    fn not(self) -> Self::Output {
        let data = self.data.iter().map(|b| !b).collect();
        Slab { data }
    }
}

impl Not for &Slab {
    type Output = Slab;

    fn not(self) -> Self::Output {
        Slab {
            data: self.data.iter().map(|&b| !b).collect(),
        }
    }
}

impl fmt::Display for Slab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

/// Returns an iterator over all possible slabs with the given number of true and false values.
pub fn all_slabs(num_trues: usize, num_falses: usize) -> impl Iterator<Item = Slab> {
    let mut data = vec![false; num_falses];
    data.extend(vec![true; num_trues]);
    data.into_iter()
        .permutations(num_trues + num_falses)
        .unique()
        .map(|data| Slab {
            data: Arc::from(data),
        })
}
