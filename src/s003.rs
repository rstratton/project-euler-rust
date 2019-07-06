use std::f32;

fn is_prime(n: u64) -> bool {
    for i in 2..((n as f32).sqrt().ceil() as u64) + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_factor(n: u64, m: u64) -> bool {
    n % m == 0
}

fn main() {
    let mut largest_factor = 0;

    for n in 2..((600_851_475_143u64 as f32).sqrt().ceil() as u64) {
        if is_factor(600_851_475_143u64, n) && is_prime(n) {
            largest_factor = n;
        }
    }

    println!("{}", largest_factor);
}