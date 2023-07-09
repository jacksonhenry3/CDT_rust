//macro for printing a digit at a given length
macro_rules! format_digit {
    ($digit:expr, $length:expr) => {
        format!("{:0width$b}", $digit, width = $length)
            .chars()
            .rev()
            .collect::<String>()
    };
}

/// A slab is a sequence of 1s and 0s, where the 1s represent upwards pointing triangles and the 0s represent downwards pointing triangles.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct Slab {
    pub length: usize,
    pub data: u64,
}

impl Slab {
    pub fn new(data: Vec<bool>) -> Slab {
        Slab {
            length: data.len(),
            data: bool_vec_to_integer(data),
        }
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
        if self[space_index] == false {
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
        for i in 0..self.length {
            if sum == triangle_index {
                return i;
            }
            if self[i] == triangle_type {
                sum += 1;
            }
        }
        panic!("triangle index out of bounds");
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

pub fn bool_vec_to_integer(data: Vec<bool>) -> u64 {
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

pub fn sum_binary_digit_range(n: u64, start: usize, end: usize) -> usize {
    let mut sum = 0;
    for i in start..end {
        sum += (n >> i) & 1;
    }
    sum as usize
}
