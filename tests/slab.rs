#[cfg(test)]
mod tests {
    use cdt_rust::{slab::all_slabs, Direction, Slab};

    #[test]
    fn test_length() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.length(), 3);
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
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.get_triangle_index(0), 0);
        assert_eq!(slab.get_triangle_index(1), 0);
        assert_eq!(slab.get_triangle_index(2), 1);
    }

    #[test]
    fn test_get_triangle_in_slab_by_index() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.get_triangle_in_slab_by_index(0, true), 0);
        assert_eq!(slab.get_triangle_in_slab_by_index(1, false), 1);
    }

    #[test]
    #[should_panic(expected = "triangle index out of bounds")]
    fn test_get_triangle_in_slab_by_index_panic() {
        let slab = Slab::new(vec![true, false, true]);
        slab.get_triangle_in_slab_by_index(2, true);
    }

    #[test]
    fn test_is_boundary() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(slab.is_boundary(0, Direction::Left), true);
        assert_eq!(slab.is_boundary(0, Direction::Right), false);
        assert_eq!(slab.is_boundary(2, Direction::Left), false);
        assert_eq!(slab.is_boundary(2, Direction::Right), true);
    }

    #[test]
    fn test_not() {
        let slab = Slab::new(vec![true, false, true]);
        let negated_slab = !slab;
        assert_eq!(negated_slab, Slab::new(vec![false, true, false]));
    }

    #[test]
    fn test_display() {
        let slab = Slab::new(vec![true, false, true]);
        assert_eq!(format!("{}", slab), "^v^");
    }

    #[test]
    fn test_all_slabs() {
        let slabs: Vec<Slab> = all_slabs(2, 1).collect();
        assert_eq!(slabs.len(), 3);
        assert!(slabs.contains(&Slab::new(vec![true, true, false])));
        assert!(slabs.contains(&Slab::new(vec![true, false, true])));
        assert!(slabs.contains(&Slab::new(vec![false, true, true])));
    }
}
