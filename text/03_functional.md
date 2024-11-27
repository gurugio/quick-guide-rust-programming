# 함수형 프로그래밍

## 함수형 프로그래밍 소개

러스트는 기본적으로 절차형 프로그래밍 언어(Imperative programming language)입니다. 함수형 프로그래밍 언어의 장점들을 도입한 것 뿐이고, 러스트가 하스켈같은 순수 함수형 프로그래밍 언어라고 말할 수는 없습니다. 함수형 프로그래밍이라는 것은 무엇이고, 왜 러스트 언어에 함수형 언어의 패러다임을 도입했을까요?

위키피디아에 따르면 "패러다임(영어: paradigm)은 어떤 한 시대 사람들의 견해나 사고를 근본적으로 규정하고 있는 테두리로서의 인식의 체계, 또는 사물에 대한 이론적인 틀이나 체계를 의미하는 개념이다." (출저: https://ko.wikipedia.org/wiki/패러다임) 이라고 합니다. 어려운 설명이긴 합니다만 제 생각에는 프로그램을 어떻게해야 더 잘 만들 수 있을까, 프로그래밍을 어떻게 더 잘할 수 있을까를 고민하는 여러가지 방식들이 있고, 그 중 하나가 함수형 프로그래밍 패러다임이라고 생각합니다. 참고로 말씀드리면 저는 Scheme나 Scala언어를 일이년씩 책과 온라인 강의로 공부한 경험이 있습니다. 좋은 개발자가 되기 위해서 한번쯤은 깊게 빠져 볼만한 패러다임이라고 생각합니다. 제가 프로그래밍 언어라고 말씀드리지않고 패러다임이라고 말씀드리는 것은 특정 언어가 좋다는 접근보다는 함수형 프로그래밍 패러다임 그 자체를 접하기 위해서 언어를 도구로 활용하시길 바라는 마음에서 패러다임을 강조해봤습니다.

함수형 언어의 역사나 종류등 함수형 언어 자체에 대한 설명은 다른 더 좋은 자료를 참고해주시기 바랍니다. 특정 언어가 함수형 언어가 되기 위한 조건들이 있습니다. 그중 러스트에서도 적용되는 조건들만 설명해보겠습니다.

1. 순수 함수를 지원한다

순수 함수는 같은 값을 넣으면 항상 같은 반환값을 얻을 수 있고, 외부의 어떤 데이터도 수정하지 않는 함수입니다. 그냥 설명만 보면 어렵지만 다음과 같은 예제를 보면 쉽게 그 차이를 알 수 있습니다.

```rust
fn imperative_add_one(x: &mut i32) {
    *x += 1;
}

fn functional_add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let mut y = 1;
    imperative_add_one(&mut y);
    println!("y={}", y);
    
    let y = 1;
    println!("y={}", functional_add_one(y));
}
```
imperative_add_one함수는 비순수 함수입니다. imperative_add_one의 외부에 있는 y라는 인자를 받아서 값을 수정하기 때문입니다. 그에 반해 functional_add_one함수는 순수 함수입니다. 어떤 인자를 받아도 수정하지 않기 때문입니다. 그리고 1이라는 값을 가진 인자를 전달하면 언제나 2를 반환합니다. 같은 인자에 대해서 같은 반환값을 얻을 수 있습니다. 이제부터 최대한 많은 함수를 순수 함수로 만드는 연습을 해보는 것도 좋습니다. 왜냐면 순수 함수는 버그를 발생시킬 위험이 적고, 병렬로 실행되어도 아무런 문제도 일으키지 않기 때문입니다. 최대한 순수 함수로 만들 수 있는 코드는 순수 함수로 만들고 불가피한 경우만 상태를 갖는 함수로 만든다면, 버그가 생겼을때 비순수 함수를 먼저 확인해서 좀 더 빨리 문제를 파악할 수 있습니다.

2. 상태를 가지지않거나 변하지 않는 데이터를 사용한다
러스트는 모든 변수가 별도의 표시가 없으면 불변 타입입니다. 또한 함수를 만들때 함수 내부의 코드 중에 마지막 라인이 함수의 반환값이 되는 것도 여러 함수형 언어들이 가지는 특징입니다.

3. 함수가 1급 객체가 되어 고차함수의 인자로 사용되거나 반환값으로 사용될 수 있다
러스트는 함수를 변수에 저장하거나, 변수처럼 다른 함수의 인자나 반환값으로 사용할 수 있습니다. 조금 뒤에 함수 포인터와 클로저를 소개하겠습니다.

그럼 러스트는 왜 함수형 언어의 패러다임을 도입했을까요?

1. 최대한 많은 함수를 순수 함수로 만들고 불변형 데이터를 사용하면 프로그램의 구조를 적절하게 나누기 쉬워집니다. 또 순수 함수만 있는 코드는 항상 예측이 가능하므로 버그를 잡기도 쉬워지고, 한번 만든 함수를 여기저기에서 재사용 하기에도 좋습니다. 

2. 성능에 좋습니다. 다음에 설명할 클로저와 이터레이터를 잘 조합해서 사용한다면, 일정한 코드에 다양한 데이터를 적용해서 사용할 수 있게 됩니다. 그러면 컴파일러 입장에서는 최적화 코드를 만들 여지가 많아집니다.

3. 제가 생각하기에 가장 큰 장점은 코드를 읽기 좋고 간결해진다는 것입니다. 사실 성능이나 프로그램 구조에 대한 것은 개발자가 하기 나름일 수 있지만, 러스트에서 도입한 클로저,  map, 이터레이터 등을 잘 사용한다면 if-else나 for 루프 등을 거의 사용하지 않고 프로그램을 만들 수 있습니다. 개발을 하다보면 보통 버그를 많이 만드는 곳이 바로 if-else와 for루프입니다. 여러가지 조건을 체크하는데 빠진 경우가 있다거나, 루프를 돌면서 빠진 경우가 생기거나, 반대로 실행을 안해야되는 조건을 실행하는
버그는 만들기는 쉬운데, 찾아내기가 어려운 난감한 버그가 되기도 합니다. 

4. 쓰레드나 async를 사용하는 비동기 프로그래밍에 활용하기 좋습니다. 

물론 이런 장점들을 잘 활용하려면 함수형 프로그래밍의 스타일에도 익숙해져야하고, 이터레이터나 map의 특징을 잘 이해해야합니다. 처음에는 동작 방식이 낯설고, 코드를 생각하는 사고 과정이 조금 다르다보니 어렵게 느껴질 수 있습니다. 하지만 연습을 통해 익숙해지면 어느순간 생각보다 어렵지 않았다는 것을 느끼실 것입니다.

## 이터레이터

사실 이터레이터가 함수형 프로그래밍에 속하는가 아닌가에 대한 논쟁은 있습니다. 이터레이터를 구현하기에따라 내부에 상태를 저장하기도하고, 상태가 없기도해서 엄격하게 따지자면 함수형 프로그래밍에 맞는 스타일은 아닙니다. 예를 들어 네트워크로부터 데이터를 받아오는 이터레이터를 만들면 상태 관리가 없을 수 있습니다. 단순히 소켓을 읽기만 하니까요. 하지만 파일에서 데이터를 읽어오게되면 파일의 어디까지 읽었는지를 저장해야되니까 상태 정보를 저장하게됩니다. 네트워크에서 데이터를 받을 때도 몇바이트를 받았는지 저장할 수도 있으니까요. 그런데 중요한건 순수 함수를 만들기위해서 이터레이터가 필요할 때가 있다는 것입니다. 그래서 함수형 프로그래밍을 소개할 때 이터레이터 (어떤 언어들은 제너레이터 Generator라고 부르기도합니다)를 같이 소개합니다. 사실 함수형 프로그래밍 스타일이냐 아니냐 따지는건 러스트 프로그래밍에서 중요한건 아닙니다. 러스트는 함수형 프로그래밍 언어가 아니니까요.

이전 장에서 이터레이터를 사용하는 예제를 여러개 만들어봤습니다. 이터레이터가 대부분의 최신 언어들이 제공하고있는 기능이기 때문에 따로 자세한 설명은 하지 않고 바로 활용했었는데요. 이번에는 이터레이터를 직접 구현해보면서 러스트에서 이터레이트가 어떻게 동작하는지를 알아보겠습니다. 정확하게는 이터레이터를 구현하기 위해서 트레이트라는 기능을 사용해야하는데 다음 장에서 설명할 내용입니다. 이번 장에서는 트레이트 자체에 대한 내용보다는 내가 만든 구조체에 next라는 메소드가 추가된다는 점만 봐주시기 바랍니다. 파이썬으로 이터레이터를 만들어보셨다면 next 메소드가 익숙하실 것입니다. 같은 역할을 한다고 생각하시면 됩니다.

보통 이터레이터라고하면 배열이나 특정 범위의 숫자등 이미 있는 데이터를 순서대로 처리하는 것을 생각하기 마련입니다. 하지만 이터레이터는 사실 지연처리(Lazy evaluation)를 위한 기법입니다. 현재 데이터가 없어도 호출될때마다 새롭게 데이터를 생성할 수 있습니다. 모든 데이터를 미리 다 만들어놓으려면, 시간과 공간이 필요합니다. 데이터를 생성하는 시간동안 프로세서를 사용해야되고 데이터를 저장할 메모리를 차지해야됩니다. 프로그램이 커지고 느려질 수밖에 없습니다. 또 막상 데이터를 다 만들어놨다해도 그 데이터중에 일부만 사용하는 경우도 많고 여러번 사용하지도 않게된다면 자원을 낭비하기만 하는 것입니다.

지연처리에 대표적인 예가 피보나치 수열을 생성하는 것입니다. 러스트의 이터레이터를 이용해서 피보나치 수열을 만드는 예제를 만들어보겠습니다.

```rust
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
```
```bash
$ cargo run --bin functional_iterator
     Running `/Users/user/study/my-rust-book/target/debug/functional_iterator`
next returns: 0
next returns: 1
next returns: 1
In a loop: 2
In a loop: 3
In a loop: 5
In a loop: 8
In a loop: 13
In a loop: 21
In a loop: 34
In a loop: 55
In a loop: 89
In a loop: 144
```

피보나치 수열을 생성하는 코드 자체는 간단합니다. 0과 1의 두가지 초기값을 더해서 다음 수열을 만들기만 하면 됩니다. 다음과 같은 형태의 트레이트 구현에 맞춰서 next라는 메소드를 구현해주기만 하면 됩니다.

```rust
impl Iterator for <구조체이름> {
    type Item = <구조체의 데이터 타입>;

    fn next(&mut self) -> Option<Self::Item> {
        함수 구현
        반환값은 Some(<Item 타입의 값>)가 되어야함
    }
}
```

그러면 러스트의 컴파일러가 자동으로 이터레이터가 가지는 next외의 다른 메소드들과 for 루프에 필요한 코드들을 자동으로 생성해줍니다.

main함수를 보면 Fibonacci 구조체의 객체를 생성하고 next 메소드를 호출합니다.

```rust
fn main() {
    let mut fib_iter = Fibonacci { curr: 0, next: 1 };
    println!("next returns: {}", fib_iter.next().unwrap());
    println!("next returns: {}", fib_iter.next().unwrap());
    println!("next returns: {}", fib_iter.next().unwrap());
```

next 메소드가 호출될 때마다 fib_iter 객체안에 저장하고 있는 2개의 값 curr과 next만을 이용해서 계속 새로운 값을 생성해냅니다.

그 다음으로 fib_iter 객체를 for 루프에서 사용하는 예제가 나옵니다.

```rust
    for i in fib_iter {
        println!("In a loop: {}", i);
        if i > 100 {
            // try again after removing this break condition
            break;
        }
    }
```

이전에 3번까지 수열을 생성했으므로, 루프에서는 4번 수열부터 생성합니다. 

혹시 그럼 이전에 생성했던 수열을 다시 읽어보려면 어떡해야하나 하는 생각을 하셨나요? 예 현재 구현에서는 다음 숫자만 계속 생성하도록 되어있습니다. 메모리 사용을 최소화할 수는 있지만, 만약에 10번째 값까지 생성한 후에 5번째 값이 무엇이었는지 다시 확인해볼 수는 없습니다. 메모리를 좀 더 사용하도록 버퍼를 두면 기능을 만들 수 있겠지요. 연습문제로 해보시기 바랍니다. 

## 함수 포인터


================================ 함수 포인터 예제와 클로저 예제 분리, FnMut 예제 추가할것 ===================================



러스트는 C/C++언어와 같이 함수 포인터를 지원합니다. 다음은 fizzbuzz 함수를 함수 포인터로 구현한 예제입니다.

```rust
// src/function_pointer/main.rs
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

fn fizzcheck(n: i32) -> bool {
    n % 3 == 0
}

fn buzzcheck(n: i32) -> bool {
    n % 5 == 0
}

fn main() {
    fizzbuzz_fn(fizzcheck, buzzcheck);
    fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}
```

```bash
$ cargo run --bin function_pointer
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/function_pointer`
Fizz
Buzz
Fizz
Fizz
Buzz
Fizz
FizzBizz
Fizz
Buzz
Fizz
Fizz
Buzz
......
```

fizzbuzz_fn함수의 인자를 보면 fn이라는 키워드가 있는데 이것이 바로 함수 포인터를 나타내는 키워드입니다. C/C++에서 함수 포인터를 표현할 때

```c
int (*변수이름)(int)
```

위와 같이 사용하는데 함수 포인터라는 타입이 있지만, int나 char처럼 함수 포인터를 가르키는 타입 이름이 없다는 것을 알 수 있습니다. 러스트에서는 명확하게 fn이라는 타입 이름을 붙여서 사용합니다. i32 타입의 인자 한개를 받아서 bool 타입의 결과값을 반환하는 함수의 포인터를 다음처럼 선언할 수 있습니다.

```rust
fn(i32) -> bool
```

사실 함수 포인터는 그냥 특정 코드 영역이 존재하는 메모리 값일 뿐입니다. 함수형 프로그래밍과는 별로 상관이 없습니다. 단지 다음에 소개할 클로저와 비교하기 위해서 소개해봤습니다.

## 클로저

클로저는 이름이 없는 함수입니다. 함수 포인터를 사용할 때도 

위의 예제에서 다음과 같이 클로저를 사용하는 것도 볼 수 있습니다.

```rust
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
```

다른 언어에서 클로저를 사용하는 것과 아주 유사합니다. |와 |사이에 클로저의 인자를 넣고, 그 다음에 클로저의 실행 코드를 적으면 됩니다. 그리고 예제에서와 같이 클로저를 다른 함수의 인자로 전달할 수도 있습니다. fn이라는 키워드가 함수 포인터를 위한 키워드일뿐 아니라 클로저를 위한 키워드도 된다는 것을 알 수 있습니다.

### 다른 변수를 참조할 수 있는 Fn와 FnMut

아주 엄밀하게 말하면 외부 변수를 사용하지않는 클로저는 그냥 익명 함수라고 부르고, 클로저가 아니라고 설명하기도 합니다. 이름이 클로저든 익명 함수이든 어쨌든 굳이 짧은 함수를 여러개 만들 필요를 줄여주니 편리하게 사용만 하면 될뿐입니다. 

어쨌든 위의 예제에서 사용한 클로저들은 외부 변수를 사용하지 않았습니다. 오직 인자로 받은 변수 x와 y만을 사용했습니다. 그럼 다음과 같이 외부 변수를 사용해야하는 경우는 어떨까요? 별다른 수정없이 그대로 컴파일이 될까요?

```rust
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
    let fizz = 3;
    let buzz = 5;
    fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
}
```

```bash
t$ cargo run --bin closure
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0308]: arguments to this function are incorrect
  --> src/closure/main.rs:24:5
   |
