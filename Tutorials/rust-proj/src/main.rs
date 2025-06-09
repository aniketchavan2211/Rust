
use rand::{Rng, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let n: u32 = rng.r#gen();
    println!("Random number is : {}", n);
}