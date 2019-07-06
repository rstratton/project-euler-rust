fn main() {
    let sum_of_squares: u32 = (1..=100).map(|n| n * n).sum();
    let sum_of_100: u32     = (1..=100).sum();
    let square_of_sum: u32  = sum_of_100 * sum_of_100;
    println!("{}", square_of_sum - sum_of_squares);
}