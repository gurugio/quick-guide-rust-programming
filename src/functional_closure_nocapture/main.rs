fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBizz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn main() {
    fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}