24 |     fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
   |     ^^^^^^^^^^^
   |
note: expected fn pointer, found closure
  --> src/closure/main.rs:24:17
   |
24 |     fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
   |                 ^^^^^^^^^^^^^^^^^
   = note: expected fn pointer `fn(i32) -> bool`
                 found closure `{closure@src/closure/main.rs:24:17: 24:20}`
note: closures can only be coerced to `fn` types if they do not capture any variables
  --> src/closure/main.rs:24:25
   |
24 |     fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
   |                         ^^^^ `fizz` captured here
note: expected fn pointer, found closure
  --> src/closure/main.rs:24:36
   |
24 |     fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
   |                                    ^^^^^^^^^^^^^^^^^
   = note: expected fn pointer `fn(i32) -> bool`
                 found closure `{closure@src/closure/main.rs:24:36: 24:39}`
note: closures can only be coerced to `fn` types if they do not capture any variables
  --> src/closure/main.rs:24:44
   |
24 |     fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
   |                                            ^^^^ `buzz` captured here
note: function defined here
  --> src/closure/main.rs:1:4
   |
1  | fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
   |    ^^^^^^^^^^^ -----------------------  -----------------------

For more information about this error, try `rustc --explain E0308`.
error: could not compile `my-rust-book` (bin "closure") due to 1 previous error
```

에러 메세지중에 "closures can only be coerced to `fn` types if they do not capture any variables"라는 에러 메세지가 있습니다. 풀어써보면 fn이라는 타입으로 전달받을 수 있는 클로저는 외부 변수를 사용(클로저에서 외부 변수를 사용하는 것을 캡쳐capture라고 부릅니다)할 수 없다는 의미입니다.

그래서 다음과 같이 고쳐야합니다.

```rust
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
```

```bash
$ cargo run --bin closure
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `/home/gkim/study/my-rust-book/target/debug/closure`
Fizz
Buzz
Fizz
Fizz
Buzz
Fizz
FizzBizz
Fizz
Buzz
......
```

fn이라는 타입이 아니라 Fn이라는 타입을 사용했습니다. Fn 타입은 외부 변수를 사용할 수 있는데, 제약 조건이 외부 변수를 수정하지 않는 불변 참조를 해서 사용한다는 것입니다. 위 예제에서 fizz와 buzz라는 변수를 사용했는데 변수를 수정하지않고 사용했으므로 Fn 타입의 클로저를 사용한 것입니다.

만약 클로저에서 외부 변수를 수정해야된다면 FnMut 타입을 사용하면 됩니다.

그리고 위 예제에서 처음보는 문법이 나왔습니다. <FA, FB>와 where등의 표현식이 처음 소개되었습니다. 이것은 나중에 트레이트trait를 소개할 때 다시 이야기하겠습니다.

## map 메소드

클로저를 사용하는 방법중에 가장 많이 사용하게 되는게 이터레이터의 map 메소드와 같이 사용하는 경우입니다. 배열이나 백터의 이터레이터를 만들고, 이레이터의 map 메소드에 클로저를 사용하는 것입니다. 그리고 Option의 map 메소드와 사용하는 것도 자주 사용되는 방식이니까 이 두가지 경우를 이야기해보려고 합니다.

### 이터레이터의 map 메소드 사용 방법

배열이나 range, 벡터등에서 각 데이터에 접근하기 위한 방법으로 이터레이터를 만들고 for 루프와 같이 사용하는 경우가 많습니다. 그런데 for루프 대신에 map을 사용하는게 더 편리할 때가 많습니다. 그리고 많은 경우에 map을 이용하는게 처리 속도가 더 빠르기도 합니다.

가장 간단한 예를 가지고 시작해보겠습니다.

```rust
// src/map/main.rs
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (_, _) => (),
        }
    }
}

fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}

fn main() {
    fizzbuzz_2(37);
    fizzbuzz_3(41);
}
```

```bash
$ cargo run --bin map
   Compiling my-rust-book v0.1.0 (/Users/user/study/my-rust-book)
    Finished dev [unoptimized + debuginfo] target(s) in 5.44s
     Running `target/debug/map`
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
12 - Fizz
15 - FizzBuzz
18 - Fizz
20 - Buzz
21 - Fizz
24 - Fizz
25 - Buzz
27 - Fizz
30 - FizzBuzz
33 - Fizz
35 - Buzz
36 - Fizz
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
12 - Fizz
15 - FizzBuzz
18 - Fizz
20 - Buzz
21 - Fizz
24 - Fizz
25 - Buzz
27 - Fizz
30 - FizzBuzz
33 - Fizz
35 - Buzz
36 - Fizz
39 - Fizz
40 - Buzz
```

fizzbuzz_2함수는 이전에 match에 대한 설명을 위해서 만들어본 예제입니다. fizzbuzz_2에서 for루프와 match구문을 대신해서 이터레이터와 map메소드를 사용하도록 만든게 fizzbuzz_3함수입니다. 가장 먼저 1부터 max까지의 각 숫자들 반환하는 이터레이터를 만듭니다. 그리고 이터레이터의 map 메소드를 호출합니다. map메소드의 인자로는 이터레이터가 값을 반환할때마다 그 값을 인자로 받아서 실행되는 함수가 들어갑니다. 우리는 함수 대신에 클로저를 전달한 것입니다. 함수형 언어를 경험해보신 분들은 많이 보시던 패턴일 것입니다. 러스트는 함수형 언어의 장점들을 많이 적용한 언어입니다.

일반적인 함수형 언어에 비해 러스트 언어를 사용하기 위해 주의해야할 점이 있는데 마지막에 반드시 collect 메소드를 호출해야한다는 것입니다. map메소드는 반환값으로 이터레이터를 반환합니다. 즉 이터레이터를 받아서 처리하고 또 다른 이터레이터를 반환하는 것이 map이 하는 일입니다. 만약에 collect메소드를 호출하지 않으면 ret 변수에 저장되는 값은 이터레이터가 됩니다. 이터레이터 자체는 사실상 아직 실행이 안된 상태입니다. collect가 호출되는 시점에서 이터레이터가 한단계씩 실행되면서 이제서야 map에 전달된 함수가 실행됩니다. collect는 그렇게 이터레이터에 의해 실행된 함수의 결과 값들을 모아서 벡터를 만들어서  ret변수에 저장하는 것입니다. 위의 fizzbuzz_3함수에서는 최종적으로 생성하는 값이 문자열의 벡터이기 때문에 collect에게 다음과 같이 collect가 반환해서 ret에 저장되어야 할 값의 타입이 문자열의 벡터라는 것을 알려줍니다.

```rust
collect::<Vec<String>>()
```

`<Vec<String>>` 부분이 바로 반환값의 타입을 지정하는 부분입니다. collect라는 메소드는 대부분 벡터를 반환하겠지만, 그 벡터안에 무엇이 들어가야될지는 모릅니다. 개발자가 이렇게 코드레벨에서 직접 지정을 해주어야 벡터안에 들어갈 데이터의 타입에 맞게 실행할 수 있습니다. 가끔은 컴파일러가 자동으로 벡터에 들어가는 데이터의 타입을 추론할 수도 있습니다. 그래서 타입을 지정하지 않아도 빌드가 될 때도 있습니다. 하지만 타입을 지정해주어야하는 경우가 더 많습니다.

러스트 언어의 매뉴얼에는 다음과 같은 예제 코드가 있습니다.

```rust
let a = [1, 2, 3];

