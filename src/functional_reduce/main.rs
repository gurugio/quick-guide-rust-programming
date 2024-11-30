fn fizzbuzz_fn<FA, FB>(fizzfn: FA, buzzfn: FB)
where
    FA: Fn(i32) -> bool,
    FB: Fn(i32) -> bool,
{
    let fizz = (1..=50)
        .into_iter()
        .filter(|i| fizzfn(*i))
        .map(|i| format!("{} is Fizz\n", i))
        .reduce(|a, b| a + &b)
        .unwrap();
    println!("{}", fizz);
    let buzz = (1..=50)
        .into_iter()
        .filter(|i| buzzfn(*i))
        .map(|i| format!("{} is Buzz\n", i))
        .reduce(|a, b| a + &b)
        .unwrap();
    println!("{}", buzz);
    let fizzbuzz = (1..=50)
        .into_iter()
        .filter(|i| fizzfn(*i) && buzzfn(*i))
        .map(|i| format!("{} is Fizz and Buzz\n", i))
        .reduce(|a, b| a + &b)
        .unwrap();
    println!("{}", fizzbuzz);
}

fn main() {
    fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}
