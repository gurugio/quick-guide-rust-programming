struct Fibonacci {
    curr: u32,
    next: u32,
    // Solution
    // buffer: Vec<u32>,
    // counter: usize,
}

// Solution
/*impl Fibonacci {
    fn nth(&self, index: usize) -> u32 {
        self.buffer[index]
    }
}*/

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Solution
        /*self.buffer.push(current);
        self.counter += 1;*/
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_iterator() {
        let mut fib_iter = Fibonacci { curr: 0, next: 1 };

        // Solution
        /*let mut fib_iter = Fibonacci {
            curr: 0,
            next: 1,
            buffer: Vec::new(),
            counter: 0,
        };*/

        for _ in 1..=10 {
            let _ = fib_iter.next().unwrap();
        }

        // Exercise
        //assert_eq!(5, fib_iter.nth(5));
    }
}
