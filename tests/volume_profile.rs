use cdt_rust::volume_profiles::volume_profiles;

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashSet, VecDeque},
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;
    #[test]
    fn volume_test() {
        for volume in (2..17).step_by(2) {
            let profiles = volume_profiles(volume);

            assert!(profiles.into_iter().all(|p| p.into_iter().all(|p| p
                .profile
                .iter()
                .copied()
                .sum::<usize>()
                == volume / 2)))
        }
    }
    #[test]
    //#[ignore]
    fn uniqueness_test() {
        for volume in (2..20).step_by(2) {
            let profiles = volume_profiles(volume);
            let mut set = HashSet::new();
            for p in profiles.flatten() {
                println!("{:?}", p);
                assert!(set.insert(p));
            }
        }
    }

    #[test]
    #[ignore]
    fn volume_profile_test() {
        for path in std::fs::read_dir("tests/cdt_ref").unwrap() {
            let path = path.unwrap().path();
            let file = File::open(&path).unwrap();

            let volume = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_start_matches("cdt_ref_")
                .parse::<usize>()
                .unwrap();
            //max testing size
            if volume >= 11 {
                continue;
            }

            let set = BufReader::new(file)
                .lines()
                .map(|line| {
                    let line = line.unwrap();

                    line.trim_matches(['{', '}'].as_slice())
                        .split(", ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<VecDeque<_>>()
                })
                .collect::<HashSet<_>>();

            println!("volume: {}", volume);
            let profiles = volume_profiles(2 * volume).collect::<Vec<_>>();
            assert_eq!(profiles.iter().map(HashSet::len).sum::<usize>(), set.len());
            for profile in profiles.into_iter().flatten() {
                assert!(set.contains(&profile.profile));
            }
        }
    }
}
