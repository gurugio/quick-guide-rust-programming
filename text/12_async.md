# Asynchronous programming 기본 개념 소개

## Asynchronous programming이 무엇인가?

여러가지 서로다른 태스크를 동시에 실행하는 방식이 제가 알기로 3가지 정도가 있습니다.
첫번째가 병렬 프로그래밍 (Parallel programming), 동시성 프로그래밍 (Concurrent programming) 그리고 비동기 프로그래밍 (Asynchronous programming) 입니다.
어떤 차이가 있는지를 간략하게 이야기해보자면
1. 병렬 프로그래밍: 가장 대표적인 것이 GPU연산입니다. 같은 연산을 다른 데이터를 가지고 실행하는데 물리적으로 여러 연산들을 동시에 실행하는 것입니다.
2. 동시성 프로그래밍: 1개 코어를 가진 CPU가 여러가지 쓰레드를 실행하면 사람에게는 마치 모든 쓰레드가 동시에 실행되는 것가 같이 보일 것입니다. 하지만 엄밀히는 각 쓰레드가 동시에 실행되는 것이 아니라 아주 빠르게 스위칭되기 때문에 마치 동시에 실행되는 것처럼 보이는 것입니다. 이렇게 여러 태스크를 스위칭해가면서 실행하는 것을 동시성 프로그래밍입니다. 쓰레드를 사용하는 것이 대표적인 예이고, 쓰레드를 사용하기 위해 다양한 락이나 스케줄링 기법들이 필요하게됩니다.
3. 비동기 프로그래밍: 어떤 태스크를 실행하도록 명령한 후 해당 태스트가 실행되는 것을 기다릴 필요없이 다른 작업을 할 수 있는 것이 비동기 프로그래밍입니다. 실 생활에서도 비동기로 처리되는 것들이 많습니다. 은행에 대출 서류를 제출한 후에 대출을 받을 때까지 우리가 은행에서 기다릴 필요가 없습니다. 우리는 은행에서 대출 준비가 완료되었다는 연락을 받을 때까지 우리 일상 생활을 계속 하면 됩니다. 그러다가 대출 준비가 완료되었다는 연락을 받으면, 그 다음에 대출받은 돈을 가지고 해야 할 일을 하면 됩니다.

거의 모든 운영체제와 언어들이 각자 나름대로의 비동기 처리를 위한 기법을 제공하고 있습니다.
최신 리눅스 커널은 epoll이나 io_uring이라는 비동기적인 파일 입출력 처리 기법을 제공합니다.
보통 프로그래밍을 처음 시작하는 분들은 read/write 시스템 콜을 사용하셨을 것입니다.
이런 전통적인 시스템 콜은 읽기나 쓰기위한 데이터가 모두 처리가 끝나야 함수가 끝나고 함수의 반환값을 받게됩니다.
하지만 epoll이나 io_uring은 파일에 어떤 데이터를 읽고 쓰라는 명령을 내린 후 다른 일을 하면서 데이터 처리가 끝나기를 기다릴 수 있습니다.
그래소 비동기적인 파일 입출력이라고 부릅니다.

쓰레드를 이용하는 동시성 프로그래밍과 크게 다르지 않아보일 수도 있습니다.
비동기적으로 실행할 태스크가 있을 때마다 쓰레드를 만들어서 태스크를 실행하고 쓰레드를 없애면, 비동기 처리와 같아 보일 수 있습니다.
하지만 크게 2가지 관점을 생각해봐야합니다.
1. 쓰레드로 실행하는 비동기 처리는 프로세서의 갯수까지의 태스크만 동시에 실행할 수 있습니다. 프로세서가 8개면 최대 8개의 태스크가 동시에 실행될 수 있고, 그 외의 쓰레드는 기다릴 수밖에 없습니다. 비동기 프로그래밍을 하게되면 이러한 제약없이 더 많은 태스크를 동시에 실행할 수 있습니다.
2. 쓰레드를 생성하고 제거하는 것이 프로그램에 큰 부담이 됩니다. 특히 아주 작은 태스크를 여러개 실행해야할 때 쓰레드를 만들고 제거하는 컨텍스트 스위칭에 들어가는 시간이 태스크의 실행 시간에 비해 점점 더 커지게됩니다. 비동기 프로그래밍을 하게되면 쓰레드를 관리하는 비용없이 순수하게 태스크만 처리할 수 있으므로 더 빨리 원하는 처리를 할 수 있습니다.

