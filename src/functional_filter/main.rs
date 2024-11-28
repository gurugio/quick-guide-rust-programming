fn fizzbuzz_fn<FA, FB>(fizzfn: FA, buzzfn: FB)
where
    FA: Fn(i32) -> bool,
    FB: Fn(i32) -> bool,
{
    let _ = (1..=50)
        .into_iter()
        .filter(|i| fizzfn(*i))
        .map(|i| println!("{} is Fizz", i))
        .collect::<()>();
    let _ = (1..=50)
        .into_iter()
        .filter(|i| buzzfn(*i))
        .map(|i| println!("{} is Buzz", i))
        .collect::<()>();
    let _ = (1..=50)
        .into_iter()
        .filter(|i| fizzfn(*i) && buzzfn(*i))
        .map(|i| println!("{} is Fizz and Buzz", i))
        .collect::<()>();
}

fn main() {
    fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}
