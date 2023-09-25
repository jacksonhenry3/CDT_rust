use std::{collections::HashSet, hash::Hash};

use integer_partitions::Partitions;
use itertools::Itertools;

//derive eq
#[derive(Debug, Clone)]
pub struct VolumeProfile {
    pub profile: Vec<usize>,
    pub id: usize,
}

impl VolumeProfile {
    pub fn new(profile: Vec<usize>) -> VolumeProfile {
        let product = profile.iter().fold(1, |acc, x| acc * x);

        let mut id = 0;
        for (index, val) in profile.iter().enumerate() {
            let next = profile[(index + 1) % profile.len()];
            id += val * (product / next);
        }
        VolumeProfile { profile, id }
    }
}

impl PartialEq for VolumeProfile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for VolumeProfile {}

impl Hash for VolumeProfile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn non_cyclic_permutations(vec: Vec<usize>) -> HashSet<VolumeProfile> {
    let result: HashSet<_> = vec
        .iter()
        .cloned()
        .permutations(vec.len())
        .map(|x| VolumeProfile::new(x))
        .collect();

    result
}

pub fn volume_profiles(volume: usize) -> impl Iterator<Item = HashSet<VolumeProfile>> {
    //assert div 2, with message
    assert_eq!(volume % 2, 0, "volume must be divisible by 2");

    let mut a = Partitions::new(volume / 2);

    std::iter::from_fn(move || match a.next() {
        Some(profile) => Some(non_cyclic_permutations(profile.to_vec())),
        _ => None,
    })
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
