use crate::slab::Slab;
use grafferous;

use rand::seq::SliceRandom;
use rand::Rng;

/// A CDT is a sequence of slabs, where the last slab is connected to the first slab.
#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash)]
pub struct CDT {
    pub slabs: Vec<Slab>,
}

impl CDT {
    pub fn new(slabs: Vec<Slab>) -> CDT {
        CDT { slabs: slabs }
    }

    pub fn random(volume_profile: Vec<u8>) -> CDT {
        let mut rng = rand::thread_rng();
        let mut slabs = Vec::new();
        for (i, volume) in volume_profile.clone().into_iter().enumerate() {
            //create a vec with the correct number of 1s and 0s
            let mut slab_data = vec![true; volume as usize];
            slab_data.append(&mut vec![
                false;
                (volume_profile[(i + 1) % volume_profile.len()])
                    as usize
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

    pub fn volume_profile(&self) -> Vec<(usize, usize)> {
        let mut result = vec![(0, 0); self.slabs.len()];
        for (i, slab) in self.iter().enumerate() {
            result[i] = (slab.ones(), slab.zeros());
        }
        result
    }

    pub fn number_of_triangles(&self) -> usize {
        2*self.slabs.iter().fold(0, |acc, x| acc + x.ones())
    }

    pub fn random_triangle(&self) -> (usize, usize) {
        let total_length = &self.slabs.iter().fold(0, |acc, x| acc + x.length);

        //random number between 0 and total_length
        let index = rand::thread_rng().gen_range(0..*total_length);

        //find the slab that contains the index using iterators
        let mut sum = 0;
        let mut time_index = 0;
        let mut space_index = 0;
        for slab in &self.slabs {
            if sum + slab.length > index {
                space_index = index - sum;
                break;
            }
            sum += slab.length;
            time_index += 1;
        }

        (time_index, space_index)
    }

    //get triangle index (how many other triangles of the same type have already appeared in that slabn) from time and space index
    pub fn get_triangle_index(&self, time_index: usize, space_index: usize) -> usize {
        let slab = &self.slabs[time_index];
        slab.get_triangle_index(space_index)
    }

    pub fn get_temporal_pair(&self, time_index: usize, space_index: usize) -> (usize, usize) {
        let slab = &self.slabs[time_index];
        let triangle = slab[space_index];
        let triangle_index = slab.get_triangle_index(space_index);

        let other_time_index = if triangle {
            (time_index + self.slabs.len() - 1) % self.slabs.len()
        } else {
            (time_index + 1) % self.slabs.len()
        };
        let other_slab = &self.slabs[other_time_index];

        let other_space_index = other_slab.get_triangle_in_slab_by_index(triangle_index, !triangle);

        //assert that the other triangle is a differnt type as the original triangle
        assert_ne!(triangle, other_slab[other_space_index]);

        (other_time_index, other_space_index)
    }

    pub fn all_transition_triangles(&self) -> Vec<(usize, usize)> {
        let mut different_triangles = Vec::new();

        for (time_index, slab) in self.slabs.iter().enumerate() {
            for (i, value) in slab.into_iter().enumerate() {
                if value != slab[(i + 1) % slab.length] {
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
            if slab.zeros() != other_slab.ones() {
                return false;
            }
        }
        true
    }

    pub fn to_graph(&self) -> grafferous::Graph<(i32,i32),()> {
        let mut g = grafferous::Graph::<(i32,i32),()>::new();

        let mut xf = 0;
        let mut xp = 0;
        let mut t = 0;


        g.add_node((t,xp));
        g.add_node((t+1,xf));
        
       
      
        for spatial_slice in self.slabs.clone() {
            let n = spatial_slice.ones() as i32;
            let m = spatial_slice.zeros() as i32;
            for triangle in spatial_slice {
                if !triangle {
                    xf+=1;
                    xf = xf.rem_euclid(n);

                    
                    g.add_node((xf,t+1));

                    g.add_directed_edge((xf,t), (xp,t+1));
                    g.add_directed_edge(((xf-1).rem_euclid(n),t), (xp,t+1));

                }
                else {
                    
                    xp+=1;
                    xp = xp.rem_euclid(m);

                    g.add_node((xp,t));

                    
                    g.add_directed_edge((xf,t), ((xp-1).rem_euclid(m),t+1));
                    g.add_directed_edge((xf,t), (xp,t+1));
                    
                }

            }
            t+=1;
        }

        g
    }

}

// display a CDT as a vertical stack of slabs
impl std::fmt::Display for CDT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let reverse_slabs = self.slabs.clone().into_iter().rev();
        for slab in reverse_slabs {
            write!(f, "{}\n", slab)?;
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
