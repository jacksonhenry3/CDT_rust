//macro for printing a digit at a given length
macro_rules! format_digit {
    ($digit:expr, $length:expr) => {
        format!("{:0width$b}", $digit, width = $length)
    };
}

/// A slab is a sequence of 1s and 0s, where the 1s represent upwards pointing triangles and the 0s represent downwards pointing triangles.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct Slab {
    pub length: usize,
    pub data: u128,
}

impl Slab {
    pub fn new(data: Vec<bool>) -> Slab {
        Slab {
            length: data.len(),
            data: bool_vec_to_integer(data),
        }
    }

    pub fn string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.length {
            result.push(if self[i] { '1' } else { '0' });
        }
        result
    }

    pub fn set(&mut self, index: usize, value: bool) {
        // assert!(index < self.length, "index out of bounds");
        let mask = 1 << index;
        if value {
            self.data |= mask;
        } else {
            self.data &= !mask;
        }
    }

    //get triangle index (how many other triangles of the same type have already appeared in that slabn) from time and space index
    pub fn get_triangle_index(&self, space_index: usize) -> usize {
        // assert!(space_index < self.length, "index out of bounds");
        //if the triangle is a zero type count the number of zeros in the slab to the left of the triangle using sum_binary_digit_range
        if !self[space_index] {
            sum_binary_digit_range(!self.data, 0, space_index)
        } else {
            sum_binary_digit_range(self.data, 0, space_index)
        }
    }

    pub fn get_triangle_in_slab_by_index(
        &self,
        triangle_index: usize,
        triangle_type: bool,
    ) -> usize {
        let mut sum = 0;

        //consider folding this using an accumulant pattern

        for i in 0..self.length {
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

    pub fn insert(&mut self, index: usize, value: bool) {
        // assert!(index < self.length, "index out of bounds");
        // insert a 1 or 0 at a given index shifting values to either side
        let slab_left_data = (self.data >> index) << index;
        let slab_right_data = self.data - slab_left_data;
        let new_slab_data = (slab_left_data << 1) + ((value as u128) << index) + slab_right_data;
        self.data = new_slab_data;
        self.length += 1;

        assert!(self.length < 128, "slab length cannot be greater than 128")
    }

    pub fn remove(&mut self, index: usize) {
        // assert!(index < self.length, "index out of bounds");
        // remove a 1 or 0 at a given index shifting values to either side
        let removed_value = !(1 << index) & self.data;
        let left_data = (removed_value >> index) << index;
        let right_data = removed_value - left_data;
        let new_slab_data = (left_data >> 1) + right_data;
        self.data = new_slab_data;
        self.length -= 1;

        //this is wrong, its not slab length its spatial slice length (3 ze5ros and 3 ones)
        assert!(self.length >= 3, "slab length cannot be less than 3")
    }

    pub fn ones(&self) -> usize {
        sum_binary_digit_range(self.data, 0, self.length)
    }

    pub fn zeros(&self) -> usize {
        sum_binary_digit_range(!self.data, 0, self.length)
    }

    pub fn to_vec(&self) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..self.length {
            result.push(self[i]);
        }
        result
    }
}

//impl Index
impl std::ops::Index<usize> for Slab {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let mask = 1 << index;
        if (self.data & mask) == mask {
            &true
        } else {
            &false
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

pub fn bool_vec_to_integer(data: Vec<bool>) -> u128 {
    let mut result = 0;
    let mut pow = 0u32;

    let mut iter = data.iter().rev().peekable();

    while iter.peek().is_some() {
        if let Some(i) = iter.position(|&x| x) {
            pow += i as u32;
            result += 2u128.pow(pow);
            pow += 1;
        }
    }

    result
}

pub fn sum_binary_digit_range(n: u128, start: usize, end: usize) -> usize {
    let mut sum = 0;
    for i in start..end {
        sum += (n >> i) & 1;
    }
    sum as usize
}

//impl iter using to_vec
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

/// Generates all possible slabs (combinations of ones and zeros) of a certain length.
///
/// This function generates an iterator over all possible combinations of ones and zeros
/// of a certain length, where the number of ones and zeros is specified by the parameters.
/// The iterator yields `Slab` objects, where each `Slab` represents one possible combination.
///
/// # Parameters
///
/// * `num_ones`: The number of ones in the slab.
/// * `num_zeros`: The number of zeros in the slab.
///
/// # Returns
///
/// An iterator over `Slab` objects, where each `Slab` represents one possible combination
/// of ones and zeros of the specified length.
pub fn all_slabs(num_ones: u32, num_zeros: u32) -> impl Iterator<Item = Slab> {
    let length = num_ones + num_zeros;

    (0..2u128.pow(length))
        .filter(move |x| x.count_ones() == num_ones)
        .map(move |x| Slab {
            data: x,
            length: length as usize,
        })
}
