pub mod slab;
use std::ops::Index;

use rand::Rng;
pub use slab::Slab;

/// A CDT is a sequence of slabs, where the last slab is connected to the first slab.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct CDT {
    slabs: Vec<Slab>,
}

// display a CDT as a vertical stack of slabs
impl std::fmt::Display for CDT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let reverse_slabs = self.slabs.clone().into_iter().rev();
        for slab in reverse_slabs {
            write!(f, "{}\n", slab)?;
        }
        Ok(())
    }
}

impl CDT {
    pub fn new(slabs: Vec<Slab>) -> CDT {
        CDT { slabs: slabs }
    }

    pub fn new_flat(space_size: usize, time_size: usize) -> CDT {
        assert!(space_size <= 64, "space size must be less than 64");
        assert!(time_size <= 64, "time size must be less than 64");
        let slab_data = Slab::new((0..2 * space_size).map(|x| x % 2 == 0).collect());
        CDT::new((0..time_size).map(|_| slab_data.clone()).collect())
    }

    pub fn volume_profile(&self) -> Vec<usize> {
        let mut result = vec![0; self.slabs.len()];
        for (i, slab) in self.slabs.iter().enumerate() {
            result[i] = slab.length;
        }
        result
    }

    pub fn increase_move(&mut self, time_index: usize, space_index: usize) {
        let (other_time_index, other_space_index) = self.get_pair(time_index, space_index);

        //calculate the delta_t using modular arithmetic
        let delta_t = (other_time_index + self.slabs.len() - time_index) % self.slabs.len();

        //if delta_t is 1 then time_index is the past and other_time_index is the future
        let (past_time_index, past_space_index, future_time_index, future_space_index) =
            if delta_t != 1 {
                (time_index, space_index, other_time_index, other_space_index)
            } else {
                (other_time_index, other_space_index, time_index, space_index)
            };

        //modify the slab at time_index and the corresponding slab at time_index+1
        self.slabs[past_time_index].insert(past_space_index, true);
        self.slabs[future_time_index].insert(future_space_index, false);
    }

    pub fn decrease_move(&mut self, time_index: usize, space_index: usize) {


        let (other_time_index, other_space_index) = self.get_pair(time_index, space_index);



        //remove whatever value is at the past_time_index and past_space_index
        self.slabs[other_time_index].remove(other_space_index);
        self.slabs[time_index].remove(space_index);
    }

    pub fn random_triangle(&self) -> (usize, usize) {
        let total_length = &self.slabs.iter().fold(0, |acc, x| acc + x.length);

        //random number between 0 and total_length
        let index = rand::thread_rng().gen_range(0..*total_length);

        //find the slab that contains the index using iterators
        let mut sum = 0;
        let mut time_index = 0;
        let mut space_index = 0;
        for slab in &self.slabs {
            if sum + slab.length > index {
                space_index = index - sum;
                break;
            }
            sum += slab.length;
            time_index += 1;
        }

        (time_index, space_index)
    }

    //get triangle index (how many other triangles of the same type have already appeared in that slabn) from time and space index
    pub fn get_triangle_index(&self, time_index: usize, space_index: usize) -> usize {
        let slab = &self.slabs[time_index];
        slab.get_triangle_index(space_index)
    }

    pub fn get_pair(&self, time_index: usize, space_index: usize) -> (usize, usize) {
        let slab = &self.slabs[time_index];
        let triangle = slab[space_index];
        let triangle_index = slab.get_triangle_index(space_index);

        let other_time_index = if triangle {
            (time_index + self.slabs.len() - 1) % self.slabs.len()
        } else {
            (time_index + 1) % self.slabs.len()
        };
        let other_slab = &self.slabs[other_time_index];

        let other_space_index = other_slab.get_triangle_in_slab_by_index(triangle_index, !triangle);

        //assert that the other triangle is a differnt type as the original triangle
        assert_ne!(triangle, other_slab[other_space_index]);

        (other_time_index, other_space_index)
    }

    pub fn is_valid(&self) -> bool {
        //check that each past sslab has the same number of ones as its corresponding future slab has zeros
        for (i, slab) in self.slabs.iter().enumerate() {
            let other_slab = &self.slabs[(i + 1) % self.slabs.len()];
            if slab.zeros() != other_slab.ones() {
                return false;
            }
        }
        true
    }
}

//impl index for CDT
impl Index<usize> for CDT {
    type Output = Slab;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slabs[index]
    }
}
