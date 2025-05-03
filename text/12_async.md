


```rust
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

async fn prep_coffee_mug() {
    sleep(Duration::from_millis(100)).await;
    println!("Pouring milk..");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured");
    println!("Putting instant coffee..");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put");
}

async fn make_coffee() {
    println!("Boiling kettle..");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("kettle boiled");
    println!("pouring boiled water..");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured");
}

async fn make_toast() {
    println!("putting bread in toaster..");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("bread toasted");
    println!("Buttering toasted bread");
    thread::sleep(Duration::from_secs(5));
    println!("toasted bread buttered");
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    // 1. run one async function
    prep_coffee_mug().await;

    // 2. run some async functions
    let _ = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug();
        let coffee_step = make_coffee();
        let toast_step = make_toast();
        tokio::join!(coffee_mug_step, coffee_step, toast_step);
    })
    .await;

    // 3. run two async block concurrently
    let person_one = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug();
        let coffee_step = make_coffee();
        let toast_step = make_toast();
        tokio::join!(coffee_mug_step, coffee_step, toast_step);
    });

    let person_two = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug();
        let coffee_step = make_coffee();
        let toast_step = make_toast();
        tokio::join!(coffee_mug_step, coffee_step, toast_step);
    });

    let _ = tokio::join!(person_one, person_two);

    // etc
    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}

```
