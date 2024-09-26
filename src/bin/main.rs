use prime::prime;
use std::time::Instant;

fn main() {
    for (i, v) in prime(500).enumerate() {
        println!("p{i} = {v}");
    }

    println!("\n\n");
    let max = 100_000_000;
    let now = Instant::now();
    println!("total prime numbers within {}: {}", max, prime(max).count());
    println!("time = {:?}", now.elapsed());
}
