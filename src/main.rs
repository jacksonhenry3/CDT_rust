use rand::Rng;

//macro for printing a digit at a given length
macro_rules! format_digit {
    ($digit:expr, $length:expr) => {
        format!("{:0width$b}", $digit, width = $length).chars().rev().collect::<String>()
    };
}

fn main() {
    let mut example = CDT::new_flat(5, 5);

    // example.increase_move(2, 3, 3, 3);

    let space_index = 8;
    let time_index = 2;

    //print the slab at time_index
    println!("slab at time_index: {}", example.slabs[time_index]);
    //loop over a slab
    for (index, bit) in example.slabs[time_index].clone().into_iter().enumerate() {
        
        println!("{}: {}", index, bit);

        
    }
}

/// A slab is a sequence of 1s and 0s, where the 1s represent upwards pointing triangles and the 0s represent downwards pointing triangles.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
struct Slab {
    length: usize,
    data: u64,
}

impl Slab {
    fn new(data: Vec<bool>) -> Slab {
        Slab {
            length: data.len(),
            data: bool_vec_to_integer(data),
        }
    }

    //setter
    fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.length, "index out of bounds");
        let mask = 1 << index;
        if value {
            self.data |= mask;
        } else {
            self.data &= !mask;
        }
    }

    //getter
    fn get(&self, index: usize) -> bool {
        assert!(index < self.length, "index out of bounds");
        let mask = 1 << index;
        (self.data & mask) == mask
    }

}

//impl intoiter giving mutable refrence to the data for a slab
impl IntoIterator for Slab {
    type Item = bool;
    type IntoIter = SlabIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SlabIntoIterator {
            slab: self,
            index: 0,
        }
    }
}

struct SlabIntoIterator {
    slab: Slab,
    index: usize,
}

impl Iterator for SlabIntoIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slab.length {
            let bit = (self.slab.data >> self.index) & 1;
            self.index += 1;
            Some(bit == 1)
        } else {
            None
        }
    }
}


// display a slab as a string of 1s and 0s
impl std::fmt::Display for Slab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = format_digit!(self.data, self.length);
        write!(f, "{}", v)
    }
}

/// A CDT is a sequence of slabs, where the last slab is connected to the first slab.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
struct CDT {
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
    fn new(slabs: Vec<Slab>) -> CDT {
        CDT { slabs: slabs }
    }

    fn new_flat(space_size: usize, time_size: usize) -> CDT {
        assert!(space_size <= 64, "space size must be less than 64");
        assert!(time_size <= 64, "time size must be less than 64");
        let slab_data = Slab::new((0..2 * space_size).map(|x| x % 2 == 0).collect());
        CDT::new((0..time_size).map(|_| slab_data.clone()).collect())
    }

    fn increase_move(
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

    fn decrease_move(
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

    fn select_triangle(&self) -> (usize,usize) {
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
    fn get_triangle_index(&self, time_index: usize, space_index: usize) -> u64 {

        //print the slab data
        println!("slab data: {}", format_digit!(self.slabs[time_index].data, self.slabs[time_index].length));
        
        //print the triangle type
        println!("triangle type: {}", (self.slabs[time_index].data & (1 << space_index) != 0));

        //print the space index
        println!("space index: {}", space_index);


        
        //if the triangle is a zero type count the number of zeros in the slab to the left of the triangle using sum_binary_digit_range
        if self.slabs[time_index].data & (1 << space_index) == 0 {
            sum_binary_digit_range(!self.slabs[time_index].data, 0, space_index)
        } else {
            sum_binary_digit_range(self.slabs[time_index].data, 0, space_index)
        }
    }

    fn get_triangle_in_slab_by_index(&self, time_index: usize,triangle_index:usize, triangle_type:u64) -> (usize,usize) {
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
        (time_index,space_index)
    }
}

fn bool_vec_to_integer(data: Vec<bool>) -> u64 {
    let mut result = 0;
    let mut pow = 0u32;

    let mut iter = data.iter().rev().peekable();

    while iter.peek().is_some() {
        if let Some(i) = iter.position(|&x| x) {
            pow += i as u32;
            result += 2u64.pow(pow);
            pow += 1;
        }
    }

    result
}

fn sum_binary_digit_range(n: u64, start: usize, end: usize) -> u64 {
    let mut sum = 0;
    for i in start..end {
        sum += (n >> i) & 1;
    }
    sum
}
