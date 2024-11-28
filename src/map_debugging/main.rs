fn fizzbuzz_3(max: i32) {
    let ret_range = 1..=max;
    println!("range: {:?}", ret_range);
    let ret_iter = ret_range.into_iter().map(|i| match (i % 3, i % 5) {
        (0, 0) => format!("{} - FizzBuzz\n", i),
        (0, _) => format!("{} - Fizz\n", i),
        (_, 0) => format!("{} - Buzz\n", i),
        (_, _) => "".to_string(),
    });
    println!("iterator: {:?}", ret_iter);
    let ret_collect = ret_iter.collect::<Vec<String>>();
    println!("after collect: {:?}", ret_collect);
    let ret_final = ret_collect.join("");
    println!("after join: {:?}", ret_final);
}

fn main() {
    fizzbuzz_3(41);
}
