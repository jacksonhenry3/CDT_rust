fn main() {
    let volume = 64;
    println!("{:?}", volume_profiles(volume));
}

fn volume_profiles(volume: u32) -> Vec<Vec<u32>> {
    /// volume profile is a list of spatial slice sizes
    /// But VOLUME is the number of triangles
    ///  2 * Total(Volume_profile) = VOLUME

    assert!(volume > 0, "Volume must be greater than 0");
    assert!((volume % 2) == 0, "Volume must be even");

    let mut volume_profile: Vec<Vec<u32>> = Vec::new();
    let total_length = volume / 2u32;

    volume_profile.push(vec![total_length]);

    // find all combinations of integers that add to total_length exluding 0

    for value in 1..total_length {
        for smaller_volume_profile in volume_profiles(2 * (total_length - value)) {
            let mut new_volume_profile = smaller_volume_profile;
            new_volume_profile.push(value);
            volume_profile.push(new_volume_profile);
        }
    }
    volume_profile
}
