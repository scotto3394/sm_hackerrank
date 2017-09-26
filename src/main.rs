extern crate rust_bucket;
use rust_bucket::algorithms::implementation::Cycle;

fn main() {
    let mut example = Cycle::new(
        4,
        4,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    );
    let example_mat = example.get_mat();
    println!("{:?}", example_mat);
    example.rotate(1);
    let example_mat_rot = example.get_mat();
    println!("{:?}", example_mat_rot);
}
