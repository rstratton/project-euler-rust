use std::fs::File;
use std::io::prelude::*;

fn read_file() -> String {
    let mut file = File::open("src/s008.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents: Vec<&str> = contents.lines().collect();
    let contents = contents.join("");
    contents
}

fn main() {
    let digits = read_file();
    let mut greatest_product = 0;
    for start in 0..(digits.len() - 12) {
        let product: u64 = digits[start..(start + 13)]
            .chars()
            .map(|c| (c.to_digit(10).unwrap() as u64))
            .product();
        if product > greatest_product {
            greatest_product = product
        }
    }
    println!("{}", greatest_product);
}