## Tokio 소개

비동기 태스크를 만들고 실행하기 위해서 가장 먼저 필요한 것은 Tokio라는 크레이트를 추가하는 것입니다.
아래와같이 `cargo add tokio --features=full` 명령으로 Cargo.toml에 추가할 수도 있고, 직접 Cargo.toml 파일에 입력해서 추가할 수도 있습니다.

```
$ cargo add tokio --features=full

$ cat Cargo.toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.44.2", features = ["full"] }
```

Tokio는 비동기 태스크를 실행하기 위한 런타임 (Runtime)을 제공하는 크레이트입니다.
비동기 태스크들이 여러개 있을때 어느 것을 언제 실행할 것인지를 판단해야합니다.
어떤 태스크는 IO를 기다리고 있으므로 아직 실행할 수 없으니 실행을 중단켜야합니다.
어떤 태스크는 네트워크에서 데이터를 기다리다가 데이터가 동작했으니 이제 실행을 시작해야합니다.
이런 태스크 관리를 해주는 것이 런타임입니다.

마치 운영체제가 프로세스나 쓰레드를 위한 스케줄러와 메모리 관리자 등을 가지고 있는 것처럼 런타임도 비동기 태스크들을 관리하기 위한 스케줄러와 메모리 할당자 등을 가지고 있습니다.
그리고 리눅스 운영체제가 프로세스를 관리하기 위한 구조체 task_struct를 가지고 있는 것과 같이, 비동기 태스크들도 각 태스크를 관리하기 위한 Future라는 트레이트가 있습니다.
우리가 함수 하나를 비동기 태스크로 만들면 Future 구현이 자동으로 생성됩니다.
(물론 필요하면 Future트레이트를 직접 구현할 수도 있지만, 이 책에서는 기본 사용법에 대해서만 이야기하겠습니다.)
그리고 런타임은 Future 트레이트의 트레이트 객체들의 리스트를 가지고 있어서, 우리가 만든 함수들을 실행하거나 대기시키는 등의 관리를 하게됩니다.

Tokio외에도 몇가지 런타임 구현체가 있습니다.
하지만 최근에는 Tokio가 사실상 표준으로 자리잡았기 때문에 Tokio만 이야기하겠습니다.

## async, await 키워드와 future 크레이트

비동기로 실행되는 함수라는 것이 무엇인지 알아보기 위해 일단 비동기 함수를 한번 만들어보겠습니다.
그리고 일단 다음 예제를 한번 실행해봅니다.

```rust
use std::time::Duration;

async fn task_one() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-one");
    1
}

async fn task_two() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    let future_one = task_one();
    let future_two = task_two();
    println!("Futures are ready but not start yet");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let v1 = future_one.await;
    println!("task_one is finished");
    let v2 = future_two.await;
    println!("task_two is finished");
    println!("v1={} v2={}", v1, v2);
}
```
```bash
 % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/bin-example`
