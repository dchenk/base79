use rand::prelude::*;

use base79::Base79;

// Start with a vector with one element at the midpoint, and 10K times randomly select an element in
// the vector and average it with either 0 (if at the first element in the vector), with the element
// immediately prior (if somewhere in the middle), or with 1 (if at the last element). Print the
// generated values, the length of the longest value, and the average length. This example shows how
// slowly the string grows.
fn main() {
    let mut v = vec![Base79::mid()];
    let mut rng = thread_rng();

    let n = 10_000;

    for _ in 0..n {
        let pos: usize = rng.gen_range(0..=v.len());
        if pos == 0 {
            v.insert(pos, Base79::avg_with_zero(&v[pos]));
        } else if pos == v.len() {
            v.insert(pos, Base79::avg_with_one(&v[pos - 1]));
        } else {
            v.insert(pos, Base79::avg(&v[pos - 1], &v[pos]));
        }
    }

    for n in &v {
        println!("{}", n.to_string());
    }
    println!(
        "Max len: {:?}",
        v.iter().map(|n| n.to_string().len()).max().unwrap()
    );
    println!(
        "Avg len: {:?}",
        v.iter().map(|n| n.to_string().len()).sum::<usize>() as f32 / n as f32
    );
}
