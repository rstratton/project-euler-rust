use std::collections::HashMap;

type PrimeFactorization = HashMap<u64, u64>;

fn divisor_limit(n: u64) -> u64 {
    ((n as f64).sqrt().ceil() as u64) + 1
}

fn prime_factorization_of(n: u64) -> PrimeFactorization {
    let mut remainder = n;
    let mut factors = HashMap::new();
    for d in 2..divisor_limit(n) {
        while remainder % d == 0 {
            remainder /= d;
            let entry = factors.entry(d).or_insert(0);
            *entry += 1;
        }
    }

    if remainder > 1 {
        factors.insert(remainder, 1);
    }

    factors
}

fn merge_prime_factorizations(prime_factorizations: Vec<PrimeFactorization>) -> PrimeFactorization {
    let mut result = HashMap::new();

    for p in prime_factorizations {
        for (prime, multiplicity) in p {
            result.entry(prime)
                  .and_modify(|entry| *entry = if multiplicity > *entry {multiplicity} else {*entry})
                  .or_insert(multiplicity);
        }
    }

    result
}

fn multiply_prime_factorization(p: PrimeFactorization) -> u64 {
    let mut result = 1;

    for (prime, multiplicity) in p {
        for _ in 0..multiplicity {
            result *= prime;
        }    
    }

    result
}

fn main() {
    let mut prime_factorizations = Vec::new();

    for n in 2..=20 {
        prime_factorizations.push(prime_factorization_of(n));
    }
    
    let merged_factorization = merge_prime_factorizations(prime_factorizations);

    println!("{}", multiply_prime_factorization(merged_factorization));
}