Futures are ready but not start yet
Start task-one
Finish task-one
task_one is finished
Start task-two
Finish task-two
task_two is finished
v1=1 v2=2
```

task_one과 task_two는 비동기(async) 함수입니다.
함수 선언부를 보면 async 키워드가 가장 앞에 있습니다.
우리가 지금까지 함수를 호출하는 방식대로 생각해보면 take_one 함수가 호출된 후 task_two 함수가 호출됩니다.
그리고 "Futures are ready but not start yet"이라는 메세지가 출력되어야 합니다.
그런데 실행 결과를 보면 가장 먼저 실행되는 것이 "Futures are ready but not start yet" 메세지 출력입니다.
그리고 task_one 함수가 실행되고 그 다음 task_two 함수가 실행됩니다.
이로서 일단 async 키워드가 있는 비동기 함수는 실행되는 시점이 일반적인 동기 함수 (async 키워드가 없는 함수)와 전혀 다르다는 것을 알 수 있습니다.
그리고 await라는 키워드가 비동기 함수의 실행과 연관이 있을 것이라는 짐작을 할 수 있습니다.

async 키워드는 지정된 함수 혹은 블럭이 비동기로 처리된다는 것을 나타내는 키워드입니다.
함수 이름에는 `async fn 함수이름`과 같이 사용되고 블럭에도 `async { ... }`과 같이 사용할 수 있습니다.
task_one과 task_two는 async 키워드가 사용되었으므로 비동기 함수입니다.
그리고 task_one이나 task_two를 호출하면 그 결과값은 `dyn Future<Output=i32>` 타입입니다.
Future라는 트레이트를 구현한 트레이트 객체를 반환하는데 Output이라는 타입은 i32으로 정의된 트레이트 구현이라는 의미가 됩니다.

----
참고로 future_one과 future_two 변수의 타입은 Opaque 타입입니다.
사용자가 타입을 직접 지정할 수 없다는 의미입니다.
`let future_one: dyn Future<Output=i32> = task_one();` 이렇게 타입을 직접 지정할 수 없습니다.
future_one 변수와 future_two 변수는 타입을 직접 지정하지않고 사용할 수밖에 없습니다.
물론 Tokio 내부적으로는 타입이 정의되어있을 것입니다.
하지만 그 타입을 사용자가 볼 수 있도록 공개하지 않았으므로 우리는 위 예제와 같이 타입을 지정하지않고 사용할 수밖에 없습니다.

----

비동기 함수를 호출하면 Future 트레이트의 트레이트 객체를 반환합니다.
그리고 함수 자체는 실행을 시작하지 않습니다.
Future는 비동기 연산 작업을 관리하기 위해 사용되는 구조체입니다.
특히 아직 실행이 완료되지 않은 연산 작업을 관리합니다.
만약 실행이 완료되었다면 그냥 함수의 결과값이 나옵니다.
task_one 이라는 함수의 결과값이 i32이지만, 비동기 연산이 아직 끝나지 않았으므로 Future 트레이트의 트레이트 객체가 반환된 상태입니다.
그리고 함수가 호출된 시점에서는 이 함수가 시작되지 않습니다.
이 함수가 아직 시작되지 않고, main 함수를 계속 실행합니다.
그래서 비동기 연산이 되는 것입니다.
비동기 함수가 실제로 실행을 시작하는 지점은 future_one변수에서 await 키워드가 사용될 때 입니다.
await은 메소드가 아니므로 키워드라고 부릅니다.
그래서 함수 호출을 의미하는 ()가 없습니다.

await은 비동기 함수의 실행을 시작하는 것뿐 아니라 함수가 종료할 때까지 기다리는 역할도 합니다.
await 키워드가 사용되어서 비동기 함수가 완료가 되었으므로, 이제서야 i32 타입의 결과값을 얻게됩니다.
그래서 예제를 실행하면 항상 "Finish task_one" 메세지가 출력된 후 "task_one is finished" 메세지가 출력됩니다.
마찬가지로 task_two 함수가 반환한 future_two에 await 키워드가 사용되었고, "Finish task-two" 메세지가 출력된 후에 "task_two is finished" 메시지가 출력됩니다.
비동기 프로그래밍을 이야기하고 있는데 사실 함수의 실행 시점만 조금 늦춰졌을 뿐, 전혀 비동기적으로 실행되는 부분이 없다는게 보이시나요?
이 await 키워드를 사용하는 것은 태스크가 실행되고 종료될때까지 기다린다는 것이므로 사실 비동기적 프로그래밍은 아닙니다.
단지 async 키워드를 사용한 비동기 함수를 특정 시점에 강제로 시작하고 반환값을 받아오는 기본적인 사용법을 설명한 것 뿐입니다.

그 외에 main함수를 보면 3가지 색다른게 보입니다.

1. `#[tokio::main]`: 이 매크로 속성은 tokio 크레이트에게 main함수에서 tokio가 비동기 태스크들을 실행할 수 있는 런타임을 실행할 것을 알려줍니다. main함수가 아닌 일반 함수가 런타임을 실행한다면 일반 함수에도 사용할 수 있는 속성입니다.
2. `async fn main`: 비동기 함수를 실행하는 함수도 비동기 함수가 되어야합니다. 즉 await 키워드를 사용해서 비동기 함수를 실행시키는 함수는 자기 자신도 async 함수가 되어야합니다.

