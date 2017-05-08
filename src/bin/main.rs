extern crate prime;

use prime::prime;

fn main() {
    for v in prime(500) {
        println!("p = {}", v);
    }
}