#[cfg(test)]
mod tests {
    use cdt_rust::{cdt::CDT, volume_profiles::VolumeProfile, Slab};

    

    #[test]
    fn test_new() {
        let slabs = vec![
            Slab::new(vec![true, false, true]),
            Slab::new(vec![false, true, false]),
        ];
        let cdt = CDT::new(slabs.clone());
        assert_eq!(cdt.slabs, slabs);
    }

    #[test]
    fn test_random() {
        let volume_profile = VolumeProfile::new(vec![2, 3, 1]);
        let cdt = CDT::random(&volume_profile);
        assert_eq!(cdt.slabs.len(), 2);
    }

    #[test]
    fn test_new_flat() {
        let cdt = CDT::new_flat(4, 3);
        assert_eq!(cdt.slabs.len(), 3);
    }

    #[test]
    fn test_volume_profile() {
        let slabs = vec![
            Slab::new(vec![true, false, true]),
            Slab::new(vec![false, true, false]),
        ];
        let cdt = CDT::new(slabs);
        let volume_profile = cdt.volume_profile();
        assert_eq!(volume_profile.profile, vec![2, 1, 2]);
    }

    #[test]
    fn test_number_of_triangles() {
        let slabs = vec![
            Slab::new(vec![true, false, true]),
            Slab::new(vec![false, true, false]),
        ];
        let cdt = CDT::new(slabs);
        assert_eq!(cdt.number_of_triangles(), 8);
    }

    #[test]
    fn test_random_triangle() {
        let slabs = vec![
            Slab::new(vec![true, false, true]),
            Slab::new(vec![false, true, false]),
        ];
        let cdt = CDT::new(slabs);
        let (time_index, space_index) = cdt.random_triangle();
        assert!(time_index < cdt.time_size());
        assert!(space_index < cdt[time_index].len());
    }

    // Add more tests for the remaining functions...
}
