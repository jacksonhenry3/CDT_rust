use crate::slab::Slab;   
use std::ops::{Index, IndexMut};

use rand::Rng;
use rand::seq::SliceRandom;


/// A CDT is a sequence of slabs, where the last slab is connected to the first slab.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct CDT {
    pub slabs: Vec<Slab>,
}


impl CDT {
    pub fn new(slabs: Vec<Slab>) -> CDT {
        CDT { slabs: slabs }
    }

    pub fn random(volume_profile: Vec<u8>) -> CDT {
        let mut rng = rand::thread_rng();
        let mut slabs = Vec::new();
        for (i,volume) in volume_profile.clone().into_iter().enumerate() {
            
            //create a vec with the correct number of 1s and 0s
            let mut slab_data = vec![true; volume as usize];
            slab_data.append(&mut vec![false; (volume_profile[(i+1)% volume_profile.len()]) as usize]);


            slab_data.shuffle(&mut rng);

            slabs.push(Slab::new(slab_data));
        }
        CDT::new(slabs)
    }

    pub fn new_flat(space_size: usize, time_size: usize) -> CDT {
        assert!(space_size <= 128, "space size must be less than 128");
        assert!(time_size <= 128, "time size must be less than 128");
        let slab_data = Slab::new((0..2 * space_size).map(|x| x % 2 == 0).collect());
        CDT::new((0..time_size).map(|_| slab_data.clone()).collect())
    }

    pub fn volume_profile(&self) -> Vec<(usize, usize)> {
        let mut result = vec![(0, 0); self.slabs.len()];
        for (i, slab) in self.iter().enumerate() {
            result[i] = (slab.ones(), slab.zeros());
        }
        result
    }

    pub fn increase_move(&mut self, time_index: usize, space_index: usize) {
        let (other_time_index, other_space_index) = self.get_temporal_pair(time_index, space_index);

        //calculate the delta_t using modular arithmetic
        let delta_t = (other_time_index + self.slabs.len() - time_index) % self.slabs.len();

        //if delta_t is 1 then time_index is the past and other_time_index is the future
        let (past_time_index, past_space_index, future_time_index, future_space_index) =
            if delta_t != 1 {
                (time_index, space_index, other_time_index, other_space_index)
            } else {
                (other_time_index, other_space_index, time_index, space_index)
            };

        //if either of the slabs are len 128 then return
        if self.slabs[past_time_index].length == 127 || self.slabs[future_time_index].length == 127
        {
            println!("slab length cannot be greater than 127, move aborted");
            return;
        }

        //modify the slab at time_index and the corresponding slab at time_index+1
        self.slabs[past_time_index].insert(past_space_index, true);
        self.slabs[future_time_index].insert(future_space_index, false);
    }

    pub fn decrease_move(&mut self, time_index: usize, space_index: usize) {
        let (other_time_index, other_space_index) = self.get_temporal_pair(time_index, space_index);

        //if either of the slabs have 4 zeros or 4 ones then return
        if self.slabs[time_index].ones() < 4 || self.slabs[other_time_index].ones() < 4 {
            println!("slab length cannot be less than 4, move aborted");
            return;
        }

        if self.slabs[time_index].zeros() < 4 || self.slabs[other_time_index].zeros() < 4 {
            println!("slab length cannot be less than 4, move aborted");
            return;
        }

        //remove whatever value is at the past_time_index and past_space_index
        self.slabs[other_time_index].remove(other_space_index);
        self.slabs[time_index].remove(space_index);
    }

    pub fn parity_move(&mut self, time_index: usize, space_index: usize) {
        let other_space_index = (space_index + 1) % self.slabs[time_index].length;

        assert!(self[time_index][space_index] != self[time_index][other_space_index], "tried to swap {} and {} in slice {} which are both {}", space_index, other_space_index, time_index, self[time_index][space_index]);
        
        let new_value = !self[time_index][space_index];

        self[time_index].set(space_index, new_value);
        self[time_index].set(other_space_index, !new_value);

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

    pub fn get_temporal_pair(&self, time_index: usize, space_index: usize) -> (usize, usize) {
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

    pub fn all_transition_triangles(&self) -> Vec<(usize, usize)> {
        let mut different_triangles = Vec::new();
    
        for (time_index, slab) in self.slabs.iter().enumerate() {
            
            for (i,value) in slab.into_iter().enumerate() {
                if value != slab[(i+1)%slab.length] {
                    different_triangles.push((time_index, i));
                }
            }
        }
    
        different_triangles
    }

    pub fn random_transisition_triangle(&self) -> (usize, usize) {
        let transition_triangles = self.all_transition_triangles();
        //select a random transition triangle
        *transition_triangles.choose(&mut rand::thread_rng()).unwrap()
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

//impl deref
impl std::ops::Deref for CDT {
    type Target = Vec<Slab>;
    fn deref(&self) -> &Self::Target {
        &self.slabs
    }
}

//impl deref mut
impl std::ops::DerefMut for CDT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slabs
    }
}