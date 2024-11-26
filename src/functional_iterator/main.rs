struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn main() {
    let mut fib_iter = Fibonacci { curr: 0, next: 1 };
    println!("next returns: {}", fib_iter.next().unwrap());
    println!("next returns: {}", fib_iter.next().unwrap());
    println!("next returns: {}", fib_iter.next().unwrap());

    for i in fib_iter {
        println!("In a loop: {}", i);
        if i > 100 {
            // try again after removing this break condition
            break;
        }
    }
}
