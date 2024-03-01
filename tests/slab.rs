use cdt_rust::Slab;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use cdt_rust::Direction;

    use super::*;

    #[test]
    fn test_new() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.data.len(), 3);
    }

    #[test]
    fn test_count_true() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.count_true(), 2);
    }

    #[test]
    fn test_count_false() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.count_false(), 1);
    }

    #[test]
    fn test_string() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.string(), "^v^");
    }

    #[test]
    fn test_get_triangle_index() {
        let slab = Slab::new(vec![true, false, true, false]);
        assert_eq!(slab.get_triangle_index(2), 1);
    }

    #[test]
    #[should_panic(expected = "triangle index out of bounds")]
    fn test_get_triangle_in_slab_by_index_out_of_bounds() {
        let slab = Slab::new(vec![true, false, true, false]);
        slab.get_triangle_in_slab_by_index(5, true);
    }

    #[test]
    fn test_is_boundary() {
        let slab = Slab::new(vec![true, false, true, false]);
        assert_eq!(slab.is_boundary(0, Direction::Left), true);
        assert_eq!(slab.is_boundary(3, Direction::Right), true);
        assert_eq!(slab.is_boundary(1, Direction::Left), false);
    }

    #[test]
    fn test_not() {
        let slab = Slab::new(vec![true, false, true]);
        let not_slab = !slab;
        assert_eq!(not_slab.data, Arc::from(vec![false, true, false]));
    }
}
