use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub type Int = u64;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PrimeMultiple {
    next_multiple: Int,
    prime: Int,
}

impl PrimeMultiple {
    fn new(p: Int) -> PrimeMultiple {
        PrimeMultiple { next_multiple: p * p, prime: p }
    }

    fn increment(&mut self) {
        self.next_multiple += self.prime
    }
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

pub struct Sieve {
    prime_multiples: BinaryHeap<PrimeMultiple>,
    last_prime: Int,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve { prime_multiples: BinaryHeap::new(), last_prime: 1 }
    }

    fn drain_multiples_eq_to(&mut self, n: Int) {
        while let Some(PrimeMultiple { next_multiple, .. }) = self.prime_multiples.peek() {
            if *next_multiple != n {
                break;
            }
            let mut multiple = self.prime_multiples.pop().unwrap();
            multiple.increment();
            self.prime_multiples.push(multiple);
        }
    }

    fn insert_new_prime(&mut self, p: Int) {
        self.prime_multiples.push(PrimeMultiple::new(p));
    }
}

impl Iterator for Sieve {
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let mut candidate_prime = self.last_prime + 1;

        for num in (self.last_prime + 1).. {
            match self.prime_multiples.peek() {
                Some(PrimeMultiple { next_multiple, .. }) if *next_multiple == num => {
                    self.drain_multiples_eq_to(num);
                },
                _ => {
                    break;
                }
            }

            candidate_prime += 1;
        };

        let prime = candidate_prime;
        self.last_prime = prime;
        self.insert_new_prime(prime);
        Some(prime)
    }
}

// New impl should enable:
// - Iterator over primes
//   - Should cache results so that future iterators don't redo work
//   - Re-using previous results should be seamless for iterator users (i.e. iterate 'til end of cached, then start discovering new primes)
//   - Might make sense to have a private iterator that we use for expanding the frontier of primes which the public iterator uses when it's beyond the cached results

struct PrimeNumbers {
    // This list of primes should grow over time, and should
    // be a source of prime caching for iteration purposes
    primes_list: Vec<Int>
}

// I feel like I want some sort of singleton pattern here, how do I go about that?
// Sounds like singletons are difficult and discouraged in Rust