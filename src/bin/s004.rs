fn is_palindrome(num: u32) -> bool {
    let bytes = num.to_string().into_bytes();
    for i in 0..(bytes.len() / 2) {
        if bytes[i] != bytes[bytes.len()-1-i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut largest_palindrome = 0;

    for n in 100..1000 {
        for m in (n+1)..1000 {
            let product = n * m;
            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    println!("{}", largest_palindrome);
}