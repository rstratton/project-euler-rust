fn main() {
    let sum: u32 = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    println!("{}", sum);
}