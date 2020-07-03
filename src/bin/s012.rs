fn num_divisors(n: u128) -> u128 {
    let mut count: u128 = 0;
    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut tri_num: u128 = 0;
    let mut nat_num: u128 = 1;

    let answer = loop {
        tri_num += nat_num;
        if num_divisors(tri_num) > 500 {
            break tri_num;
        }
        nat_num += 1;
    };
    println!("{}", answer);
}