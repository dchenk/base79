use base79::Base79;

fn main() {
    let mut working = Base79::mid();
    for _ in 0..30 {
        working = Base79::avg_with_one(&working);
        println!("{:?}", working);
    }
}
