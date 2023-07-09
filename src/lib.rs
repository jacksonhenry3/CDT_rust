pub mod slab;
use std::ops::Index;

use rand::Rng;
pub use slab::Slab;

pub fn main() {
    let mut example = CDT::new_flat(5, 5);

    // example.increase_move(2, 3, 3, 3);

    let space_index = 8;
    let time_index = 2;

    //print the slab at time_index
    println!("slab at time_index: {}", example.slabs[time_index]);
    //loop over a slab
    // for (index, bit) in example.slabs[time_index].clone().into_iter().enumerate() {

    //     println!("{}: {}", index, bit);

    // }
}

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

    pub fn increase_move(
        &mut self,
        past_time_index: usize,
        past_space_index: usize,
        future_time_index: usize,
        future_space_index: usize,
    ) {
        //modify the slab at time_index and the corresponding slab at time_index+1
        let past_slab = self.slabs[past_time_index].clone();
        let past_slab_left_data = (past_slab.data >> past_space_index) << past_space_index;
        let past_slab_right_data = past_slab.data - past_slab_left_data;
        let new_past_slab_data =
            (past_slab_left_data << 1) + (1 << past_space_index) + past_slab_right_data;
        self.slabs[past_time_index].data = new_past_slab_data;
        self.slabs[past_time_index].length += 1;

        let future_slab = self.slabs[future_time_index].clone();
        let future_slab_left_data = (future_slab.data >> future_space_index) << future_space_index;
        let future_slab_right_data = future_slab.data - future_slab_left_data;
        let new_future_slab_data =
            (future_slab_left_data << 1) + (0 << future_space_index) + future_slab_right_data;
        self.slabs[future_time_index].data = new_future_slab_data;
        self.slabs[future_time_index].length += 1;

        // check that the slabs are not too long
        assert!(
            self.slabs[past_time_index].length < 64,
            "past slab would increase beyond 64"
        );

        assert!(
            self.slabs[future_time_index].length < 64,
            "future slab would increase beyond 64"
        );
    }

    pub fn decrease_move(
        &mut self,
        past_time_index: usize,
        past_space_index: usize,
        future_time_index: usize,
        future_space_index: usize,
    ) {
        //remove whatever value is at the past_time_index and past_space_index
        let past_slab = self.slabs[past_time_index].clone();
        let mut past_slab_left_data = (past_slab.data >> past_space_index) << past_space_index;
        let past_slab_right_data = past_slab.data - past_slab_left_data;
        past_slab_left_data = (past_slab.data >> (past_space_index + 1)) << (past_space_index);
        let new_past_slab_data = (past_slab_left_data) + past_slab_right_data;
        self.slabs[past_time_index].data = new_past_slab_data;
        self.slabs[past_time_index].length -= 1;

        //remove whatever value is at the future_time_index and future_space_index
        let future_slab = self.slabs[future_time_index].clone();
        let mut future_slab_left_data =
            (future_slab.data >> future_space_index) << future_space_index;
        let future_slab_right_data = future_slab.data - future_slab_left_data;
        future_slab_left_data =
            (future_slab.data >> (future_space_index + 1)) << (future_space_index);
        let new_future_slab_data = (future_slab_left_data) + future_slab_right_data;
        self.slabs[future_time_index].data = new_future_slab_data;
        self.slabs[future_time_index].length -= 1;

        //if any slab would decrease in size below 3, panic
        assert!(
            self.slabs[past_time_index].length > 2,
            "past slab would decrease below 3"
        );

        assert!(
            self.slabs[future_time_index].length > 2,
            "future slab would decrease below 3"
        );
    }

    pub fn select_triangle(&self) -> (usize, usize) {
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
            time_index - 1
        } else {
            time_index + 1
        };
        let other_slab = &self.slabs[other_time_index];

        let other_space_index = other_slab.get_triangle_in_slab_by_index(triangle_index, triangle);
        (other_time_index, other_space_index)
    }

    pub fn get_triangle_in_slab_by_index(
        &self,
        time_index: usize,
        triangle_index: usize,
        triangle_type: u64,
    ) -> (usize, usize) {
        let mut sum = 0;
        let mut space_index = 0;
        for i in 0..self.slabs[time_index].length {
            if sum == triangle_index {
                space_index = i;
                break;
            }
            if self.slabs[time_index].data & (1 << i) == 0 {
                sum += 1;
            }
        }
        (time_index, space_index)
    }
}


//impl index for CDT
impl Index<usize> for CDT {
    type Output = Slab;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slabs[index]
    }
}