예를 들어 다음 func_not_async 함수는 비동기 함수가 될 필요가 없습니다.
await 키워드로 비동기 함수를 실행시키지 않았기 때문입니다.
```rust
fn func_not_async() {
    let future_one = task_one();
}
```
하지만 다음과 같이 await 키워드를 사용하는 함수 func_async는 비동기 함수가 되어야합니다.
```rust
async fn func_async() {
    task_one().await;
}
```

## async 함수들을 async 하게 실행시키기

우리는 이전 예제에서 각 비동기 함수를 순서대로 동기 함수인 것 같이 호출해보았습니다.
이번에는 비동기 함수들을 정말 비동기 방식으로 호출하는 예제를 만들어보겠습니다.

```rust
use std::time::Duration;

async fn task_one() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-one");
    1
}

async fn task_two() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    let future_one = task_one();
    let future_two = task_two();
    println!("Futures are ready but not start yet");
    tokio::time::sleep(Duration::from_secs(1)).await;
    let (v1, v2) = tokio::join!(future_one, future_two);
    println!("v1={} v2={}", v1, v2);
}
```
```bash
 % cargo run
   Compiling bin-example v0.1.0 (/Users/user/study/bin-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.32s
     Running `target/debug/bin-example`
Futures are ready but not start yet
Start task-one
Start task-two
Finish task-two
Finish task-one
v1=1 v2=2
```

예제를 보면 이전에는 join이라는 매크로를 사용하고 있습니다.
join은 여러개의 future 혹인 비동기 블럭을 입력을 받아서 하나의 비동기 태스크에서 실행하는 것입니다.
따로 비동기 태스크를 실행하지는 않았지만 main이 가지는 기본 비동기 태스크에서 두 함수를 실행합니다.

참고로 비동기 태스크와 비동기 블럭이 헷갈릴 수도 있습니다만 비동기 태스크를 하나의 운영체제로 생각하고, 비동기 블럭(함수)를 하나의 프로세스라고 생각할 수 있습니다.
비동기 태스크 하나에 여러개의 비동기 블럭(함수)들이 실행될 수 있고, 비동기 태스크는 여러 비동기 블럭을 스케줄링하면서 실행합니다.
위 예제는 프로그램에 하나의 비동기 태스크만 실행되고, 2개의 비동기 함수가 하나의 태스크 안에서 실행되는 것입니다.
당연히 필요에 따라 여러개의 비동기 태스크가 실행될 수 있겠지요.
하지만 서로 다른 비동기 태스크에서 실행되는 비동기 함수는 서로 스케줄링 되는 기준이 달라지는 것입니다.

비동기 함수들이 비동기적으로 실행되는 것을 좀 더 잘 이해하기 위해 한가지 실험을 더 해보겠습니다.
사실 비동기 함수에서 std::thread::sleep이 아니라 tokio::time::sleep를 사용하고 있다는 것을 눈치채셨나요?
둘 다 1초를 기다리는 함수인데 어떤 차이가 있을까요?

아래 예제는 대기 시간을 좀 더 잘 확인하기 위해 5초로 바꾸고, std::thread::sleep이 아니라 tokio::time::sleep를 비교해본 예제입니다.

```rust
use std::time::Duration;

async fn task_one_async_sleep() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Finish task-one");
    1
}

async fn task_two_async_sleep() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Finish task-two");
    2
}

async fn task_one_thread_sleep() -> i32 {
    println!("Start task-one");
    std::thread::sleep(Duration::from_secs(5));
    println!("Finish task-one");
    1
}

async fn task_two_thread_sleep() -> i32 {
    println!("Start task-two");
    std::thread::sleep(Duration::from_secs(5));
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    let start_time = std::time::Instant::now();
    let future_one = task_one_thread_sleep();
    let future_two = task_two_thread_sleep();
    let (_v1, _v2) = tokio::join!(future_one, future_two);
    let elapsed = start_time.elapsed();
    println!("Thread-sleep: {} seconds", elapsed.as_secs());

    let start_time = std::time::Instant::now();
    let future_one = task_one_async_sleep();
    let future_two = task_two_async_sleep();
    let (_v1, _v2) = tokio::join!(future_one, future_two);
    let elapsed = start_time.elapsed();
    println!("Async-sleep: {} seconds", elapsed.as_secs());
}
```
```bash
$ cargo run
   Compiling bin-example v0.1.0 (/Users/user/study/bin-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/bin-example`
