use cdt_rust::Slab;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.length, 3);
        assert_eq!(slab.data, 5);
    }

    #[test]
    fn test_set() {
        let mut slab = Slab::new(vec![true, false, true]);
        slab.set(1, true);
        assert_eq!(slab.data, 7);
    }

    #[test]
    fn test_get_triangle_index() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.get_triangle_index(0), 0);
        assert_eq!(slab.get_triangle_index(1), 0);
        assert_eq!(slab.get_triangle_index(2), 1);
    }

    #[test]
    fn test_get_triangle_in_slab_by_index() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.get_triangle_in_slab_by_index(0, true), 0);
        assert_eq!(slab.get_triangle_in_slab_by_index(0, false), 1);
        assert_eq!(slab.get_triangle_in_slab_by_index(1, true), 2);
    }

    #[test]
    fn test_insert() {
        let mut slab = Slab::new(vec![true, false, true]);
        slab.insert(1, true);
        assert_eq!(slab.data, 11);
        assert_eq!(slab.length, 4);
    }

    #[test]
    fn test_remove() {
        let mut slab = Slab::new(vec![true,true, false, true]);
        slab.remove(1);
        assert_eq!(slab.data, 7);
        assert_eq!(slab.length, 3);

        let mut slab = Slab::new(vec![true,true, false, true]);
        slab.remove(0);
        assert_eq!(slab.data, 6);
        assert_eq!(slab.length, 3);

        // 10 tests with new data
        let mut slab = Slab::new(vec![true, false, true, true, false, true, true, false, true, true]);
        slab.remove(0);
        assert_eq!(slab.data, 0b101101101);
        assert_eq!(slab.length, 9);

        let mut slab = Slab::new(vec![true, false, true, true, false, true, true, false, true, true]);
        slab.remove(1);
        println!("slab.data: {}", slab);
        assert_eq!(slab.data, 0b101101101);
        assert_eq!(slab.length, 9);

        // let mut slab = Slab::new(vec![true, false, true, true, false, true, true, false, true, true]);
        // slab.remove(2);
        // assert_eq!(slab.data, 0b101101111);

    }

    #[test]
    fn test_ones() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.ones(), 2);
    }

    #[test]
    fn test_zeros() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.zeros(), 1);
    }

    #[test]
    #[should_panic(expected = "slab length cannot be greater than 64")]
    fn test_insert_panic() {
        let mut slab = Slab::new(vec![true; 64]);
        slab.insert(0, false);
    }

    #[test]
    #[should_panic(expected = "slab length cannot be less than 3")]
    fn test_remove_panic() {
        let mut slab = Slab::new(vec![true, false, true]);
        slab.remove(1);
        slab.remove(0);
    }
}