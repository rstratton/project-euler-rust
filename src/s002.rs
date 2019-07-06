struct FibIter {
    curr: u32,
    prev: u32
}

impl FibIter {
    fn new() -> FibIter {
        FibIter { curr: 1, prev: 0 }
    }
}

impl Iterator for FibIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = next;
        Some(self.prev)
    }
}

fn main() {
    let solution : u32 = FibIter::new().filter(|n| n % 2 == 0)
                                       .take_while(|n| n <= &4_000_000u32)
                                       .sum();
    println!("{}", solution);
}