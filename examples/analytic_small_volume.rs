use cdt_rust;

//this code should take a volume and a time size and generate a file of all CDTs
// where each line of the file corresponds to a CDT and its measured properties (in this case volume profile)

fn main() {
    let a = cdt_rust::volume_profiles::volume_profiles(16, 4);
    for i in a {
        println!("{:?}", i);
    }
}
