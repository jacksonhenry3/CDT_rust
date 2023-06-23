use graph::{self, Graph,ID};
use rand::thread_rng;
use rand::seq::SliceRandom;

//derive debug
#[derive(Debug)]
pub struct CDT {
    data: Vec<Vec<bool>>,
}

impl CDT {

    //this function isnt working, test by plugging in to mathematica.
    pub fn to_graph(&self) -> Graph<()> {
        let mut g = Graph::<()>::new();

        let mut past_index = 0usize;
        let mut future_index = self.data[0].len();

        g.add_node(ID(past_index));
        g.add_node(ID(future_index));
        g.add_directed_edge(ID(past_index), ID(future_index));
      
        for spatial_slice in self.data.iter() {
            for triangle in spatial_slice.iter() {
                if *triangle {
                    future_index+=1;
                    g.add_node(ID(future_index));
                    g.add_directed_edge(ID(past_index), ID(future_index));

                }
                else {
                    past_index+=1;
                    g.add_node(ID(past_index));
                    g.add_directed_edge(ID(past_index), ID(future_index));
                }

            }
        }

        g
    }

}

pub fn flat_cdt(x_size: usize, t_size: usize) -> CDT {
    //create t_size vecs of size x_size altyernating true and false
    let mut data = Vec::with_capacity(t_size);
    for i in 0..t_size {
        let mut row = Vec::with_capacity(x_size);
        for j in 0..x_size*2 {
            row.push((i) % 2 == 0);
        }
        data.push(row);
    }
    CDT { data }
}

pub fn random_cdt(volume_profile: Vec<usize>) -> CDT {
    let mut data = vec![];

    //for each row in the volume profile chose true and false such that the volume profile is satisfied
    for i in 0..volume_profile.len() {

        //choose volume_profile[i] number of true and volume_profile[(i+1)%volume_profile.len()] number of false
        let trues = vec![true; volume_profile[i]];
        let falses = vec![false; volume_profile[(i+1)%volume_profile.len()]];
        //combine and shuffle
        let mut combined = trues.into_iter().chain(falses.into_iter()).collect::<Vec<bool>>();
        combined.shuffle(&mut thread_rng());
        
        data.push(combined);
    }
    CDT { data }
}

fn main() {
    let g = (CDT {data : vec![vec![true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false]]}).to_graph();

    //loop through the edges and convert it in to mathematica format
    for (source,targets) in g.edges {
        for target in targets{
            println!("{} -> {},", source.0, target.0);
        }
    }


    
    // println!("{:?}",);
}
