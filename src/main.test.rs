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
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(7));
    }

    #[test]
    fn test_bool_vec_to_integer() {
        assert_eq!(bool_vec_to_integer(vec![true, false, true, false]), 5);
        assert_eq!(bool_vec_to_integer(vec![true, true, true, true]), 15);
        assert_eq!(bool_vec_to_integer(vec![false, false, false, true]), 1);
    }
}