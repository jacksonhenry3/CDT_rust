use crate::slab::Slab;
use crate::volume_profiles::{self, VolumeProfile};
use grafferous;

use rand::seq::SliceRandom;
use rand::Rng;
/// A CDT is a sequence of slabs, where the last slab is connected to the first slab.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct CDT {
    // This should probably be an array, not a slice, becouse it doesnt change size at run time. The size can be defined in a const.
    pub slabs: Vec<Slab>,
}

impl CDT {
    pub fn new(slabs: Vec<Slab>) -> CDT {
        CDT { slabs }
    }

    pub fn random(volume_profile: VolumeProfile) -> CDT {
        let volume_profile = volume_profile.profile;
        let mut rng = rand::thread_rng();
        let mut slabs = Vec::new();
        for (i, volume) in volume_profile.clone().into_iter().enumerate() {
            //create a vec with the correct number of 1s and 0s
            let mut slab_data = vec![true; volume];
            slab_data.append(&mut vec![
                false;
                volume_profile[(i + 1) % volume_profile.len()]
            ]);

            slab_data.shuffle(&mut rng);

            slabs.push(Slab::new(slab_data));
        }
        CDT::new(slabs)
    }

    pub fn new_flat(space_size: usize, time_size: usize) -> CDT {
        assert!(space_size <= 128, "space size must be less than 128");
        assert!(time_size <= 128, "time size must be less than 128");
        let slab_data = Slab::new((0..2 * space_size).map(|x| x % 2 == 0).collect());
        CDT::new((0..time_size).map(|_| slab_data.clone()).collect())
    }

    pub fn volume_profile(&self) -> VolumeProfile {
        let mut result = vec![0; self.slabs.len()];
        for (i, slab) in self.iter().enumerate() {
            result[i] = slab.count_true();
        }
        VolumeProfile::new(result.into())
    }

    pub fn number_of_triangles(&self) -> usize {
        2 * self.slabs.iter().fold(0, |acc, x| acc + x.count_true())
    }

    pub fn random_triangle(&self) -> (usize, usize) {
        let total_length = &self.slabs.iter().fold(0, |acc, x| acc + x.len());

        //random number between 0 and total_length
        let index = rand::thread_rng().gen_range(0..*total_length);

        //find the slab that contains the index using iterators
        let mut sum = 0;
        let mut time_index = 0;
        let mut space_index = 0;
        for slab in &self.slabs {
            if sum + slab.len() > index {
                space_index = index - sum;
                break;
            }
            sum += slab.len();
            time_index += 1;
        }

        (time_index, space_index)
    }

    //get triangle index (how many other triangles of the same type have already appeared in that slabn) from time and space index
    pub fn get_triangle_index(&self, time_index: usize, space_index: usize) -> usize {
        let slab = &self.slabs[time_index];
        slab.get_triangle_index(space_index)
    }

    pub fn get_temporal_pair(
        &self,
        time_index: usize,
        space_index: usize,
    ) -> Option<(usize, usize)> {
        let slab = &self.slabs[time_index];
        let triangle = slab[space_index];
        let triangle_index = slab.get_triangle_index(space_index);

        // connect the top and bottom layers.
        let other_time_index = if triangle {
            (time_index + self.time_size() - 1).rem_euclid(self.time_size())
        } else {
            (time_index + 1).rem_euclid(self.time_size())
        };

        let other_slab = &self.slabs[other_time_index];

        //print the cdt
        let other_space_index = other_slab.get_triangle_in_slab_by_index(triangle_index, !triangle);

        //assert that the other triangle is a differnt type as the original triangle
        assert_ne!(triangle, other_slab[other_space_index]);

        Some((other_time_index, other_space_index))
    }

    pub fn all_transition_triangles(&self) -> Vec<(usize, usize)> {
        let mut different_triangles = Vec::new();

        for (time_index, slab) in self.slabs.iter().enumerate() {
            for (i, value) in slab.into_iter().enumerate() {
                if value != slab[(i + 1) % slab.len()] {
                    different_triangles.push((time_index, i));
                }
            }
        }

        different_triangles
    }