let mut iter = a.iter().map(|x| 2 * x);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
```

<https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map>

참고로 assert_eq!는 2개의 인자를 받아서 서로 같지 않으면 프로그램을 종료시키는 매크로입니다. 1, 2, 3이 들어있는 벡터의 이터레이터를 만든 후 map 메소드를 실행한 결과값을 iter라는 변수에 저장했습니다. iter라는 변수가 처음 만들어졌을때는 아직 클로저를 실행하지 않은 것입니다. 하지만 최초로 이터레이터의 next 메소드가 호출되었을 때야 처음으로 1을 클로저에 전달해서 Some(2)라는 값을 얻게 됩니다. 그리고 다음 next 메소드가 호출될때 각각 2와 3을 클로저에 전달해서 Some(4)와 Some(6)이라는 값을 얻게 됩니다. 그리고 마지막으로 이터레이터에 남은 데이터가 없으면 클로저가 실행되지 못하고, 반환값도 None을 반환합니다.

왜 next의 결과값이 Option이 되었는지 이해가 되시나요? 이터레이터가 모든 값을 다 처리하고 더 이상 처리할 값이 없을때를 알려주기 위해 Option을 반환값으로 사용하게되었습니다. 이렇게 결과값이 있을때도 있고 없을 때도 있는 경우를 처리하기 위해 Option이 있는 것입니다. 결과값이 에러가 났기 때문에 없는 것이 아닙니다. 그냥 더 이상 처리할 데이터가 없는 정상적인 경우입니다. 그것이 Result와의 차이점입니다.

이터레이터의 가장 대표적인 메소드 next와 collect를 알아봤습니다. next메소드는 위와같이 이터레이터를 한번씩 실행해주는 메소드이고, 모든 연산을 한꺼번에 실행하고 모든 결과값을 벡터에 담아서 반환하는 메소드가 collect 입니다. 벡터에 데이터가 아주 많은 경우를 생각해보면, 꼭 모든 데이터를 다 처리해야될 필요가 없을 때도 있습니다. 조금씩 나눠서 처리해도되는 경우가 있다면 next를 사용하면 됩니다.

### Option의 map 메소드 사용 방법

이터레이터뿐 아니라 Option 타입도 map메소드를 가지고 있습니다.

```rust
// src/map_option/main.rs
fn main() {
    let some_number = Some(5);
    let none_number: Option<i32> = None;

    let double_some = some_number.map(|x| x * 2);
    let double_none = none_number.map(|x| x * 2);

    println!("Double Some: {:?}", double_some); // Double Some: Some(10)
    println!("Double None: {:?}", double_none); // Double None: None
}
```

```bash
$ cargo run --bin map_option
   Finished dev [unoptimized + debuginfo] target(s) in 0.40s
    Running `target/debug/map_option`
