use itertools::Itertools;

fn main() {
    let a = all_cdts(vec![7, 7, 7, 7, 7, 7, 7]);
    println!("{:?}", a.len());
}

//derive debug order
#[derive(Debug, Eq, PartialOrd, Ord, Clone)]
struct Slab {
    length: usize,
    data: u16,
}

impl Slab {
    fn new(data: Vec<bool>) -> Slab {
        Slab {
            length: data.len(),
            data: bool_vec_to_integer(data),
        }
    }
}

//impl equality for slab
impl PartialEq for Slab {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false;
        }

        self.data == other.data
    }
}

//impl hash for slab
impl std::hash::Hash for Slab {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut m = self.data;
        while m % 2 == 0 {
            m >>= 1;
        }
        m.hash(state);
    }
}

fn all_slabs(m: u32, n: u32) -> Vec<Slab> {
    (0u16..2u16.pow(m + n))
        .filter(|x| x.count_ones() == n)
        .map(reduce_to_canonical_form)
        .unique()
        .map(|x| Slab {
            length: (m + n) as usize,
            data: x,
        })
        .collect()
}

fn all_cdts(volume_profile: Vec<u32>) -> Vec<Vec<Slab>> {
    let mut all_slabs_for_each_layer = vec![];
    for i in 0..volume_profile.len() {
        let past = volume_profile[i];
        let future = volume_profile[(i + 1) % volume_profile.len()];

        let all_slabs = all_slabs(past, future);
        all_slabs_for_each_layer.push(all_slabs);
    }

    all_slabs_for_each_layer
        .into_iter()
        .multi_cartesian_product()
        .collect()
}

fn bool_vec_to_integer(data: Vec<bool>) -> u16 {
    let mut result = 0;
    let mut pow = 0u32;

    let mut iter = data.iter().rev().peekable();

    while iter.peek().is_some() {
        if let Some(i) = iter.position(|&x| x) {
            pow += i as u32;
            result += 2u16.pow(pow);
            pow += 1;
        }
    }
    //reduce to canonical form
    reduce_to_canonical_form(result)
}

fn reduce_to_canonical_form(x: u16) -> u16 {
    let mut result = x;
    while result % 2 == 0 {
        result >>= 1;
    }
    result
}
