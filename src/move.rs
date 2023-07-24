use crate::cdt::CDT;
use rand::Rng;

const LAMBDA: f64 = 0.69314718056;
pub trait Move {
    // Method signatures; these will return a string.
    fn acceptance_ratio(&self,cdt: &CDT) -> f64;
    fn is_possible(&self, cdt: &CDT, location: (usize, usize)) -> bool;
    fn execute(&self, cdt: &mut CDT, location: (usize, usize)) -> ();
    fn random_valid_position(&self, cdt: &CDT) -> (usize, usize);

    // Traits can provide default method definitions.
    fn try_execute(&self, cdt: &mut CDT) -> bool {
        let position = self.random_valid_position(cdt);

        if !self.is_possible(cdt, position) {
            return false;
        }

        if !rand::thread_rng().gen_bool(self.acceptance_ratio(cdt)) {
            return false;
        }

        self.execute(cdt, position);
        return true
    }

    fn trial_execute(&self, cdt: &CDT, location: (usize, usize)) -> CDT {
        let mut new_cdt = cdt.clone();
        self.execute(&mut new_cdt, location);
        new_cdt
    }

    fn delta_s(&self, cdt: &CDT, location: (usize, usize), action: &dyn Fn(&CDT) -> f64) -> f64 {
        let trial_cdt = self.trial_execute(cdt, location);
        action(&trial_cdt) - action(cdt)
    }
}

pub struct DecreaseMove;

//this isnt a 42 move its a merge move. 
pub struct IncreaseMove;
pub struct ParityMove;

impl Move for DecreaseMove {
    fn random_valid_position(&self, cdt: &CDT) -> (usize, usize) {
        cdt.random_triangle()
    }

    fn acceptance_ratio(&self, cdt: &CDT) -> f64 {
        let N0 = (cdt.number_of_triangles()/2) as f64;
        let possible = (N0+1.0)/N0*f64::exp(2.0*LAMBDA);

        //possible or 1
        possible.min(1.0)

    }

    fn is_possible(&self, cdt: &CDT, location: (usize, usize)) -> bool {
        // check if either of the effected slabs would decrease such that they have fewer than 3 ones or zeros.
        // if so, then the move is not possible
        let (time_index, space_index) = location;
        let (other_time_index, _other_space_index) = cdt.get_temporal_pair(time_index, space_index);

        if cdt.slabs[time_index].ones() < 4 || cdt.slabs[other_time_index].ones() < 4 {
            return false;
        }

        if cdt.slabs[time_index].zeros() < 4 || cdt.slabs[other_time_index].zeros() < 4 {
            return false;
        }

        true
    }

    fn execute(&self, cdt: &mut CDT, location: (usize, usize)) -> () {
        let (time_index, space_index) = location;
        let (other_time_index, other_space_index) = cdt.get_temporal_pair(time_index, space_index);

        //remove whatever value is at the past_time_index and past_space_index
        cdt.slabs[other_time_index].remove(other_space_index);
        cdt.slabs[time_index].remove(space_index);
    }
}

impl Move for IncreaseMove {
    fn random_valid_position(&self, cdt: &CDT) -> (usize, usize) {
        cdt.random_triangle()
    }

    fn acceptance_ratio(&self, cdt: &CDT) -> f64 {
        let N0 = (cdt.number_of_triangles()) as f64;
        let possible = N0/(N0+1.0)*f64::exp(-2.0*LAMBDA);

        //possible or 1
        possible.min(1.0)

    }

    fn is_possible(&self, cdt: &CDT, location: (usize, usize)) -> bool {
        // check if either of the effected slabs would increase such that they have more than 127 ones or zeros.
        // if so, then the move is not possible
        let (time_index, space_index) = location;
        let (other_time_index, _other_space_index) = cdt.get_temporal_pair(time_index, space_index);

        if cdt.slabs[time_index].length == 127 || cdt.slabs[other_time_index].length == 127 {
            return false;
        }

        true
    }

    fn execute(&self, cdt: &mut CDT, location: (usize, usize)) -> () {
        let (time_index, space_index) = location;
        let (other_time_index, other_space_index) = cdt.get_temporal_pair(time_index, space_index);

        //calculate the delta_t using modular arithmetic
        let delta_t = (other_time_index + cdt.slabs.len() - time_index) % cdt.slabs.len();

        //if delta_t is 1 then time_index is the past and other_time_index is the future
        let (past_time_index, past_space_index, future_time_index, future_space_index) =
            if delta_t != 1 {
                (time_index, space_index, other_time_index, other_space_index)
            } else {
                (other_time_index, other_space_index, time_index, space_index)
            };

        //modify the slab at time_index and the corresponding slab at time_index+1
        cdt.slabs[past_time_index].insert(past_space_index, true);
        cdt.slabs[future_time_index].insert(future_space_index, false);
    }
}

impl Move for ParityMove {
    fn random_valid_position(&self, cdt: &CDT) -> (usize, usize) {
        cdt.random_transition_triangle()
    }

    fn acceptance_ratio(&self, cdt:&CDT) -> f64 {
        1.0
    }

    fn is_possible(&self, _cdt: &CDT, _location: (usize, usize)) -> bool {
        true
    }

    fn execute(&self, cdt: &mut CDT, location: (usize, usize)) -> () {
        let (time_index, space_index) = location;
        let other_space_index = (space_index + 1) % cdt.slabs[time_index].length;

        assert!(
            cdt[time_index][space_index] != cdt[time_index][other_space_index],
            "tried to swap {} and {} in slice {} which are both {}. THe fact that you are seeing this means something is very wrong with how transition triangles are being selected.",
            space_index,
            other_space_index,
            time_index,
            cdt[time_index][space_index]
        );

        let new_value = !cdt[time_index][space_index];

        cdt[time_index].set(space_index, new_value);
        cdt[time_index].set(other_space_index, !new_value);
    }
}