Double Some: Some(10)
Double None: None
```

Some(5)라는 값을 가진 변수와 None 값을 가진 변수가 있습니다. 그리고 각각 map메소드를 호출해주었습니다. 이 예제 소스는 워낙 간단하니까 우리 눈에 변수가 Some타입일지 None타입일지 알 수 있지만, 당연히 보통의 경우에는 어떤 함수의 반환값이 어느 타입일지는 알 수 없습니다. 그러면 매번 패턴 매칭이나 if let을 사용해서 값을 꺼내서 필요한 연산을 해주게 되면 코드가 길어질 것입니다. 코드가 길어진다는 것은 읽기 힘들어지고, 에러가 날 경우도 많아진다는 것입니다. 단순히 성능의 최적화를 위해 코드를 짧게 유지하는게 필요한게 아니라, 읽기 좋고 버그가 적은 코드를 만들기 위해서도 코드를 짧게 유지하는게 좋습니다.

Option의 메소드인 map은 타입이 Some일때는 Some안에 있는 값을 꺼내서 클로저의 인자로 넘겨주고, 클로저의 결과값을 Option타입으로 반환해줍니다. None 타입의 map 메소드는 아무런 처리도 하지않고 None을 그대로 반환해줍니다. 따라서 어떤 변수의 값이 Option타입일때, if let이나 match를 사용할 필요없이, 그 변수를 그대로 다른 함수나 클로저에 전달할 수 있게됩니다.

참고로 Some이나 None 값을 출력하기 위해서 “{:?}” 리터럴을 사용하면 Some인지 None인지를 출력해주니까 디버깅할때 편리합니다.

### Result의 map 메소드 사용 방법

Option과 마찬가지로 Result 또한 map 메소드를 가지고 있습니다.

```rust
// src/map_result/main.rs
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}

