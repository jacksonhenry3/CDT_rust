use std::collections::HashMap;

const VOLUME: u32 = 16;

pub fn ok() {
    println!("{:?}", volume_profiles(VOLUME));
}

fn volume_profiles(volume: u32) -> Vec<Vec<u32>> {
    // volume profile is a list of spatial slice sizes
    // But VOLUME is the number of triangles
    //  2 * Total(Volume_profile) = VOLUME

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

fn histogram(volume: u32) -> HashMap<u64, u32> {
    let profiles = volume_profiles(volume);
    let mut counts = HashMap::new();
    for profile in profiles {
        let count = num_cdts_in_profile(&profile);
        *counts.entry(count).or_insert(0) += 1;
    }
    counts
}

pub fn write_histogram_to_file(volume: u32) {
    let histogram = histogram(volume);
    let mut sorted = histogram.into_iter().collect::<Vec<_>>();
    sorted.sort_by_key(|&(n, _)| n);
    let _ = std::fs::write(
        format!("Histogram {volume}"),
        sorted
            .into_iter()
            .map(|(num, freq)| format!("{num}: {freq}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

fn num_cdts_in_profile(volume_profile: &[u32]) -> u64 {
    let mut count = 0;
    for i in 1..volume_profile.len() {
        let n = volume_profile[i];
        let m = volume_profile[i - 1];
        count += chose_n_plus_m(n, n);
    }
    count
}

fn chose_n_plus_m(n: u32, m: u32) -> u64 {
    let n = u128::from(n);
    let m = u128::from(m);
    (((n + 1)..=m).product::<u128>() / (1..=m).product::<u128>()) as u64
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;
    #[test]
    fn volume_test() {
        for volume in (2..50).step_by(2) {
            let profiles: Vec<Vec<u32>> = volume_profiles(volume);

            assert!(profiles
                .into_iter()
                .all(|p| p.into_iter().sum::<u32>() == volume / 2))
        }
    }
    #[test]
    fn uniqueness_test() {
        for volume in (2..50).step_by(2) {
            let profiles: Vec<Vec<u32>> = volume_profiles(volume);
            let mut set = HashSet::new();
            for p in profiles {
                assert!(set.insert(p))
            }
        }
    }

    #[test]
    #[ignore]
    fn volume_profile_test() {
        for path in std::fs::read_dir("cdt_ref").unwrap() {
            let path = path.unwrap().path();
            let file = File::open(&path).unwrap();

            let volume = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_start_matches("cdt_ref_")
                .parse::<u32>()
                .unwrap();

            let set = BufReader::new(file)
                .lines()
                .map(|line| {
                    let line = line.unwrap();

                    line.trim_matches(['{', '}'].as_slice())
                        .split(", ")
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>();

            let profiles = volume_profiles(2 * volume);
            assert_eq!(profiles.len(), set.len());
            for profile in profiles {
                assert!(set.contains(&profile));
            }
        }
    }
}