Start task-one
Finish task-one
Start task-two
Finish task-two
Thread-sleep: 10 seconds
Start task-one
Start task-two
Finish task-two
Finish task-one
Async-sleep: 5 seconds
```

첫번째로 std::thread::sleep을 사용했을 때 2개의 비동기 함수를 실행한 총 시간이 10초가 나왔습니다
그 다음으로 tokio::time::sleep을 사용하면 5초가 걸립니다.
그리도 또 다른 차이도 있습니다.
std::thread::sleep를 사용하면 2개의 비동기 함수를 join 매크로를 이용해서 호출했음에도 불구하고 비동기로 실행된게 아니라 순서대로 실행되고 있습니다.
"Finish task-one" 메세지가 출력된 후에 "Start task-two" 메세지가 출력되는 것을 보면 2개의 비동기 함수가 동시에 실행되는 것이 아님을 알 수 있습니다.
왜 이런 차이가 생길까요?
왜 굳이 std::thread:sleep라는 시간 대기 함수가 있는데도 tokio에서는 따로 같은 일을 하는 함수를 제공하고 있는걸까요?

근본적인 차이점을 이야기하자면 std::thread:sleep은 쓰레드의 슬립이라는 이름과 같이 현재 쓰레드의 실행을 중단합니다.
그리고 tokio::time:sleep은 현재 쓰레드나 비동기 태스크의 실행을 중단하지 않고, 비동기 블럭(함수)의 실행만 중단합니다.
하나의 비동기 블럭(함수)의 실행만 중단되면, 다른 비동기 블럭(함수)를 실행할 수 있겠지요.
하지만 현재 쓰레드가 중단되면, 쓰레드에서 실행중인 런타임 자체가 중단되는 것이므로 모든 비동기 블럭(함수)의 실행이 중단되는 것입니다.
그래서 위의 예제를 보면 task_one함수에서 쓰레드 슬립 함수를 사용하면 런타임이 중단되어서 task_two 함수가 실행이 안되게 됩니다.
모든 비동기 블럭(함수)의 실행을 중단해야되는 특수한 경우를 제외하고 일반적으로 하나의 함수를 잠시 중단시키려는 경우에는 반드시 Tokio에서 제공하는 sleep함수를 사용해야됩니다.

sleep함수이외에도 tokio에서 제공하는 tokio::fs 등의 크레이트들이 있습니다.
어떤 차이인지 아시겠지요?
표준 크레이트에 있는 파일 입출력은 파일을 처리를 하던 중간에 잠들 수가 있습니다.
그러면 쓰레드 전체가 잠들어서 런타임의 실행도 중단됩니다.
비동기적으로 실행되는 코드를 만들기 위해서는 외부 크레이트들도 비동기적으로 실행되는 크레이트를 사용해야합니다.
만약 비동기적으로 실행되는 크레이트가 아닌데 꼭 사용해야한다면 별도의 비동기 태스크를 만드는 등 비동기로 실행될 수 있도록 해주어야합니다.

상황에 따라 다르겠지만 예를 들면 다음과 같이 2개의 비동기 태스크를 실행할 수 있습니다.
하나의 비동기 태스크에는 동기 크레이트 thread::sleep을 사용하는 함수들을 실행하고, 다른 비동기 태스크에는 비동기 함수들만 실행하고 있습니다.

```
use std::time::Duration;

async fn task_one_async_sleep() -> i32 {
    println!("Start async-task-one");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Finish async-task-one");
    1
}

async fn task_two_async_sleep() -> i32 {
    println!("Start async-task-two");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Finish async-task-two");
    2
}

async fn task_one_thread_sleep() -> i32 {
    println!("Start thread-task-one");
    std::thread::sleep(Duration::from_secs(5));
    println!("Finish thread-task-one");
    1
}

async fn task_two_thread_sleep() -> i32 {
    println!("Start thread-task-two");
    std::thread::sleep(Duration::from_secs(5));
    println!("Finish thread-task-two");
    2
}

