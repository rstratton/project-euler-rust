fn main() {
    let mut sum = 0;

    for n in 0..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }

    println!("{}", sum);
}