#[cfg(test)]
mod tests {
    use cdt_rust::volume_profiles::{
        self, acceptance_function, generate_sample_profile, step, volume_profile_samples,
        VolumeProfile,
    };

    #[test]
    fn test_step() {
        let volume_profile = VolumeProfile::new(vec![1, 2, 3, 4]);
        let new_profile = step(&volume_profile, 1);
        assert_ne!(volume_profile, new_profile);
    }

    #[test]
    fn test_acceptance_function() {
        let old_profile = VolumeProfile::new(vec![1, 2, 3, 4]);
        let new_profile = VolumeProfile::new(vec![2, 3, 4, 5]);
        let log_old_num_cdts = volume_profiles::log_num_cdts_in_profile(old_profile.clone()).1;
        let (result, _, _) =
            acceptance_function(old_profile, log_old_num_cdts, new_profile.clone());
        assert_eq!(result, new_profile);
    }

    #[test]
    fn test_volume_profile_samples_complete() {
        // this is a stochastic test, that makes sure the acceptance function has error less than 10%
        let initial_state = VolumeProfile::new(vec![3, 3, 3]);
        //  generate 10_000 samples profiles and make sure none of them are significantly more likely than the others
        let num_steps = 10;
        let samples = volume_profile_samples(initial_state.clone(), num_steps, 10_000, 1);

        let num_samples = samples.len();
        let mut counts = std::collections::HashMap::new();
        for sample in samples {
            let count = counts.entry(sample).or_insert(0f64);
            *count += 1f64;
        }

        // divide each entry by num_samples to get the probability of each sample
        for count in counts.values_mut() {
            *count /= num_samples as f64;
        }

        // create the expected probability for each key
        let mut expected = std::collections::HashMap::new();
        for key in counts.keys() {
            expected.insert(
                key.clone(),
                volume_profiles::num_cdts_in_profile(&initial_state) as f64,
            );
        }

        // get the expected probability for each key
        let total = expected.values().sum::<f64>();
        for value in expected.values_mut() {
            *value /= total;
        }

        // compare the expected and actual probabilities
        for (key, value) in counts.iter() {
            let expected_value = expected.get(key).unwrap();
            assert!((value - expected_value).abs() < 0.05);
        }
    }

    #[test]
    fn test_volume_profile_samples() {
        let initial_state = VolumeProfile::new(vec![3, 2, 3, 4]);
        let num_steps = 10;
        let num_samples = 5;
        let result = volume_profile_samples(initial_state, num_steps, num_samples, 1);
        assert_eq!(result.len(), num_samples);
    }

    #[test]
    fn test_volume_profiles() {
        let volume = 4;
        let time_size = 2;
        let result = volume_profiles::volume_profiles(volume, time_size);
        println!("{:?}", result);
        assert_eq!(result.len(), 3); // There should be 3 unique volume profiles for the given volume and time size
    }

    #[test]
    fn test_log_num_cdts_in_profile() {
        let volume_profile = VolumeProfile::new(vec![1, 2, 3, 4]);
        let (_vp, result) = volume_profiles::log_num_cdts_in_profile(volume_profile);
        assert!(result.is_finite());
    }

    #[test]
    fn test_num_cdts_in_profile() {
        let volume_profile = VolumeProfile::new(vec![1, 2, 3, 4]);
        let result = volume_profiles::num_cdts_in_profile(&volume_profile);
        assert!(result > 0);
    }
}
