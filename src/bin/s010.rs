use num_bigint::BigUint;
use primes::*;

fn main() {
    let sieve = Sieve::new();
    let sum_of_primes: BigUint = sieve.take_while(|n| n < &2_000_000).sum();
    println!("{}", sum_of_primes);
}
