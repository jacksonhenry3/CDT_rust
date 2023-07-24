#[cfg(test)]
mod tests {
    use cdt_rust::cdt::CDT;

    #[test]
    fn test_all_transition_triangles() {
        //test a random cdt
        let cdt = CDT::random(vec![10, 10, 10, 10, 10]);
        let transition_triangles = cdt.all_transition_triangles();
        for (time_index, space_index) in transition_triangles {
            let triangle = cdt[time_index][space_index];
            let (other_time_index, other_space_index) =
                (time_index, (space_index + 1) % cdt.slabs[time_index].length);
            let other_triangle = cdt[other_time_index][other_space_index];

            assert_ne!(triangle, other_triangle);
        }
    }

    #[test]
    fn test_is_valid() {
        let mut cdt = CDT::new_flat(3, 4);

        assert!(cdt.is_valid());

        cdt.slabs[0].data = 0b1000;
        assert!(!cdt.is_valid());
    }
}