#[tokio::main]
async fn main() {
    let start_time = std::time::Instant::now();

    let one = tokio::task::spawn(async {
        let future_one = task_one_thread_sleep();
        let future_two = task_two_thread_sleep();
        let (_v1, _v2) = tokio::join!(future_one, future_two);
    });
    let two = tokio::task::spawn(async {
        let future_one = task_one_async_sleep();
        let future_two = task_two_async_sleep();
        let (_v1, _v2) = tokio::join!(future_one, future_two);
    });
    let _ = tokio::join!(one, two);

    let elapsed = start_time.elapsed();
    println!("Sleep: {} seconds", elapsed.as_secs());
}

```
```bash
 % cargo run
   Compiling bin-example v0.1.0 (/Users/user/study/bin-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/bin-example`
Start thread-task-one
Start async-task-one
Start async-task-two
Finish async-task-two
Finish async-task-one
Finish thread-task-one
Start thread-task-two
Finish thread-task-two
Sleep: 10 seconds
```

실행 결과를 보면 thread_task_one 함수가 실행된 후 해당 비동기 태스크는 잠듭니다.
하지만 별도의 비동기 태스크는 실행되고 있기 때문에 async_task_one, async_task_two 함수가 모두 실행되는 것을 볼 수 있습니다.
그래서 총 10초의 시간에 모든 태스크가 실행될 수 있습니다.

## async 테스트

비동기 함수들을 만들었으면 만든 함수들을 테스트해봐야됩니다.
기존 테스트 프레임워크는 비동기 런타임이 아니므로 비동기 함수들을 호출 할 수 없습니다.
그래서 Tokio는 비동기 테스트 프레임워크도 지원하고 있습니다.

아래 예제는 2개의 비동기 함수 task_one, task_two를 테스트 케이스에서 호출하는 예제입니다.

```rust
use std::time::Duration;

async fn task_one() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-one");
    1
}

async fn task_two() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    let future_one = task_one();
    let future_two = task_two();
    println!("Futures are ready but not start yet");
    tokio::time::sleep(Duration::from_secs(1)).await;
    let (v1, v2) = tokio::join!(future_one, future_two);
    println!("v1={} v2={}", v1, v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_func() {
        let v = task_one().await;
        assert_eq!(v, 1);
    }

    #[tokio::test]
    async fn test_async_task() {
        let future_one = task_one();
        let future_two = task_two();
        let (v1, v2) = tokio::join!(future_one, future_two);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
```

main함수는 이전에 "async 함수들을 async 하게 실행시키기" 챕터에서 사용한 그대로입니다.
테스트 함수를 보면 함수 이름앞에 async 키워드를 추가하고, main함수에 `#[tokio::main]` 속성을 추가한 것과 같이 `#[tokio::test]` 속성을 추가한 것을 볼 수 있습니다.
그외에 각 테스트 코드를 보면 이전에 만들어본 것과 동일합니다.
test_async_func 테스트는 하나의 비동기 함수를 실행한 테스트이고, test_async_task 테스트는 main함수와 동일하게 2개의 비동기 함수를 동시에 실행하는 테스트입니다.
main함수에서 비동기 함수들을 호출하는 것과 전혀 다를게 없다는 것을 알 수 있습니다.

## 좀더 나아가기 위해 할 일

러스트를 이용한 비동기 프로그래밍은 최근 매우 중요해지고 있습니다.
멀티 코어를 활용한 멀티 쓰레드 프로그래밍으로도 최신 AI 제품등에 필요한 충분한 성능을 내지 못하고 있기 때문입니다.
이 책에서 하나의 챕터로 기본부터 실무에 필요한 내용까지 다루기에는 책의 분량이나 제 능력에 한계가 있기 때문에 아쉽지만 async, await, Future의 기본 사용법을 소개하는 정도로 마무리하려고 합니다.
좀더 러스트의 비동기 프로그래밍을 공부하고 싶으신 분들은 최근 좋은 원서들이 나오기 시작했으니 찾아보시기 바랍니다.
직접 Future 트레이트를 구현해보거나, 런타임을 직접 구현해보면서 Tokio 크레이트의 작동 원리를 이해하고 비동기 프로그래밍에 익숙해질 수 있도록 설명해주는 책들도 있습니다.
그리고 파이썬같이 좀더 접근하기 쉬운 언어로 구현된 프로젝트를 러스트로 재구현해보는 것도 좋은 출발점이 될 것이라 생각합니다.