fn main() {
    let ok_number = divide(10, 2);
    let error_number = divide(10, 0);

    let double_ok = ok_number.map(|x| x * 2);
    let double_error = error_number.map(|x| x * 2);

    println!("Double Ok: {:?}", double_ok); // Double Some: Ok(5)
    println!("Double Error: {:?}", double_error); // Double Error: Error
}
```

```bash
$ cargo run --bin map_result
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/map_result`
Double Ok: Ok(10)
Double Error: Err("denominator cannot be zero")
```

하는 일도 거의 동일합니다. 값이 있는 경우, 즉 Ok 타입인 경우에는 그 안의 값을 꺼내서 전달받은 클로저를 호출합니다. 만약 값이 Err 타입인 경우에는 아무일도 하지 않고, 자기 자신을 반환합니다. if let이나 match 패턴은 클로저로 처리할 수 없을만큼 코드가 길때 사용하고, 코드가 간단하다면 map을 사용하는게 더 좋겠지요.


### WIP map 메소드를 사용하면서 디버깅하는 방법

1. inspect()
```
fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
}
```
2. 여러단계를 하나씩 쪼개기
```rust
fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .inspect(|x| println("map returns {}", x))
        .collect::<Vec<String>>()
        .join("");
}

let ret = (1..=max)
println!("{:?}", ret);
let ret = ret.into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
print("{:?}", ret);
ret.join("");
print("{:?}", ret);
```


