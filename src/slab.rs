use itertools::Itertools;

use crate::Direction;
use std::{
    fmt,
    ops::{Deref, DerefMut, Index, Not},
};

/// A slab is a sequence of 1s and 0s, where the 1s represent upwards pointing triangles and the 0s represent downwards pointing triangles.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct Slab {
    pub data: Vec<bool>,
}

impl Slab {
    pub fn new(data: Vec<bool>) -> Slab {
        Slab { data: data }
    }

    pub fn count_true(&self) -> usize {
        self.data.iter().filter(|&b| *b).count()
    }

    pub fn count_false(&self) -> usize {
        self.data.iter().filter(|&b| !*b).count()
    }

    pub fn string(&self) -> String {
        let mut result = String::new();
        for value in &self.data {
            if *value {
                result.push('^');
            } else {
                result.push('v');
            }
        }
        result
    }

    //get triangle index (how many other triangles of the same type have already appeared in that slab) from time and space index
    pub fn get_triangle_index(&self, space_index: usize) -> usize {
        // assert!(space_index < self.length, "index out of bounds");
        //if the triangle is a zero type count the number of zeros in the slab to the left of the triangle using sum_binary_digit_range
        if !self[space_index] {
            (self)[..space_index].iter().filter(|&b| !*b).count()
        } else {
            (self)[..space_index].iter().filter(|&b| *b).count()
        }
    }

    pub fn get_triangle_in_slab_by_index(
        &self,
        triangle_index: usize,
        triangle_type: bool,
    ) -> usize {
        let mut sum = 0;

        //consider folding this using an accumulant pattern

        for i in 0..self.len() {
            if self[i] == triangle_type {
                sum += 1;
            }
            if sum == triangle_index + 1 {
                return i;
            }
        }

        //debug messages
        println!("triangle index: {}", triangle_index);
        println!("triangle type: {}", triangle_type);
        println!("slab: {}", self);
        panic!("triangle index out of bounds");
    }

    pub fn is_boundary(&self, index: usize, side: Direction) -> bool {
        let triangle_index = self.get_triangle_index(index);

        // use triangle index not space index
        match side {
            Direction::Left => triangle_index == 0,
            Direction::Right => triangle_index == self.count_true() - 1,
        }
    }
}

impl Deref for Slab {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// impl iter for &Slab

impl IntoIterator for Slab {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

//same thing but for &Slab
impl<'a> IntoIterator for &'a Slab {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl DerefMut for Slab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
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
        write!(f, "{}", self.string());
        Ok(())
    }
}

pub fn all_slabs(num_trues: usize, num_falses: usize) -> impl Iterator<Item = Slab> {
    let mut data = vec![false; num_falses];
    data.extend(vec![true; num_trues]);
    data.into_iter()
        .permutations(num_trues + num_falses)
        .map(|data| Slab { data })
}