    pub fn random_transition_triangle(&self) -> (usize, usize) {
        let transition_triangles = self.all_transition_triangles();
        //select a random transition triangle
        *transition_triangles
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    pub fn triangles(&self) -> Vec<(usize, usize, bool)> {
        let mut result = Vec::new();
        for (time_index, slab) in self.slabs.iter().enumerate() {
            for (space_index, value) in slab.into_iter().enumerate() {
                result.push((time_index, space_index, value));
            }
        }
        result
    }

    pub fn is_valid(&self) -> bool {
        //check that each past sslab has the same number of ones as its corresponding future slab has zeros
        for (i, slab) in self.slabs.iter().enumerate() {
            let other_slab = &self.slabs[(i + 1) % self.slabs.len()];
            if slab.count_false() != other_slab.count_true() {
                return false;
            }
        }
        true
    }

    pub fn to_graph(&self) -> grafferous::Graph<(i32, i32), ()> {
        let mut g = grafferous::Graph::<(i32, i32), ()>::new();

        let mut xp: i32 = 0;
        let mut xf: i32 = 0;

        let time_size = self.slabs.len() as i32;

        for (t, spatial_slice) in self.slabs.iter().enumerate() {
            let t = t as i32;
            let n = spatial_slice.count_true() as i32;
            let m = spatial_slice.count_false() as i32;
            for triangle in spatial_slice {
                if triangle {
                    xp += 1;
                    xp = xp.rem_euclid(n);
                    g.add_directed_edge((t, xp), ((t + 1).rem_euclid(time_size), xf));
                } else {
                    xf += 1;
                    xf = xf.rem_euclid(m);
                    g.add_directed_edge((t, xp), ((t + 1).rem_euclid(time_size), xf));
                }
            }
        }

        g
    }

    pub fn time_size(&self) -> usize {
        self.slabs.len()
    }

    pub fn spatial_multiplicity(&self) -> usize {
        // gives the number of different representations that corespond to the same physical triangulation.

        //create a graph
        let g = self.to_graph();

        //for each node in the first row of the graph count the number of paths back to that node
        let mut result = 0;
        for node in g.nodes.clone() {
            result += grafferous::count_paths(&g, &node, &node, Some(self.time_size()));
        }

        result
    }

    pub fn temporal_multiplicity(&self) -> usize {
        //check for a repeating pattern of slabs of length T/2 or less by shifting each slab in the cdt by 1 and checking if it is equal to the original cdt
        for shift in 1..=self.time_size() / 2 {
            let mut rotated_slabs = self.slabs.clone();
            rotated_slabs.rotate_right(shift);
            if self.slabs == rotated_slabs {
                match shift {
                    1 => return 1,
                    2 => return 2,
                    _ => return 2 * shift,
                }
            }
        }

        2 * self.time_size()
    }
}

// display a CDT as a vertical stack of slabs
impl std::fmt::Display for CDT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let reverse_slabs = self.slabs.clone().into_iter().rev();
        for slab in reverse_slabs {
            writeln!(f, "{}", slab)?;
        }
        Ok(())
    }
}

//impl deref
impl std::ops::Deref for CDT {
    type Target = Vec<Slab>;
    fn deref(&self) -> &Self::Target {
        &self.slabs
    }
}

//impl deref mut
impl std::ops::DerefMut for CDT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slabs
    }
}

fn measure_boundaries(cdt: &CDT) -> usize {
    let _max = f64::NAN;
    let transition_triangles = cdt.all_transition_triangles();
    transition_triangles.len()
}

pub fn random_sample(time_size: usize, volume: usize, num_samples: usize) -> Vec<CDT> {
    let mut result = Vec::new();
    let volume_profiles = volume_profiles::random_sample(volume, time_size, num_samples);
    for profile in volume_profiles {
        result.push(CDT::random(profile));
    }

    result
}