## Option과 Result를 러스트 스타일에 맞게 사용하는 방법

map이 편리하지만 반드시 주의해야 할 점이 하나 있는데 바로 map을 호출하면 객체가 해지된다는 것입니다. 영어로는 consume이라고 표현하는데, 그 의미는 자기 자신의 값을 소비해서 없애버리고 반환값을 생성한다는 것입니다.

다음은 Option의 매뉴얼에 있는 예제 코드입니다.

```rust
let maybe_some_string = Some(String::from("Hello, World!"));
// `Option::map` takes self *by value*, consuming `maybe_some_string`
let maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));
//println!("{:?}", maybe_some_string); // error
```

출처: <https://doc.rust-lang.org/std/option/enum.Option.html#method.map>

maybe_some_string은 소비consume되어버렸으니 map연산을 호출한 이후에는 다시 사용할 수 없는 변수가 됩니다. 마지막줄에서 maybe_some_string의 값을 확인해보려고한다면 빌드 에러가 발생합니다.  map 메소드는 Some(x)에 들어있는 값을 해지하고 Some(y)라는 새로운 값으로 바꾸고 이전 값은 다시 사용할 필요가 없을 때 사용합니다. maybe_some_string이라는 객체가 더 이상 필요하지 않으면 괜찮지만 만약 계속 써야하는 데이터라면 객체가 해지되지 않도록 해야합니다.

다음에는 Option이나 Result의 내부에 있는 값을 해지하지않고 접근하거나 수정할 수 있는 메소드들을 소개하겠습니다.

### Option의 as_ref 메소드

먼저 map을 써도 원본 객체가 해지되지 않으려면 어떡해야할까요? 답은 컴파일러가 이미 알려주고 있습니다. 아래는 위 예제의 마지막 줄을 주석처리하지않고 빌드했을 경우 에러 메세지입니다.

```rust
error[E0382]: borrow of moved value: `maybe_some_string`
    --> main.rs:60:22
     |
56   |     let maybe_some_string = Some(String::from("Hello, World!"));
     |         ----------------- move occurs because `maybe_some_string` has type `Option<String>`, which does not implement the `Copy` trait
57   |     // `Option::map` takes self *by value*, consuming `maybe_some_string`
58   |     let maybe_some_len = maybe_some_string.map(|s| s.len());
     |                          ----------------- ---------------- `maybe_some_string` moved due to this method call
     |                          |
     |                          help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
59   |     assert_eq!(maybe_some_len, Some(13));
60   |     println!("{:?}", maybe_some_string);
     |                      ^^^^^^^^^^^^^^^^^ value borrowed here after move
```

중간에보면 as_ref이나 as_mut 메소드를 호출해서 객체의 레퍼런스를 만든 후에 map 메소드를 호출하라고 알려줍니다. as_ref는 `&Option<T>` (Option의 내부에 T타입의 데이터가 저장된 enum타입을 의미합니다. C++의 템플릿이라고 생각하면 이해가 쉬울 수 있습니다.)을 `Option<&T>`으로 바꾸는 일을 합니다.

일단 한번 고쳐서 실행해보겠습니다.

```rust
let maybe_some_string = Some(String::from("Hello, World!"));
let maybe_some_len = maybe_some_string.as_ref().map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));
println!("{:?}", maybe_some_string);
```

```bash
Some("Hello, World!")
```

우리 눈에 직접적으로 보이지 않지만 다음과 같은 타입 변화가 일어나는 것입니다.

1. Some(String<"Hello, World!">)가 &Some(String<"Hello, World!">)로 바뀜
2. &Some(String<"Hello, World!">)가 Some(&String<"Hello, World!">)로 바뀜
3. 결국 map 내부의 인자 s는 &String<"Hello, World!">가 됨

함수를 호출할때도 객체의 값을 전달하면 소유권이 이동되서 객체를 더 이상 사용할 수 없었습니다. map 메소드도 마찬가지로 객체의 값으로 메소드를 호출하면 소유권이 map 메소드로 이동되서, 그 이후로는 원본 객체를 사용할 수 없습니다. 함수에서 객체의 값을 전달하는 대신에 레퍼런스틀 전달한 것처럼 map 메소드를 호출할때도 레퍼런스를 전달하면 소유권이 이동하는 것을 막을 수 있습니다.

as_ref 메소드를 사용한 후에는 정상적으로 빌드됩니다. s는 &String타입이 됩니다. 따라서 map의 호출이 끝난 뒤에도 계속 maybe_some_string을 사용할 수 있습니다.

컴파일러가 알려준 두번째 메소드는 as_mut메소드입니다. 이름에서 알 수 있듯이 단순히 불변 레퍼런스를 전달하는게 아니라 가변 레퍼런스를 전달해서 객체의 값을 수정할 수도 있게 해주는 메소드입니다.

