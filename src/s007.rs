use primes::Sieve;
use primes::Int;

fn main() {
    let sieve = Sieve::new();
    let values: Vec<Int> = sieve.take(10001).collect();

    println!("{:?}", values[values.len()-1]); 
}