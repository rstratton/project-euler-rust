use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Int = u64;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PrimeMultiple {
    next_multiple: Int,
    prime: Int,
}

impl Ord for PrimeMultiple {
    fn cmp(&self, other: &PrimeMultiple) -> Ordering {
        other.next_multiple.cmp(&self.next_multiple)
    }
}

impl PartialOrd for PrimeMultiple {
    fn partial_cmp(&self, other: &PrimeMultiple) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Sieve {
    prime_multiples: BinaryHeap<PrimeMultiple>,
    last_prime: Int,
}

impl Sieve {
    fn new() -> Sieve {
        Sieve { prime_multiples: BinaryHeap::new(), last_prime: 1 }
    }

    fn drain_multiples_eq_to(&mut self, n: Int) {
        loop {
            let top = self.prime_multiples.peek();

            match top {
                Some(PrimeMultiple { next_multiple: m, prime: _ }) => {
                    if *m != n {
                        break;
                    }
                    let mut multiple = self.prime_multiples.pop().unwrap();
                    multiple.next_multiple += multiple.prime;
                    self.prime_multiples.push(multiple);
                },
                None => { break; }
            }
        }
    }
}

impl Iterator for Sieve {
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let mut candidate_prime = self.last_prime;

        loop {
            candidate_prime += 1;

            match self.prime_multiples.peek() {
                Some(PrimeMultiple { next_multiple, .. }) if *next_multiple == candidate_prime => {
                    self.drain_multiples_eq_to(candidate_prime);
                },
                _ => {
                    // If heap is empty (i.e. when candidate_prime is 2), we've found a prime.
                    break;
                }
            }
        }

        let prime = candidate_prime;
        self.last_prime = prime;
        self.prime_multiples.push(PrimeMultiple { next_multiple: prime * prime, prime: prime });
        Some(prime)
    }
}

fn main() {
    let sieve = Sieve::new();
    let values: Vec<Int> = sieve.take(10001).collect();

    println!("{:?}", values[values.len()-1]); 
}