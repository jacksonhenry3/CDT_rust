use itertools::Itertools;


fn main() {
    let mut a = all_slabs(6,6);
    println!("{:?}", a.len());
}

//derive debug order
#[derive(Debug , Eq, PartialOrd, Ord, Clone)]
struct Slab {
    length: usize,
    data: u32,
}

impl Slab{
    fn new(data:Vec<bool>) -> Slab {
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
        println!("{}",m);
        while m % 2 == 0 {
            m = m >> 1;
        }
        m.hash(state);
    }
}


fn all_slabs(m:usize,n:usize) -> Vec<Slab> {
    let mut state = vec![false; m];
    state.append(&mut vec![true; n]);

    state.into_iter().permutations(m+n).map(|x| Slab::new(x)).sorted().dedup().collect()
}


fn bool_vec_to_integer(data:Vec<bool>) -> u32 {
    let mut result = 0;
    let mut pow = 0u32;

    let mut iter = data.iter().rev().peekable();
    
    while iter.peek().is_some() {
        if let Some(i) = iter.position(|&x| x) {
            pow+=i as u32;
            result+=2u32.pow(pow);
            pow+=1;
        }
    }
    //reduce to canonical form
    while result % 2 == 0 {
        result = result >> 1;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_equality() {
        let slab1 = Slab::new(vec![true, false, true, false]);
        let slab2 = Slab::new(vec![true, false, true, false]);
        let slab3 = Slab::new(vec![true, false, true, true]);
        assert_eq!(slab1, slab2);
        assert_ne!(slab1, slab3);
    }


    #[test]
    fn test_bool_vec_to_integer() {
        assert_eq!(bool_vec_to_integer(vec![true, false, true, false]), 10);
        assert_eq!(bool_vec_to_integer(vec![true, true, true, true]), 15);
        assert_eq!(bool_vec_to_integer(vec![false, false, false, true]), 1);
        assert_eq!(bool_vec_to_integer(vec![false, false, false, false]), 0);
        assert_eq!(bool_vec_to_integer(vec![true, true, true, false]), 14);
    }
}