```rust
let mut maybe_some_string = Some(String::from("Hello, World!"));
maybe_some_string.as_mut().map(|s| s.push_str(" Again!"));
println!("{:?}", maybe_some_string);
```

```bash
Some("Hello, World! Again!")
```

위와같이 map의 s라는 인자에 &mut String을 전달합니다. 그래서 객체를 수정할 수 있습니다.

Result도 마찬가지로 as_ref와 as_mut을 사용할 수 있습니다.

### Option과 Result의 실체 확인

Option과 Result는 enum타입입니다. 간단하게 아래 실험을 통해 enum타입이라는게 뭔지 그리고 as_ref라는게 무엇인지를 확인해보려고 합니다.

```rust
fn main() {
    let mut maybe_some_string = Some(String::from("Hello, World!"));
    println!("{:p}", &maybe_some_string);
    maybe_some_string.as_ref().map(|s| println!("{:p}", s));
    maybe_some_string.as_mut().map(|s| println!("{:p}", s));
}
```

```bash
0x7ffc8810d420
0x7ffc8810d420
0x7ffc8810d420
```

첫번째로 `Some<String>`타입을 가지는 maybe_some_string가 메모리 어디에 존재하는지를 확인해봤습니다. 메모리 주소를 보니 main함수의 스택입니다. 그리고 as_ref/as_mut메소드와 map메소드를 통해 Some안에 있는 String객체의 포인터를 확인해보니 같은 주소가 나왔습니다. 우리는 이것으로 2가지를 확인할 수 있습니다.

1. Some타입의 변수의 시작 주소와 String타입 객체의 시작 주소가 같습니다. 이것으로 우리는 Option이나 Result등의 타입은 실제로 메모리에 저장되는 데이터가 아니라는 것을 알 수 있습니다. enum타입은 컴파일러가 자체적으로 관리하는 가상의 타입입니다. 어느 변수가 Some타입으로 감싸여있다고하는 것은 실제로는 컴파일러가 그렇게 Some타입이 있는 것 같이 관리를 한다는 의미이지, 실제로 메모리에 Some이라는 객체가 존재하고 그 안에 다른 데이터가 존재하는 것은 아닙니다. 이것이 러스트 언어가 Zero Cost Abstraction (추가적인 성능 감소나 메모리 사용없이 추상화된 계층을 제공한다는 의미입니다)를 제공한다고 말하는 이유입니다. 지금 초급 단계에서 깊게 이야기할 수는 없지만, 다른 언어에 비해 러스트 컴파일러가 하는 일이 많다고 생각하셔도 좋습니다.
2. 가변 레퍼런스나 불편 레퍼런스나 사실상 포인터 주소는 같습니다. 즉 이 메모리 주소를 읽기용으로만 써야할지 데이터를 바꿀 수도 있는지는 관리하는 것도 컴파일러입니다. 마찬가지로 성능 감소나 메모리 사용없이 컴파일러가 제공해주는 기능인 것입니다.

### Result의 map_err 메소드

Result의 map 메소드는 값이 에러일때는 실행되지 않았습니다. 하지만 map_err은 map과 반대로 Result의 값이 에러일때 실행할 코드를 지정하는 것입니다.

하나의 예를 들면 아래처럼 serde_json에서 전달된 에러를, 자신이 정의한 에러 타입으로 변환할 때 사용할 수 있습니다.

```rust
let new_value = serde_json::to_string(&row).map_err(|e| {
                            MyError::StorageMsg(format!(
                                "failed to serialize row={:?} error={}",
                                row, e
                            ))
                        })?;
```

map에 대해서 기본적인 설명을 했으니 이번에는 제가 실제로 프로젝트를 하면서 작성한 코드에서 예제를 가져와봤습니다. serde_json은 JSON포맷을 처리하는 라이브러리입니다. to_string을 JSON포맷의 데이터를 문자열로 바꾸는 라이브러리 함수입니다. row라는 객체에 JSON포맷 바이너리 데이터가 들어있는데, 이것을 문자열로 바꾸려는 코드입니다.

위의 예제에서 만약 serde_json::to_string메소드의 반환값이 에러가 아니라면 ? 연산자는 Ok()안에 있는 문자열의 값을 Ok밖으로 꺼내서 new_value에 저장합니다. 하지만 serde_json::to_string 메소드가 row안에 있는 바이너리 데이터의 포맷에 문제를 발견하고 에러를 반환하면 그것을 MyError::StorageMsg라는 타입으로 변환합니다. 그러면 map_err은 Err(MyError::StorageMsg)타입의 에러를 반환하고 ? 연산자는 에러 값을 상위 함수로 전달합니다.

map은 Ok나 Some타입의 메소드로 호출되어서 Ok나 Some타입을 반환하지만, map_err은 Err타입의 메소드로 호출되어서 Err타입을 반환하는 차이가 있습니다.
