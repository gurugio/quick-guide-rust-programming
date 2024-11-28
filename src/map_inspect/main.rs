fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .inspect(|s| println!("map returns '{}'", s))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}

fn main() {
    fizzbuzz_3(41);
}
