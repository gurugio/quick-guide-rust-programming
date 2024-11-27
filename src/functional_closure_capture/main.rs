fn fizzbuzz_fn<FA, FB>(fizzfn: FA, buzzfn: FB)
where
    FA: Fn(i32) -> bool,
    FB: Fn(i32) -> bool,
{
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
    let fizz = 3;
    let buzz = 5;
    fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
}
