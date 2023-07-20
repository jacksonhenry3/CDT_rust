use crate::cdt::CDT;  
use rand::Rng;

trait Move {


    // Method signatures; these will return a string.
    fn acceptance_ratio(&self) -> f64;
    fn is_possible(&self) -> bool;
    fn execute(&self,  cdt:&mut CDT, location: (usize,usize)) -> ();
    fn get_region(&self, cdt:&CDT) -> (usize, usize);

    // Traits can provide default method definitions.
    fn try_execute(&self, cdt: &mut CDT) {
        let position = self.get_region(cdt);

        if ! self.is_possible() {
            return;
        }

        if rand::thread_rng().gen_bool(self.acceptance_ratio()) {
            self.execute(cdt, position);
        }
    }
}