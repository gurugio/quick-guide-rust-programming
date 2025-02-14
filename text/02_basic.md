# 기본 문법

Rust 언어의 기본 문법을 소개하겠습니다. 이 책은 변수라는게 뭔지, 함수라는게 뭔지 등 일반적인 프로그래밍에 대한 설명은 생략하고, Rust 언어에 빠르게 적응하기 위해 필요한 사항들만 소개하겠습니다.

## 함수와 for 루프

FizzBuzz라는 함수를 만들어보겠습니다. 1부터 100까지의 숫자중에 3의 배수를 만나면 Fizz라고 출력하고 5의 배수를 만나면 Buzz라고 출력합니다. 만약에 3과 5의 공배수이면 FizzBuzz라고 출력합니다.

만약에 아래와 같이 for문에 시작값과 종료 조건등을 지정하는 문법만을 사용해보셨다면, 이런 형태의 for문은 버그에 굉장히 취약한 형태라는 것을 경험해보셨을 것입니다. "1부터 100까지의 숫자"라는 조건을 봤다면 다음처럼 생각하기 쉽습니다.

```c
for (i = 1; i < 100; i++)
```

이렇게하면 i가 100일때 처리를 하지 못하는데요 이런식으로 실수하기 쉬운 문법을 가지고 있습니다.

러스트에는 for문에 오직 이터레이터(Iterator)만 사용합니다. 대부분의 최신 언어들이 이미 제공하는 기능입니다. for와 if 구문을 사용해서 3의 배수이면 "Fizz"라는 메세지를 출력하고, 5의 배수이면 "Buzz"라는 메세지를 출력하고, 3과 5의 공배수이면 FizzBuzz를 출력하는 예제를 만들어봤습니다.

>
> 예제 코드는 <https://github.com/gurugio/quick-guide-rust-programming>에서 다운받을 수 있습니다.
>

```rust
// src/function_for/main.rs
fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
            /* do nothing */
        }
    }
}

fn main() {
    println!("Hello, function_for!");
    fizzbuzz_if_else(10);
}
```

아래와 같이 function_for/main.rs 파일을 빌드하고 실행할 수 있습니다. cargo run 명령은 빌드와 실행을 다 하는 명령입니다.

```bash
$ cargo run --bin function_for
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/function_for`
Hello, function_for!
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
```

예제 자체는 아주 간단하고 다른 많은 언어들에서 사용되는 문법들과 유사합니다. 예제에 사용된 러스트 문법만 간단히 소개하겠습니다.

프로그램은 main함수에서 시작합니다. 프로그램이 시작된 직후 println!매크로를 호출해서 간단한 메세지를 출력합니다. 그리고 fizzbuzz_if_else함수를 시작합니다.

함수의 정의는 fn 키워드로 시작합니다.

```rust
fn 함수이름(인자: 타입, 인자: 타입, ...) -> 반환값 {
	함수 라인1;
	라인2;
	결과값
}
```

코드의 각 라인은 C/C++과 같이 “;”로 끝나야됩니다. 그리고 마지막에 함수 반환값은 “;”없이 쓰게됩니다. Scala등의 함수형 언어와 유사한 점입니다. 반환값이 없으면 안써주거나 반환할게 없지만 구문의 특성상 반드시 반환값을 지정해야되는 경우 (if-else만 있는 함수같은 경우)에는 ()라고 써주기도 합니다.

러스트에서는 Range라는 타입으로 for문의 범위를 지정합니다. 파이썬과 유사합니다. 아래와 같이 작성하면 10을 제외한 9까지만 처리하는 코드가 됩니다. Bash나 몇몇 언어에서는 “1..10”이 10을 포함하지만 러스트에서는 10은 포함하지 않습니다.

```rust
for i in 1..10 {
......
}
```

1부터 10까지 처리하도록 하려면 다음과 같이 =를 추가해야합니다.

```rust
for i in 1..=10 {
......
}
```

if문에서 마지막 else는 아무것도 하지 않습니다만 else를 꼭 넣어줘야 완벽하게 모든 케이스를 처리하는 코드가 됩니다. else로 따로 할 일이 없다해도 else를 넣고 아무 처리를 하지 않는다는 코멘트라도 넣어줘야 보안에 신경쓴 코드가 됩니다. 러스트는 이런 보안 헛점들을 방지하기 위한 문법들을 가지고 있습니다. if문에 대한 세부적인 설명은 바로 다음장에서 이야기하겠습니다.

변수의 정의는 let 키워드를 사용합니다.

```rust
let 변수이름: 타입 = 초기값;
```

## if-else

이미 사용했지만 if-else만 사용하는 코드를 다시 만들어보겠습니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var;
    if (num % 3 == 0) {
        var = 3;
    } else if (num % 5 == 0) {
        var = 5;
    } else {
        var = 0;
    }
    printf("var=%d\n", var);
}
```

이 코드를 Rust로 변환하면 다음과 같이 만들 수 있습니다. 미리 주의해야할 것은 Rust에서 if-else문은 값을 가진다는 것입니다.

```rust
// src/if/main.rs
fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };
    println!("var = {}", var);
}
```

```bash
$ cargo run --bin if_return_value
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/if_return_value`
var = 5
```

첫번째 if문 블럭안에 3이 써있는데 3옆에 ;가 없습니다. 3이 { ... }로 둘러싸인 블럭의 값이 된다는 의미입니다. 만약에 num변수에 저장된 값이 3의 배수라면 var에 블럭의 값인 3을 저장하게됩니다.

C에 익숙한 분들은 아마 아래와 같은 3항 연산자가 생각나실 겁니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var = num % 3 == 0 ? 3 : num % 5 == 0 ? 5 : 0;
    printf("var=%d\n", var);
}
```

C언어에서 위와 같은 3항 연산자는 구문 자체가 값을 반환하게 됩니다. C언어에서 if문이 러스트와 같이 반환값을 가질 수는 없지만 3항 연산자를 사용하면 러스트와 같이 좀더 짧고 에러 처리가 확실한 코드가 될 수 있습니다.

물론 러스트도 아래와 같이 처음에 만든 C코드와 완전히 동일하게 구현할 수 있지만 추천하는 방식은 아닙니다.

```rust
fn main() {
    let num = 5;
    let var;
    if num % 3 == 0 {
        var = 3;
    } else if num % 5 == 0 {
        var = 5;
    } else {
        var = 0;
    }

    println!("var = {}", var);
}
```

왜 굳이 if문이 값을 가지도록 만들었을 까요? 프로그래밍 언어의 철학에 대해서 본격적으로 이야기하자면 저도 잘 모르는 깊은 이론들이 있겠지만, 제가 개발하면서 느꼈던 장점은 값이 초기화되지 않은 변수를 최소화할 수 있다는 것입니다. 초기화되지 않은 변수를 일부러 만드는 사람은 없을 것입니다. 하지만 코드가 길어지고 다른 사람의 코드를 유지보수하다보면 실수로 변수의 초기화 코드를 제거하기도 합니다. 그리고 아주 심각한 문제는 초기화가 안된 변수를 알아차리지 못하고 계속 사용하는 경우가 생길 수 있다는 것입니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var;
    if (num % 3 == 0) {
        var = 3;
    } else if (num % 5 == 0) {
        printf("This is error, please enter 3!\n");
    } else {
        var = 0;
    }
    printf("var=%d\n", var);
}
```

위 코드는 var변수를 초기화하는 것을 깜빡한 것입니다. if-else가 한두개이거나 처음 코딩할때는 초기화를 깜빡하지않겠지만, 개발이 점점 진행되고 코드가 길어지고 복잡해지면 초기화를 깜빡하는 경우가 자주 생깁니다. 운이 좋으면 그냥 쓰레기값이 출력되는 것으로 끝나겠지만, 대부분 의미없는 값이 이리저리 돌아다기나가 엉뚱한 곳에서 패닉을 발생시키고 디버깅하는데 며칠이 걸리게 만드는 경험을 해보셨을 것입니다.

러스트에서 if-else로 변수를 초기화하면 다음과 같이 값을 반환하지 않는 경우를 방지할 수 있습니다.

```rust
fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            //5 missing by mistake
            println!("Wrong number");
        } else {
            0
        }
    };
    println!("var = {}", var);
}
```

```bash
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:10:13
   |
6  | /         if num % 5 == 0 {
7  | |             //5 missing by mistake
8  | |             println!("Wrong number");
   | |             ------------------------ expected because of this
9  | |         } else {
10 | |             0
   | |             ^ expected `()`, found integer
11 | |         }
   | |_________- `if` and `else` have incompatible types
```

추가로 한가지 팁을 드리자면 러스트는 다양한 함수형 프로그래밍 기법을 사용하고 있어서 if-else를 사용할 일이 많이 줄어듭니다. C언어같은 절차형언어에서 if-else를 사용하는 처리를 Rust에서는 함수형 언어와같이 match나 map, filter등을 사용하는 경우가 훨씬 많습니다. 만약에 내 코드에 if-else가 많이 보인다면 Rust다운 코드를 만들지 못하고있는게 아닌가 자문해봐야합니다.

## Mutable 변수와 타입 추론

지금까지 우리가 만들어본 예제들은 변수가 생성될 때 값을 정하고 그 후에는 값을 바꾸지 않았었습니다.

```rust
fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
                /* do nothing */
        }
    }
}
```

위 예제를 보면 3개의 변수가 있습니다. “let”으로 생성한 변수가 2개, rem_three, rem_five가 있고, 하나는 max라는 함수 인자입니다. 이렇게 총 3개의 변수가 하나의 함수를 이루는 컨텍스트에서 생성되었습니다.

함수의 컨텍스트는 간단하게 이해하자면 함수가 동작하기 위한 모든 메모리, 코드 등을 합쳐서 부르는 말입니다. 로컬 변수는 함수의 스택에 저장되고, 함수 인자는 함수를 호출하는 상위 함수의 메모리에 있겠지요. 그리고 함수 자체를 이루는 코드도 있을 것입니다. 이런 것들은 컨텍스트라고 생각할 수 있습니다.

어쩌면 지금까지의 예제 코드를 보면서 모든 변수들이 값이 고정된 변수들이었다는 것을 알아차리지 못한 분들도 계셨을 것입니다. 그게 바로 러스트가 변수 선언의 기본을 변하지않는 속성(Immutable)으로 정한 이유입니다. 생각외로 많은 변수들이 한번 초기화되면 값이 바뀌지 않습니다. 특히 함수형으로 코드를 작성하면 더욱 그런 경우가 많아지고요 그리고 함수들을 잘 분리해서 여러 함수들이 간결하게 작성되면 함수의 로컬 변수들의 값이 자주 바뀌지 않는 경우가 많습니다.

어쨌든 값이 변하는 변수를 만들고자할 때는 추가로 mut라는 키워드를 넣어주어야 합니다. 이것은 let으로 변수를 선언할 때 뿐 아니라 함수 인자에서도 마찬가지입니다.

mut 키워드를 사용하는 예제를 보겠습니다.

```rust
// src/mutable_var/main.rs
fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}

fn main() {
    println!("{}", fib(10));
}
```

```bash
$ cargo run --bin mutable_var
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/mutable_var`
144
```

이 예제에서 3가지 새로운 컨셉을 소개하고 있습니다.

1. mut키워드를 사용해서 값이 변하는 변수와 함수 인자를 사용
2. loop 사용방법
3. 각 변수마다 일일이 타입을 안써줘도 러스트가 타입을 추론할 수 있음 (컴파일러가 변수가 사용되는 상황을 확인해서 적당한 타입으로 만들어줍니다. Python 언어와 같이 완전한 덕 타이핑 Duck typing 언어는 아니지만 어느정도 타입을 알아서 판단해줍니다. 하지만 강타입 언어이므로 컴파일러가 판단하지 못하는 경우가 자주 있고, 타입을 명확하게 지정하라는 컴파일 에러를 자주 만날 수 있습니다.)

mut키워드는 mutable의 약자입니다. 값이 변하는 변수라는 표시인데요 보통의 프로그래밍 언어들이 값이 안변하는 상수를 위한 키워드가 있는 것에 비하면 반대로 러스트에서는 값이 안변하는 변수가 기본이고, 값이 변하는 변수가 mut 키워드를 추가해줘야합니다. 실제로 러스트로 프로그래밍하다보면 의외로 자주 mut 키워드를 빼먹어서 컴파일 에러가 나는 경우를 겪을 것입니다. 그런데 생각보다 더 많은 경우에 mut 키워드를 무의식적으로 빼먹고도 문제없이 동작하는 것을 겪을 것입니다. 저도 처음에는 mut 키워드를 써줘야한다는게 번거로울것 같았지만, 곧 제 자신이 값이 변하지 않는 변수를 생각보다 많이 써왔다는 것을 깨달았습니다. 그리고 좀 더 경험이 쌓일 수록 값이 바뀌지 않는 변수를 많이 사용하게되고 결국 코드가 조금 더 간결해지는 것을 경험하게 되었습니다. 물론 러스트가 가진 함수형 프로그래밍 기능들을 사용해서 그렇기도 합니다만 지금은 값이 변하지 않는 변수가 기본이라는 것이 코드의 안정성을 위해서 괜찮은 선택이었다고 생각합니다.

loop 사용법은 저 개인적으로 타 언어의 while 보다 더 편리하다고 생각합니다. while을 사용하다보면 while(..)의 ()안에 탈출 표현식을 써주고 또 while() {..} 바디안에 또 다른 탈출식을 써주게 되는 경우가 많습니다. 이것보다는 loop처럼 기본 탈출식이 없는게 좀 더 읽기 편하다고 생각합니다.

러스트는 타입 추론 기능을 가지고 있습니다. C++에서는 auto라는 타입을 써주면 컴파일러가 최적의 타입을 찾아주는데 러스트는 아예 타입을 안써줄 수도 있습니다. 러스트는 강타입 언어이므로 타입이 분명 존재합니다만, 타입을 안써주면 컴파일러 코드에서 변수의 사용을 보고 타입을 지정해줍니다. 이게 왜 편리하냐면 러스트의 문법 특성 상 타입이 아주 긴 경우가 자주있습니다.

아래와 같은 타입을 변수 타입에 써야한다면 번거로울 수밖에 없겠지요.

```rust
Box<dyn Fn() + Send + 'static>
Arc<Vec<Box<dyn Collector<dyn CollectorModel> + 'static>>>,
```

출처 link: <https://doc.rust-lang.org/book/ch19-04-advanced-types.html>

참고로 함수를 정의할 때는 타입을 생략할 수 없습니다. 함수를 호출하기위해서는 입출력의 타입을 정확하게 알아야 컴파일러가 기계 코드를 생성할 수 있기 때문입니다.

## as를 이용한 타입 바꾸기

러스트는 값의 타입을 바꿀 때 항상 명확하게 선언해주어야 합니다. C언어 등 타입의 변환을 암묵적으로 지원해주는 언어들도 있지만 그런 언어들은 버그를 만들기 쉽고 보안에도 좋지 않습니다. 러스트는 그런 보안적인 요소에 특히 잘 대비하고 있습니다.

다음 예제 코드는 문자열로 된 숫자를 정수로 반환하는 함수입니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c - '0'; // 컴파일 에러 발생
    }
    ret
}
```

```bash
error[E0369]: cannot subtract `char` from `char`
  --> src/main.rs:10:19
   |
10 |         ret += c - '0'; // 컴파일 에러 발생
   |                - ^ --- char
   |                |
   |                char
```

혹시 코드만 보고도 바로 문제를 알아차리셨다면 타입에 대해 많이 고민해보셨다는 의미입니다. 이 코드를 빌드해보면 char타입 변수에서 char타입의 값을 뺄 수 없다는 에러가 발생합니다. 아래는 임시방편으로 char타입의 변수를 i32타입의 정수로 변환한 후 빼기 연산을 수행한 코드입니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}
```

이 코드는 컴파일 에러도 없고 잘 동작합니다.

위 예제를 통해 우리는 for문에서 생성되는 c라는 변수가 char라는 타입인지 알 수 있습니다. 러스트에서 for문에는 항상 이터레이터만 사용할 수 있다고 설명했는데, String이라는 구조체의 chars라는 메소드가 문자열의 각 문자들을 반환하는 이터레이터를 생성해주는 메소드라고 생각하면 됩니다.

러스트에서는 char 타입의 변수 c에서 char 타입의 문자 ‘0’를 뺄 수는 없습니다. 현대 언어들에 익숙한 분들에게는 당연할 수도 있는 사항인데요, C언어에 익숙한 분들에게는 당황스러운 것일 수도 있습니다.

C언어는 사실 어셈블리로 개발하던 프로젝트의 생산성을 높이기 위해 나온 언어입니다. 따라서 어셈블리가 익숙한 개발자들의 관점에서 디자인된 언어이다보니 모든게 숫자입니다. 참과 거짓도 숫자이고, 오류를 나타내는 NULL이나 에러 값도 숫자이고, 포인터도 숫자, char라는 타입도 숫자입니다. 그러니 타입이 다른 변수들간에도 더하기 빼기가 가능합니다. 하지만 이게 논리적으로 좋은 디자인인지는 의문입니다. 숫자가 아닌 타입을 더하거나 뺀다는건 논리적으로는 잘못된 연산이 되는게 더 옳지 않을까요.

잡설이 좀 길었지만 어쨌든 러스트에서는 타입의 변환을 as라는 키워드로 합니다. 추후에 몇가지 타입 변환과 관련된 키워드를 보겠지만 가장 기본적인 것은 바로 as 입니다. 이렇게 언어 자체에 키워드가 있어서 타입이 변환된다는 것도 논리적으로 옳은건지는 모르겠습니다만 Syntax sugar라고 생각해도 될듯합니다.

```rust
// src/as/main.rs
use std::num::ParseIntError;

fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}

fn parse_example(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

fn main() {
    println!("{}", string_to_digit("1234".to_string()));
    let ret = parse_example("1234");
    match ret {
        Ok(value) => {
            println!("Parsed integer: {}", value);
        }
        Err(_) => {
            println!("Failed to parse the string as an integer");
        }
    }
}
```

```bash
$ cargo run --bin as
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/as`
1234
Parsed integer: 1234
```

string_to_digit 함수의 문제점은 문자열이 숫자 외의 문자를 가지고있는 등의 에러 상황에서 에러 값을 반환할 수가 없다는 것입니다. i32 타입의 값을 반환하고 있는데 어떤 값이 에러인지, 아니면 정상적인 결과인지 알 수가 없습니다. 다른 프로그래밍 언어들도 에러를 표현하는 타입이 별도로 존재하지 않는 경우가 많습니다. 만약에 0을 에러값으로 정하면 input에 "0"을 받는 경우를 처리할 수 없습니다. -1도 마찬가지이고 i32의 최대값이나 최소값인 -2,147,483,648, 2,147,483,647 값들도 에러값으로 사용할 수 없습니다. 사용자가 "-2,147,483,648"라고 입력할 수도 있으니까요. 이런 사소해보이는 문제들이 중대한 보안 이슈의 원인이 되는 경우가 많습니다.

또 대부분의 프로그래밍 언어들이 에러인 경우와 반환값이 없는 경우에 대해서 명확하기 구분하지 못하고 있습니다. 함수를 정의할 때 반환값이 없다고 정의하고 에러를 처리하기 위해 예외를 반환하는 경우도 많습니다만, 사실 논리적으로 생각하면 이상한 코드입니다. 반환값이 없다고 정의된 함수인데 예외를 반환하는게 어쩔 수 없다고는 하지만 엄밀히 말해서 좋은 방법인지는 의문입니다.

러스트는 함수의 반환값으로 Result나 Option같은 타입을 사용하게 됩니다. Result나 Option안에 에러값이나 정상적인 결과 값을 담아서 처리합니다. 에러와 정상 반환값의 타입을 따로 지정할 수 있습니다. `Result<i32, ParseIntError>`이렇게 2개의 타입을 지정한게 첫번째는 정상적인 반환값의 타입이고 두번째는 에러값의 타입을 지정한 것입니다. 정상적인 상황에 반환값이 없다면 `Result<(), ParseIntError>`라고 지정할 수도 있습니다. 따라서 함수가 반환값이 없는 경우에도 에러 상황을 판단할 수 있습니다. Result/Option안에 에러가 있으면 에러 상황이 발생한 것이고, 아무런 값도 없으면 정상적으로 실행이되고 반환값이 없는 경우입니다. 추후에 좀 더 자세히 알아볼 것인데 미리 이런 상황에서 필요하다는 것을 알고 넘어가면 나중에 좀 더 이해하기 쉬울것 같아서 예제를 만들어보았습니다.

## match를 이용한 패턴 매칭

저는 for, while, if, 함수호출 등 러스트의 기본 문법들에 대해서는 모든 프로그래밍 언어들에서 봐왔던 것들이라 금방 익숙해질 수 있습니다. 그러다가 처음으로 흥미를 느낀 문법이 바로 패턴 매칭 부분이었습니다. if-else가 여러개 있는 케이스는 다른 언어들에서도 switch문으로 작성하기 때문에 별다른게 있을까 생각했었는데 패턴 매칭이라는 것의 의미를 이해하게되면서 감탄했던 기억이 있습니다.

패턴 매칭은 사실 정확한 정의가 무었이기를 따지기 보다는 쓰다보면서 적응해나가는게 더 효율적인 접근방법이라고 생각합니다. 일단 가장 쉬운 예제를 하나 보겠습니다.

```rust
// src/match/main.rs
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            _ => (),
        }
    }
}

fn main() {
    fizzbuzz_2(15);

    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    for i in 1..=30 {
        let msg = match i {
            n if n % 15 == 0 => format!("{} - FizzBizz", n),
            n if n % 3 == 0 => format!("{} - Fizz", n),
            n if n % 5 == 0 => format!("{} - Buzz", n),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin match
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/match`
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
12 - Fizz
15 - FizzBuzz
generation=X
1
2
3 - Fizz
4
5 - Buzz
6 - Fizz
7
8
9 - Fizz
10 - Buzz
11
12 - Fizz
13
14
15 - FizzBizz
16
17
18 - Fizz
19
20 - Buzz
21 - Fizz
22
23
24 - Fizz
25 - Buzz
26
27 - Fizz
28
29
30 - FizzBizz
```

fizzbuzz_2함수는 이전에 만든 fizzbuzz 예제를 패턴 매칭으로 바꾼 함수입니다. match라는 키워드 다음에 있는 (i % 3, i % 5) 는 하나의 튜플 Tuples를 만들었습니다. 이 튜플의 값이 match다음에 나오는 각 패턴에 해당될 때 각기 다른 메세지를 출력하도록 만든 것입니다. 패턴에서 언더바 \_ 는 모든 값을 의미합니다. 튜플의 값을 비교해야하는데 두개의 값이 있어야 하므로 하나는 0이고 다른 값은 무엇이든 상관없을 때 언더바를 사용한 것입니다. 패턴 매칭이므로 2개의 값이 있는 튜플의 패턴과 매칭이 되려면 각각의 매칭 케이스마다 2개의 값이 있게됩니다.

if-else를 쓸 때와 마찬가지로 패턴의 비교 순서가 바뀌면 전혀 다른 결과를 만들어냅니다. 만약에 아래처럼 패턴 순서를 바꿨다면 어떤 일이 벌어질까요?

```rust
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (0, 0) => println!("{} - FizzBuzz", i),
            _ => (),
        }
    }
}
```

만약 튜플 값이 (0, 0)이면 FizzBuzz를 출력하는게 아니라 Fizz를 출력할 것입니다.

흔하게 사용하는 패턴을 하나 더 보겠습니다. main함수안에 다음과 같이 어떤 변수의 값이 어느 범위에 속하는지 판단하는 코드가 있습니다.

```rust
let gen = match age {
    0..=20 => "MZ",
    21..=50 => "X",
    51..=100 => "A",
    _ => "?",
};
```

age라는 변수를 0..20, 21..=50, 51=100이라는 3가지의 Range타입과 비교하는 것인데, 세밀하게따지면 age가 Range타입이 아니므로 패턴으로 매칭이 되는게 조금 이상할 수도 있습니다. 문법적으로 세세하게 따지면 복잡할 수도 있는 코드입니다만 지금은 이런 것도 가능하다 정도로 생각하고 일단 자주 활용하면서 익숙해지면 될듯 합니다. 단순히 age라는 변수의 범위만 따져서 어떤 동작을 수행하는게 아니라 각 경우에 따른 반환 값을 gen이라는 변수를 생성하는데 사용한 것을 주의해서 보시기 바랍니다. match라는 구문도 결국 if와 같이 반환값을 가지게됩니다.

단순히 패턴에 일치하는 것만 확인하는게 아니라 아래와 같이 if와 결합해서 조건식을 써줄 수도 있습니다.

```rust
for i in 1..=30 {
    let msg = match i {
        n if n % 15 == 0 => format!("{} - FizzBizz", n),
        n if n % 3 == 0 => format!("{} - Fizz", n),
        n if n % 5 == 0 => format!("{} - Buzz", n),
        _ => format!("{}", i),
    };
    println!("{}", msg);
};
```

format!()은 문자열 객체를 반환해주는 매크로 함수입니다. 조건식에 따라 다른 문자열을 생성해서 msg라는 변수에 저장하게됩니다.

러스트 관련 소개 자료나 공식 문서등을 보면 러스트의 패턴 매칭은 강력하다라는 설명이 있습니다. 보통 강력하다라는 말은 다양한 방식으로 사용된다는 의미입니다. 이 책에서 모든 케이스를 일일이 보여드릴 수는 없지만 러스트로 개발하면서 조금씩 시도를 하다보면 이런것도 되네 하면서 감탄하는 경험을 하게되실 것입니다. 보통의 switch문은 변수의 값을 비교해서 사용하지만, 패턴 매칭은 말 그대로 패턴이나 코드의 값을 비교하므로 많은 장점이 있습니다.

## 값을 가지는 표현식

지금까지 if-else나 패턴 매칭의 예제를 보면 변수를 선언하는 부분에 복잡한 코드를 넣은 것을 볼 수 있었습니다. 이런 표현식에 대해서 좀 더 자세히 이야기해보겠습니다. 다음 예제는 지금까지 나온 값을 반환하는 표현식들을 모아놓은 것입니다.

참고로 아래 예제를 실행해보면 ret_zero라는 함수를 호출하고있지 않다는 경고 메세지가 나옵니다. 에러는 아니므로 코드에 문제는 없습니다. \_var같은 변수도 마찬가지로 생성만하고 사용하지않습니다만 경고 메세지가 없습니다. 변수 이름을 \_로 시작했기 때문입니다. 이와같이 임시로 만들어놓고 나중에 사용하게될 변수는 이름을 \_로 시작하면 컴파일을 깔끔하게 할 수 있습니다. ret_zero 함수의 이름에도 앞에 \_를 붙여보면 경고 메세지가 사라지는 것을 볼 수 있습니다.

```rust
// src/expr/main.rs
fn ret_zero() -> i32 {
    0
}

fn main() {
    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    let num = 45;
    let _var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };

    let x = 9;
    let _y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
}
```

```bash
$ cargo run --bin expr
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
warning: function `ret_zero` is never used
 --> src/expr/main.rs:1:4
  |
1 | fn ret_zero() -> i32 {
  |    ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "expr") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr`
generation=X
```

위 예제의 match를 이용한 패턴 매칭 구문에서 { 로 시작하고 } 로 끝나는 하나의 블럭을 표현식Expressions 라고 합니다. 마찬가지로 if-else도 하나의 표현식입니다.

그리고 표현식은 값을 반환합니다.

값을 반환하는건 또 무엇이 있나요? 함수가 있습니다. 함수도 하나의 표현식입니다.

```rust
fn ret_zero() -> i32 {
    0
}
```

함수도 {와 }로 시작과 끝을 나타내고 반환값을 마지막에 써놓은 표현식입니다. 위 예제의 if-else도 각 {} 블럭 안에 반환값이 정해져있습니다. 그리고 match 구문에서도 각 상황에 따른 반환값이 있습니다. 이런 표현식은 ;를 만나면 그 반환값이 무시됩니다.

반환값을 가지는 표현식을 이용해서 아래와 같은 변수 초기화 코드가 만들어질 수 있게됩니다. 중간중간에 있는 코드들은 ;로 끝나므로 반환값이 없고 마지막에있는 코드에만 ;이 없으므로 마지막줄에 있는 수식의 결과값이 y의 값이 됩니다.

```rust
let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
```

y라는 변수의 값을 계산하기 위해 복잡한 코드가 나열되는게 아니라 y의 선언부에 모여있을 수 있습니다. 물론 위에서 본것과 같이 match, if-else등 도 올 수 있습니다. 다른 언어와 마찬가지로 당연히 함수 호출도 올 수 있겠지요.

참고로 영어로 표현식은 Expression이고 표현식에 ;가 붙은 것을 Statement라고 구분해서 부르는 경우도 있습니다.

## 배열과 배열을 참조하는 슬라이스

문자열을 사용하는 예제를 몇개 봤으니 우선 배열과 슬라이스에 대해서 알아본 후 본격적으로 문자열(사실은 문자열이라기 보다는 String 타입의 객체가 정확한 표현이긴 합니다)을 알아보겠습니다.

배열의 정의는 같은 타입의 데이터가 메모리에 연속적으로 나열된 데이터 구조를 말합니다.

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Example array of numbers
```

위와 같이 i32이라는 같은 타입의 데이터가 연속적으로 5개가 메모리에 연속적으로 저장되어있습니다. 메모리에 연속적으로 위치하고있기 때문에 우리는 numbers[0] 다음에 있는 데이터가 numbers[0]의 위치(포인터)에서 i32타입의 크기, 4만큼 더한 곳에 위치한다는 것을 알 수 있습니다.

```code
numbers[0] ==> 0x100
numbers[1] ==> 0x100 + 1 * 4 = 0x104
...
numbers[i] ==> 0x100 + i * 4
```

그래서 배열의 인덱스 [0], [1]을 가지고 빠르게 각 데이터에 접근할 수 있다는 것이 배열의 특징입니다. 연결 리스트나 트리 등보다 빠른 접근이 가능합니다.

슬라이스는 이런 배열의 일부(혹은 전체)만을 접근하려고 만든 레퍼런스 타입입니다.

```rust
let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)
```

위의 슬라이스는 numbers라는 배열의 1,2,3번 데이터에만 접근할 수 있습니다. [i32]는 i32타입의 배열에 접근한다는 표시히고 참조 연산자 &를 써서 배열에 대한 참조라는 것을 나타냅니다. 내부적으로는 배열의 크기도 저장하고 있습니다. 그래서 참조하고있는 배열의 3개 데이터만을 참조한다는 것도 내부적으로 저장하고 있습니다.

여기서 배열이나 구조체같은 여러 데이터가 같이 묶여있는 타입의 디버깅을 위해 한가지 좋은 방법을 소개하겠습니다.

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Example array of numbers

    let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);
}
```

“{:?}”라는 출력 포맷을 사용하면 아래와 같이 배열의 데이터를 전부 출력해줍니다.

```rust
Array: [1, 2, 3, 4, 5]
Slice: [2, 3, 4]
```

참고로 나중에 배울 구조체도 같은 방법으로 "{:?}"을 이용해서 구조체의 각 데이터 필드를 출력할 수 있으니, 디버깅을 위해서는 유용하게 사용해보시기 바랍니다.

실행 결과를 보면 슬라이스를 통해 3개의 데이터만 접근할 수 있는 것을 알 수 있습니다. 배열의 참조이지만, 슬라이스라는 타입 내부에는 이 슬라이스가 몇개의 데이터를 참조할 수 있는지에 대한 정보가 같이 저장되어있다고 보시면 됩니다. 그래서 슬라이스의 데이터를 전부 출력해보면 3개만 출력되는 것입니다.

슬라이스가 뭔지는 알았는데 그럼 슬라이스가 왜 필요한지가 의문일 수 있습니다. 왜냐면 몇몇 언어에서는 슬라이스가 없기 때문입니다. 파이썬등 최신 언어들은 대부분 슬라이스를 지원합니다만 C언어 등 오래전 출시된 언어들은 지원하지 않습니다. 배열이나 구조체를 읽다가 데이터가 저장된 영역을 넘어서 그 다음 메모리까지 읽는 것을 방지할 수 없는 언어들입니다. numbers[4]까지만 읽어야되는데 numbers[5]를 읽거나 쓰려는 실수를 누구나 해본적이 있을 것입니다.

슬라이스는 바로 이런 실수를 방지하려고 있는 것입니다. 함수나 쓰레드를 호출 할 때, 그 함수나 쓰레드가 배열의 일부분에만 접근해야한다고 할 때, 배열 전체를 전달하는게 아니라 슬라이스를 전달하게됩니다.

위에서 슬라이스는 배열의 일부 혹은 전체에 접근하기 위한 타입이라고 설명했습니다. 다음 예제를 보겠습니다.

```rust
// src/array_slice/main.rs
fn sum_array_ref(nums: &[i32]) -> i32 {
    let mut s = 0;
    let len = nums.len();
    let mut index = 0;
    loop {
        if index >= len {
            return s;
        }
        s += nums[index];
        index += 1;
    }
}

fn sum_slice(nums: &[i32]) -> i32 {
    let mut s = 0;
    for i in nums.iter() {
        s += i;
    }
    s
}

fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);

    println!("{}", sum_array_ref(&numbers));
    println!("{}", sum_slice(slice));
}
```

```bash
$ cargo run --bin array_slice
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/array_slice`
Array: [1, 2, 3, 4, 5]
Slice: [2, 3, 4]
15
9
```

sum_array_ref는 배열을 참조로 전달받는 함수이고,  sum_slice는 슬라이스를 전달받는 함수입니다. 슬라이스가 배열의 참조이므로 형태가 동일합니다. 각 함수는 자신이 전달받은 데이터가 배열의 참조인지, 배열의 일부분을 가르키는 슬라이스인지 알 필요가 없습니다. 두개가 완전히 동일한 것이기 때문입니다.

배열이나 문자열을 처리하는 함수를 만들 때는 항상 슬라이스로 인자를 받도록 만드는 습관을 가지면 좋습니다.

물론 슬라이스같은 참조가 아니라 배열이나 문자열을 그대로 전달하면 소유권까지 함수로 넘어가게되서 함수를 호출한 코드에서 다시는 배열이나 문자열을 접근할 수 없게되기 때문에, 그렇게 슬라이스로 처리를 할 수 밖에 없습니다. 러스트의 소유권 개념은 앞으로 좀 더 자세히 이야기하겠습니다만, 러스트는 이렇게 사용자가 데이터를 전달하면서 실수할 수 있는 부분들을 최대한 막으려는 디자인 철학을 가진 언어입니다. 

Unmanaged언어의 대표 주자인 C언어를 생각해봅시다. C언어도 버전을 계속 올려가며 최신 기능들을 추가하고 있습니다. C언어의 근본적인 문제점들을 해결하려고 C++언어를 출시하고 또 C++언어를 수십차례 버전을 올려가며 규약들을 만들고, 스마트 포인터를 만드는 등의 노력을 했었지만, 근본적으로 언어의 철학 자체가 개발자가 모든 것을 직접 처리할 수 있어야 한다는 철학을 밑바탕에 가지고 있기 때문에 완전히 막을 수 없는 헛점들이 있었습니다. 러스트는 그런 C/C++언어의 최신 기법들을 모두 모아놓고, 강제로 쓰도록 정해놓은 언어라고 생각하시면 이해하는데 도움이 될것 같습니다.

## 문자열을 저장하는 String타입

많은 언어에서 문자열은 문자 하나를 표현하는 char 타입의 배열입니다. char타입은 8비트 부호없는 정수를 의미합니다. 결국 바이트의 배열을 의미하게됩니다. 하지만 러스트를 비롯한 고급언어들은 문자열은 String이라는 구조체나 새로운 타입으로 표현합니다. 단순히 1바이트로 표현할 수 있는 ASCII 코드의 배열이 아니라, UTF-8 코드를 사용해서 각 문자를 관리하기 때문입니다.

그럼에도 러스트에서 슬라이스와 함께 문자열을 설명하는 경우가 많은데 그 이유를 알아보겠습니다.

우선 String(<https://doc.rust-lang.org/std/string/struct.String.html>) 이라는 타입에 대해서 알아보겠습니다.

메뉴얼을 자세히 볼 필요는 없지만 첫줄만 봐도 결국 하나의 구조체라는 것을 알 수 있습니다. 문자의 배열은 아닙니다. 따라서 String을 사용하기 위해서는 우선 String타입의 객체를 생성해야합니다. 다음 짧은 예제에는 몇가지 흔하게 사용되는 String 생성 방법들을 모아봤습니다. 참고로 String은 러스트의 “The Rust Standard  Library”(약자로 std)에 포함되기 때문에 명시적으로 지정해주지않아도 자동으로 빌드에 포함됩니다. C언어의 include나 파이썬의 import등과 같은 추가적인 절차없이 바로 사용할 수 있습니다.

String을 사용하는 몇가지 방식들에 대한 예제를 보면서 이야기하겠습니다.

```rust
// src/string/main.rs
fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
    let _hello = String::from("Hello, world!");
    let mut _s = String::new();
    let _s = "initial contents".to_string();
    let _hello = String::from("안녕하세요");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);

    let mut mutable_string = String::from("hello");
    mutable_string.push_str(" world");
    println!("{}", mutable_string.chars().nth(0).unwrap());

    {
        let hello = "hell".to_string();
        let _r1 = &hello;
        //let mut r2 = &mut hello; // Build Error!!!
    }
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin string
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/string`
hello
hello
h
```

가장 먼저 주의해야할 것은 “이렇게 큰따옴표안에있는 문자열”, 즉 리터럴(Literal)은 String이 아니라는 것입니다. 리터럴은 컴파일 시점에 고정된 크기의 데이터입니다. 리터럴은 소스 코드에 하드 코딩되었으므로 프로그램의 실행중에 데이터가 변하거나 크기가 바뀌는 일이 없습니다. 따라서 컴파일러는 힙 영역에 메모리를 할당해서 리터럴을 저장하는게 아니라 프로그램 코드가 저장되는 영역 (프로그램의 코드가 저장된 메모리 영역)에 저장합니다. 리터럴과 String이라는 타입의 객체는 다른 것입니다. String타입의 객체는 프로세스가 실행 중에 프로세스의 힙 영역에 메모리를 할당하고, 할당된 메모리에 리터럴 데이터를 복사해서 생성합니다. 예제 코드에있는 from이라는 정적 메소드나 to_string이라는 메소드가 실행하는 리터럴을 String 객체의 메모리에 복사합니다.

각 메소드들을 설명하자면 다음과 같습니다.

- from(”msg”): “와 “안에 들어가는 리터럴을 이용해서 String 객체를 생성함
- new(): 아무런 데이터가 없는 String 객체 생성
- “msg”.to_string(): “와 “안에 있는 문자열을 String객체로 생성함, from과 같음

그리고 또 하나 흔하게 String을 만드는 방법이 다음과 같이 format! 매크로 함수를 이용한 방식입니다.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

String은 워낙 많이 사용되는 타입이므로 다양한 기능의 메소드들이 있습니다. 파이썬이나 자바등 고급 언어를 다뤄본 분이라면 이미 익숙한 메소드들이 많을 것입니다. push_str나 insert, len 등등 대부분의 언어들과 마찬가지로 String객체에 추가로 문자열을 넣거나 길이를 반환하거나 하는 메소드들이 있습니다.

그럼 “이렇게 큰따옴표 안에 들어간 문자열”은 무엇이고, String과의 관계는 어떻게 되는 걸까요?

String 구조체의 정의부터 찾아보겠습니다.

```rust
pub struct String {
    vec: Vec<u8>,
}
```

출처: <https://doc.rust-lang.org/src/alloc/string.rs.html#365>

고급 언어를 다뤄봤다면 반드시 사용해봤을만한 벡터가 나타났습니다. 벡터는 기본적으로 배열이지만, 데이터의 크기가 동적으로 커지거나 작아질 수 있는 기능을 추가한 타입입니다. 배열은 생성시에 지정된 크기만을 갖지만 벡터를 사용하면 데이터를 추가하면 추가하는만큼 데이터의 크기가 커지고, 데이터를 지우면 지워진만큼 크기가 작아집니다. 벡터에 대한 보다 자세한 설명은 다음 장에 이야기하겠습니다.

다시 String으로 돌아와서 String의 정의를 보고 알게된 것은 String은 그냥 8비트 값들의 배열이라는 것입니다. 그럼 결국 String의 슬라이스는 8비트 값들의 배열이라는 것도 알게됩니다. 그리고 String의 슬라이스를 나타내는 &str라는 타입이 있습니다.

예제 코드에 다음과 같이 2줄을 추가해보세요.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &String = &greeting[1..3];
```

이 예제 코드는 아래와 같은 에러를 내고 빌드되지 않습니다.

```rust
error[E0308]: mismatched types
   --> src/main.rs:173:35
    |
173 |     let gretting_slice: &String = &greeting[1..3];
    |                         -------   ^^^^^^^^^^^^^^^ expected `&String`, found `&str`
    |                         |
    |                         expected due to this
    |
    = note: expected reference `&String`
               found reference `&str`
```

아직 에러 메세지가 다 이해는 안되지만 다음과 같이 코드를 바꾸라는 도움말이 나왔습니다. 도움말대로 수정하면 빌드가 제대로 되는 것을 확인할 수 있습니다.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &str = &greeting[1..3];
```

이렇게 String객체의 슬라이스를 사용할 때는 &String이 아닌 &str 타입을 사용해야한다는것을 알 수 있습니다.

String은 흔히 말하는 Fat pointer입니다. 실제로 문자열이 저장된 메모리의 주소도 들어있지만, 데이터의 현재 길이나 버퍼의 크기 등 추가 정보들이 들어있습니다. 슬라이스는 이전 배열에서도 봤지만 코드가 실행되는 시점에서 순수 데이터의 일부분이나 데이터 전체를 보기 위한 참조입니다. String객체를 C에서 하나의 구조체라고 생각하고, 슬라이스를 데이터중 일부분에 대한 포인터 char*라고 생각하면 그 관계에 대해 이해하는데 도움이될지도 모르겠습니다.

```rust
struct String {
    int buffer_len;
    int data_len;
    char *buffer;
};
```

그럼 한가지 의문이 드는게 &str이 아니라 str으로 참조 연산자를 안쓰고 사용할 수는 없는가 입니다. 사용할 수 없습니다. 왜냐면 러스트는 메모리 관리를 위해 컴파일 시점에서 크기가 정해진 객체만을 허용하기 때문입니다.

str타입은 벡터에 들어있는 UTF-8데이터 그 자체, 순수하게 문자들만을 저장하는 메모리 버퍼를 가리키는 변수가 되어야 합니다. 하지만 문자열의 길이는 정해진 것이 아닙니다. 일반적인 구조체는 각 필드의 데이터 타입이 고정되어있으니 컴파일 시점에 크기를 알 수 있고, 배열은 배열을 선언할 때 데이터가 몇개인지 지정하기 때문에 크기를 알 수 있습니다. 하지만 벡터가 가진 메모리 덩어리의 크기는 프로그램이 실행되는 중간에 얼마든지 변할 수가 있는 것입니다. 결국 str라는 타입은 그 자체만으로는 사용할 수가 없습니다.

하지만 참조연산자 &를 붙이면 이야기가 달라집니다. 왜냐면 &str는 포인터이고 포인터의 크기는 컴파일 시점에 고정되어있기 때문입니다. 최신 씨피유를 쓴다면 64비트, 8바이트로 고정된 크기를 가잡니다. 따라서 아래와 같이 선언하는 것은 컴파일러가 봤을 때 64비트의 변수를 스택 메모리에 만들면 되는 것이니, 아무런 문제가 없습니다.

```rust
let ref_literal: &str = "hello";
let ref_string: &str = &string_object[2..5];
```

ref_literal이나 ref_string이나 그 자체는 포인터로 고정된 크기를 가지고 스택에 저장되어있습니다. 그리고 둘다 가리키는 데이터 또한 모두 동적으로 크기가 변하지 않습니다. 따라서 컴파일러는 앞으로 이런 문자열 슬라이스를 사용하는 코드를 볼때마다 메모리 안전성을 확인할 수 있습니다. 가지고있는 문자열의 크기 이상으로 읽고쓰는 것을 방지할 수 있습니다.

반대로 "hello"와 같은 리터럴을 이용해서 String을 만드는 것을 생각해보겠습니다. 가장 먼저 문자열이 저장될 벡터를 만들어야합니다. 벡터를 만들려면 힙 메모리에 메모리를 할당해야겠지요. 그리고 그 외에 문자열의 크기나 기타 정보들을 관리하기 위한 추가 데이터들을 할당해야합니다. 그렇게 슬라이스 “hello”를 가지고 String을 만들기 위한 추가 작업들을 format!과 같은 매크로 함수나 to_string등의 메소드 등이 실행하는 것입니다.

사실 이와같이 세부적인 것을 깊게 따지다보면 처음 러스트를 접하는 입장에서는 혼란만 생길 수 있습니다. 왜 메모리 크기가 컴파일 시점에 고정되어야하는지, 그럼 동적으로 할당되는 메모리는 사용할 수 없는 것인지 의문이 더 생깁니다. 지금 이 순간에는 String에 대한 참조나 슬라이스가 &str라는 것만 생각하고 넘어가는 것도 방법입니다. 아니면 러스트의 String 구조체에 대한 매뉴얼이나 The Rust Programming Language등의 추가 자료를 찾아보면서 각자 수준에 맞게 이해해 나가는 것도 필요합니다. 그렇게 지금은 간략하게 이애하고 넘어가서 점점 연습하고 문제에 부닥치면서 아 이래서 그랬구나하는 순간을 만나는 것이 프로그래밍 공부라고 생각합니다.

마지막으로 String 객체를 다른 함수에 전달할 때 슬라이스를 써야한다는 것을 이야기하겠습니다. 아직 소유권에 대해서 배우지는 않았지만 간단하게 말하면 String 을 그대로 다른 함수에 전달하면, 그 함수를 호출한 이후에는 그 객체를 다시 사용할 수 없습니다. 객체를 그대로 전달한다는 것은 소유권까지 넘겼다는 것이기 때문입니다. 그 와 반대로 String객체의 슬라이스를 넘기는 것은 객체에 있는 문자열 데이터의 참조권을 잠시 빌려주는 것으로 생각하면 됩니다. 함수가 끝나더라도 객체의 소유권은 함수를 호출한 코드에 남아있기 때문에 계속 객체를 사용할 수 있습니다.

예제 코드에 get_moved_string이라는 함수가 있습니다.

```rust
fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
......
    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);
......
}
```

이 함수를 참조가 아닌 객체 그대로 전달받도록 바꿔보겠습니다. get_moved_string함수의 인자 타입이 &str에서 String으로 바꾸겠습니다.

```rust
fn get_moved_string(data: String) {
    println!("{}", data);
}

fn main() {
......
    let moving_string = String::from("hello");

    get_moved_string(moving_string);
    println!("{}", moving_string);
......
}
```

빌드를 해보면 이런 에러 메세지를 볼 수 있습니다.

```bash
> 161 | fn get_moved_string(data: String) {
|        ------------       ^^^^^^ this parameter takes ownership of the value
|        |
|        in this function
> 
```

객체를 그대로 함수에 전달했기 때문에 함수에 객체의 소유권까지 옮겨졌다는 뜻입니다. 원래는 main함수가 moving_string이라는 변수의 소유권을 가지고있었고, 마음대로 사용할 수 있었지만, 객체를 get_moved_string함수에 전달했으므로 객체의 소유권까지 넘어가버려서 main함수는 더 이상 moving_string을 사용할 수 없게 되었습니다.

어떤 객체를 함수에 전달할 때는 일반적인 상황에서는 참조를 전달해야하고, String을 함수에 전달할 때는 &str을 전달해야한다는 것을 기억합시다.

### String을 배열처럼 참조할 수 없는 이유

아래와 같이 String객체에서 첫번째 글자를 출력할 수 있을까요?

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string[0]);
```

할 수 없습니다. String타입은 배열이 아니므로 기본적으로 `[0]`과 같은 인덱스를 사용할 수 없습니다.
물론 굳이 인덱스를 사용할 수 있도록 암묵적인 구현을 만들 수도 있습니다만 Rust언어는 그런 암묵적인 구현을 최대한 피한다는 철학을 가지고 있습니다.
위 코드를 빌드하면 아래와 같은 에러 메세지를 얻으실겁니다.

```bash
> error[E0277]: the type `String` cannot be indexed by `{integer}`
--> src/main.rs:167:20
|
167 |     println!("{}", mutable_string[0]);
|                    ^^^^^^^^^^^^^^^^^ `String` cannot be indexed by `{integer}`
> 
```

일단 답부터 이야기하면 아래와 같이 chars메소드를 호출해서 이터레이터를 만든 후, nth메소드로 특정 인덱스의 문자를 얻을 수 있습니다.

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string.chars().nth(0).unwrap());
```

nth메소드는 Option이라는 Enums를 반환하므로 이 Enums에서 최종 문자를 얻어내기 위해 unwrap이라는 메소드를 호출한 것입니다.

일단 Option이라는 Enums은 추후에 알아보기로 하고, 왜 인덱스를 이용한 직접 접근이 안되게 막아놨을까요?

그 이유는 UTF-8을 완벽하게 지원하기 위해서입니다. 언어를 디자인할 때 인덱스 참조를 지원해서 [0]이 0번째 바이트를 반환하도록 만들었을 수도 있습니다. 하지만 이러면 ascii에 대한 지원은 잘 될지 몰라도 UTF-8을 제대로 지원하는 언어가 될 수는 없습니다. 첫번째 글자를 반환할 수도 있었겠지만, 첫번째 글자 하나만 놓고봤을 때 이 첫글자가 1바이트가 될지 2바이트가 될지 알 수가 없습니다. 이런 여러가지 문제들이 있기 때문에, 항상 이터레이터를 호출하도록 만들고, 이터레이터가 문자열의 전체 데이터를 분석한 후에 한 문자씩 반환하도록 만들었습니다. 그런 이유로 String의 이터레이터 메소드 chars의 처리 속도가 느린 것입니다.

만약 바이트 단위로 쪼개고싶다면 as_bytes라는 메소드를 호출하면 됩니다. 문자열 데이터가 반드시 ascii 문자열이라는 상황이라면 사용할 수 있는 옵션입니다.

## 변수를 읽고 쓸수있는 권한을 의미하는 소유권(Ownership)

배열에서의 슬라이스나 String과 &str의 관계를 보면서 소유권을 넘기지 않기 위해 참조를 사용한다는 이야기를 수차례 했습니다. 슬라이스도 그렇지만 그 외에 러스트의 문법적인 특징들의 상당수가 소유권 개념을 구현하기 위해서 만들어진 것들이라고 해도 과언이 아닙니다. 왜 이런 문법을 정했을까? 왜 이건 이렇게 복잡하지? 등등 러스트를 공부하면서 겪게되는 의문들과 진입장벽들의 상당수가 소유권과 연관이 있습니다. 러스트가 가진 장점 중에 가장 큰 장점이라고 이야기하는 메모리 안전성이 바로 소유권으로 인해 가능한 것입니다.

소유권이 뭔지 그래서 러스트가 데이터를 메모리에 어떻게 배치하고 관리하는지를 이야기해보겠습니다.

### 소유권의 의미

소유권은 단어 그대로 생각하면 변수를 내 마음대로 할 수 있는 권한 즉 변수에 데이터를 할당하고 읽고 쓰고 해지해할 수 있는 권리일 것입니다. 함수의 인자로 전달받은 데이터에 대한 소유권도 있을 수 있으니 여러 함수나 여러 쓰레드에서 공유되는 변수나 메모리에 대한 권한을 의미합니다.

가비지 콜렉터가 있는 자바 등의 언어는 메모리를 해지할 수 있는 권한이 프로그램 코드가 아닌 가비지 콜렉터에게 있습니다. 프로그램은 메모리를 할당받아서 객체를 만들어서 읽고 쓸 수 있지만 해지하지는 않습니다. 그냥 더 이상 접근하지 않고 있으면 가비지 콜렉터가 알아서 메모리를 해지해줍니다.

러스트는 컴파일러가 프로그램 코드를 컴파일 할 때 모든 메모리의 소유권을 추적합니다. 러스트가 정한 규칙에 어긋나게 메모리에 접근하는 코드가 있으면 친절한 안내 메세지를 출력하고 더 이상 컴파일을 하지 않습니다. 그래서 러스트 코드의 컴파일 시간이 오래걸린다는 불평이 많습니다. 수십 ~ 수백줄의 간단한 코드도 몇 초정도 시간이 걸리는걸 보면서 좀 답답할 때도 있긴합니다. (하지만 그정도의 간단한 코드를 만드는데 컴파일하고 고치고 다시 컴파일하는 과정을 여러번 반복해야할 정도로 컴파일 에러를 많이 만들고 있다는 것은 개발자에게도 문제가 있는게 아닌가 생각됩니다.) 하지만 빌드를 여러번 할 필요도 없는게 VSCODE등 대부분의 개발툴에서 러스트 언어를 동적으로 분석해주고, 코드를 쓸 때마다 에러 체크를 해줍니다. 빌드하기전에 미리 모든 컴파일 에러를 고칠 수 있습니다. 또 "cargo check"같은 명령을 사용하면 컴파일 에러가 있는지 확인하는 시간을 줄일 수 있습니다.

VSCODE를 예를 들면 Inlay hints <https://code.visualstudio.com/docs/languages/rust#_inlay-hints> 나 Linting <https://code.visualstudio.com/docs/languages/rust#_linting> 등의 기능이 있어서, cargo를 호출하기전에 코드를 쓰는 단계에서 미리 거의 모든 컴파일 에러를 잡을 수 있습니다.

또한 러스트 언어는 한번 빌드가 되고나면 좀처럼 메모리 관련 에러는 발생하지 않습니다. 기타 로우레벨 언어들로 만든 코드들이 빌드되서 실행은 되더라도 오랜 시간동안 에러가 없는지 검증해야되고, 정적 분석 툴 등을 돌려야 되는 시간들을 생각해보면 전체적인 개발 시간은 확실히 줄어드는 것이라 생각합니다.

The Rust Programming Language(<https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html>)에서는 소유권이라는 것이 3가지 규칙을 의미한다고 설명합니다.

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

제가 나름대로 번역하고 그 의미에 대해서 설명을 붙이면 이렇습니다.

- 모든 값(메모리나 변수, 데이터라고 생각해도 괜찮습니다)는 누군가에게는 소유되어야 합니다. 특정한 누군가에게 소유되지않고 누구나 마음대로 쓸 수 있는 값은 없습니다.
- 한번에 하나의 소유권자가 있을 수밖에 없습니다. 여럿이 하나의 변수를 동시에 소유할 수 없습니다.
- 소유권이 자기가 존재할 수 있는 범위(Scope라고 하는데 보통 {로 시작하고 }로 끝나는 구역을 의미합니다.) 끝나면 변수는 메모리가 해지되고 더 이상 사용할 수 없게됩니다.

함수가 대표적인 하나의 스코프입니다. 몇가지 스코프를 실험하는 예제를 만들어봤습니다.

```rust
// src/ownership_scope/main.rs
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}

fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}

fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}

fn main() {
    internal_scope();
    duplicated_names();

    println!("main starts");
    {
        println!("inner-scope starts");
        let _my: MyStruct = MyStruct {};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```bash
$ cargo run --bin ownership_scope
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/ownership_scope`
hello
world
hello again
world
hello
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

간단하게 스코프에 대한 실험을 하는 internal_scope라는 함수를 보겠습니다.

```rust
fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}
```

함수 시작 부분에서 생성된 hello_string이라는 변수는 함수가 끝나는 }를 만나면서 해지됩니다. 함수의 스코프가 끝나는 }에서 함수가 소용했던 hello_string이라는 변수가 해지되는 것입니다. "hello"가 저장되어있는 메모리가 해지되고, hello_string이라는 변수를 더 이상 사용할 수 없게 되는 것입니다.

hello_string이라는 변수는 \_internal_scope함수가 소유하고, 따라서 스코프는 \_internal_scope함수가 끝날 때까지 입니다. world_string이라는 변수의 소유권은 \_internal_scope함수안에 새로 만들어진 블럭에 있습니다. 그 새로운 블럭의 시작 지점은 두번째 {이고 끝 지점은 첫번째 }가 있는 곳입니다. 따라서 아래와같이 world_string을 소유한 블럭 밖에서 world_string을 사용할 수가 없습니다.

```rust
fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", world_string);
    println!("{} again", hello_string);
}
```

```rust
$ cargo build
error[E0425]: cannot find value `world_string` in this scope
   --> src/main.rs:15:20
    |
  7 |     println!("{}", world_string);
    |                    ^^^^^^^^^^^^ help: a local variable with a similar name exists: `hello_string`
```

world_string이 사용된 스코프는 main함수의 스코프이므로 world_string을 소유하지 않았다는 에러 메세지를 확인할 수 있습니다. 하지만 world_string이 존재하는 스코프는 main함수의 스코프이기도 하므로 hello_string변수를 사용할 수는 있습니다.

그럼 duplicated_names함수에서와 같이 같은 이름의 변수가 중첩된 스코프에 존재할 때는 어떨까요?

```rust
fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}
```

Cargo를 이용해서 코드를 실행해보면 다음과 같이 출력됩니다.

```rust
$ cargo run --bin ownership_scope
...
world
hello
...
```

2개의 변수가 동일한 이름으로 생성되지만 “hello”라는 값를 가진 변수는 main 함수가 끝나는 바깥 스코프에가 소유권을 가지고있고, “world”라는 데이터를 가진 변수는 main함수 안에서 새로 생성된 작은 스코프가 소유권을 가지고 있는 것입니다. 작은 스코프가 끝날 때 “world”라는 데이터를 가진 변수(혹은 객체)는 해지됩니다.

참고로 스코프가 끝날 때 자신이 소유한 변수들의 drop 메소드를 호출합니다. 예제에 MyStruct라는 아무런 데이터를 가지지않는 구조체를 선언하고, drop 메소드를 구현해준 코드가 있습니다. (아직 구조체에 대한 문법을 알아보지않았지만, 구조체의 선언만 보면 C언어와 거의 동일합니다. 구조체의 메소드를 정의하는 문법은 아직 모르지만, 일단 drop이라는 메소드가 호출되는 시점만 생각해보겠습니다.)

```rust
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}
......
fn main() {
    ......
    println!("main starts");
    {
        println!("inner-scope starts");
        let my: MyStruct = MyStruct{};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```bash
$ cargo run --bin ownership_scope
......
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

drop메소드가 호출되는 지점이 곧 변수의 메모리가 해지되는 지점인데, “inner-scope ends”라는 메세지 후에 drop메소드가 호출되는 것을 볼 수 있습니다. 즉 스코프 안의 모든 코드가 끝나고 스코프가 없어지는 최후의 순간에 스코프가 소유한 변수들을 해지하는 것을 확인할 수 있습니다.

### 소유권의 이동

사실 개념 설명만 들으면 약간 그래서 어쩌라는 건가 라는 생각이 들 수도 있습니다. 몇가지 제가 자주 겪어본 케이스 몇가지를 소개하겠습니다. 이정도만 일단 알고 시작하면 작은 프로젝트를 시작하는데는 문제가 없을거라 생각합니다.

#### 변수 할당에서 소유권이 이동하는 경우

가장 간단한 예는 변수간에 할당이 발생할 때 소유권이 이동하는 경우입니다.

```rust
let s1 = String::from("foo");
println!("{}", s1);
let s2 = s1;
println!("{} {}", s1, s2);
```

이 예제에서 러스트는 s1을 s2로 이동시킵니다. 보통의 언어들에서는 객체의 복사가 일어나거나 포인터의 복사가 일어날 것입니다. 러스트에서는 내부적으로는 포인터의 복사만 일어나고, 거기에 더해서 소유권의 이동까지 일어납니다. 객체 데이터를 복사하지 않기 때문에 속도는 빠르면서 소유권의 이동까지 일어나므로 데이터가 의도하지 않게 공유되는 것을 방지합니다.

그런데 실제로 뭔가를 만드는 경우에 이렇게 예제와 같이 단순하게 변수와 변수사이에 값을 옮기는 경우는 거의 없습니다. 실제로 많이 겪는 경우는 변수값의 이동이 일어나는지 잘 안보이는 경우들이 대부분입니다.

```rust
let mut user_input = String::from("페리스");
println!("{}", user_input);
let mut greeting = user_input + "씨 안녕하세요";
println!("{}", greeting);
println!("{}", user_input); // Compile error
```

기존 언어에 익숙하다보면 이 코드에 문제가 안보일 수 있습니다. 사실 안보이는게 당연한것입니다. 하지만 러스트에서는 user_inut의 소유권 이동이 일어나고, 거기에 메세지가 추가되서 greeting 변수에 저장된다는 차이가 있습니다.

```rust
error[E0382]: borrow of moved value: `user_input`
   --> src/main.rs:175:20
    |
171 |     let mut user_input = String::from("아이유");
    |         -------------- move occurs because `user_input` has type `String`, which does not implement the `Copy` trait
172 |     println!("{}", user_input);
173 |     let mut greeting = user_input + "씨 안녕하세요";
    |                        ---------------------------- `user_input` moved due to usage in operator
174 |     println!("{}", greeting);
175 |     println!("{}", user_input); // Compile error
    |                    ^^^^^^^^^^ value borrowed here after move
```

이와같이 소유권이 이동이 안보이는 경우가 많긴합니다만, 변수간의 소유권 이동은 컴파일러가 너무나 친절하게 어디에서 이동이 발생했고, 소유권이 없는 변수를 어디에서 접근해서 에러가 발생했는지를 다 알려줍니다. 그래서 에러를 찾기도 쉽고 고치기도 어렵지 않습니다.

#### 함수 인자로 전달되고 반환값을 받을 때 소유권이 이동하는 경우

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let user = "페리스".to_string();
    let greeting = make_greeting(user);
    println!("{}", greeting);
}
```

이번에도 크게 어렵지 않은 경우입니다. user라는 변수가 make_greeting의 name이라는 인자에 바인딩되었습니다. 따라서 이전에 본 변수의 할당과 유사한 경우입니다. user라는 변수가 가진 값에 대한 소유권이 name으로 옮겼갔습니다. make_greeting 함수가 끝난 뒤에는 user 변수는 사용할 수 없습니다. greeting이라는 변수는 make_greeting이라는 함수에서 생성되었지만 main 함수로 소유권이 이동된 경우입니다.

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "페리스".to_string();
    user = make_greeting(user);
    println!("{}", user);
}
```

좀 웃기긴 하지만 위 예제는 user 변수의 소유권을 main에서 make_greeting으로 이동한 후, 다시 main으로 가져오는 예제입니다. 그냥 이런것도 가능하다는 것을 보여드린 예제입니다.

실질적으로 이렇게 함수런에 소유권이 이동하도록 구현하는 경우는 많지 않습니다. 함수를 호출할 때는 보통 객체의 레퍼런스를 전달해서 소유권을 넘기지 않습니다.

```rust
fn make_greeting(name: &str) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "아이유".to_string();
    user = make_greeting(&user);
    println!("{}", user);
}
```

이전에 알아본 레퍼런스가 이렇게 변수의 이동없이도 다른 스코프에서 변수를 사용할 수 있도록 하기 위한 방법입니다. 내부적으로는 포인터만을 전달하는 것입니다. 따라서 C/C++ 언어와 성능 차이가 없습니다. 하지만 컴파일러가 컴파일을 하는 단계에서 소유권의 이동을 체크하고 메모리 공유를 막기 때문에 성능이 좋으면서도 메모리 안전성이 좋은 언어가 된 것입니다.

러스트는 이렇게 레퍼런스를 생성하는 것을 빌렸다(Borrowing)이라고 표현합니다. 소유권의 이동없이 다른 스코프에서 사용하도록 해주는 것이니 적절한 표현이라고 생각합니다.

위 예제에서는 단순히 읽기만 가능한 불변 레퍼런스(Immutable reference)를 사용했는데요, 쓰기도 가능한 가변 레퍼런스(Mutable reference)도 있습니다.

```rust
fn make_greeting(name: &mut String) {
    name.push_str("씨 안녕하세요");
}

fn main() {
    let mut user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

mut 키워드를 함수 호출하는 부분에도 넣고, 함수 인자에도 넣어야 된다는 것을 기억해야합니다.

아래와 같이 Immutable한 변수의 Mutable reference를 만드는 것은 불가능합니다.

```rust
fn main() {
    let user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

소유자가 바꾸고 싶지 않은 변수를 빌린쪽에서 맘대로 바꾸는 것은 당연히 허용하면 안되겠지요.

레퍼런스에 대한 규칙을 요약하자면 다음과 같습니다.

- Mutable reference는 오직 하나만 존재할 수 있다.
- Immutable reference는 여러개 존재할 수 있다.
- 레퍼런스는 언제나 실제 데이터를 참조해야한다.

상식적으로 생각해도 이해가 되는 규칙입니다. 데이터를 바꿀 수 없는 Immutable reference가 여러개 있다고해도 데이터는 변하지 않으니까 상관없습니다. 데이터를 바꿀 수 있는 Mutable reference가 있다면 이 데이터는 언제든지 바뀔 수 있으므로 다른 어떠한 형태의 레퍼런스도 존재하면 안됩니다.

#### 이터레이터에서 소유권이 이동하는 경우

마지막으로 벡터나 배열같이 여러개의 데이터를 이터레이터로 접근하는 경우를 알아보겠습니다. 사실 이 경우가 가장 흔하게 실수하는 경우이고, 소유권 개념을 알았더라도 한동안은 당황하게 되는 경우입니다.

 배열이나 벡터등의 이터레이터를 만드는 메소드는 2가지가 있습니다.

- iter(): 슬라이스 이터레이터를 만듬
- into_iter(): 변수 값으로 이터레이터를 만듬

참고: <https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter>

두개가 무슨 차인지 한번 실험을 해보겠습니다. 먼저 변수의 값으로 이터레이터를 만들어보겠습니다.

```rust
fn main() {
    let user: [String;3] = ["My".to_string(),
                            "Bloody".to_string(),
                            "Valentine".to_string()];
    for c in user.into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

```rust
error[E0382]: borrow of moved value: `user`
   --> src/main.rs:8:22
    |
2   |     let user: [String;3] = ["My".to_string(),
    |         ---- move occurs because `user` has type `[String; 3]`, which does not implement the `Copy` trait
...
5   |     for c in user.into_iter() {
    |                   ----------- `user` moved due to this method call
...
8   |     println!("{:?}", user);
    |                      ^^^^ value borrowed here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `user`
   --> /Users/user/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:262:18
    |
262 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you can `clone` the value and consume it, but this might not be your desired behavior
    |
5   |     for c in user.clone().into_iter() {
    |                   ++++++++
```

Copy trait나 self, clone등 모르는 키워드들이 나와서 당황스러울 수 있습니다. 미리 into_iter가 무엇인지 모른 상태에서 이 에러메세지를 본다면 막막할 수 있습니다.

일단 지금 상태에서 into_iter말고 iter 메소드를 사용해보겠습니다.

```rust
// src/ownership_move/main.rs
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

```bash
$ cargo run --bin ownership_move
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/ownership_move`
My
Bloody
Valentine
["My", "Bloody", "Valentine"]
```

아무 이상없이 실행됩니다. 무슨 차이일까요?

into_iter를 사용했을 때의 에러 메세지를 보면 이런 말이 있습니다.

> note: `into_iter` takes ownership of the receiver `self`, which moves `user`
>

값으로 이터레이터를 만든다는 의미는 변수의 값을 이동시킨다는 의미입니다. 즉 소유권을 가져간다는 뜻입니다. for 루프에서 c라는 변수의 타입이 String이 되고, c변수가 루프를 돌때마다 배열에 있는 객체를 하나씩 소유하게 됩니다. 그리고 루프가 끝날 때마다 스코프가 끝나고, 객체가 해지됩니다. 모든 루프가 끝나면 배열 전체가 다 해지됩니다.

>
> Visual Studio Code 등의 IDE툴을 사용하면 변수의 타입이 생략된 경우 자동으로 IDE가 타입을 판단해서 보여주는 기능이 있습니다. 이 기능을 활용해보면 c가 어떤 타입이 되는지 쉽게 확인할 수 있습니다.
>

반대로 iter()는 슬라이스를 만든 후 슬라이스의 이터레이터를 만듭니다. 슬라이스는 레퍼런스이므로 소유권을 가져갈 수 없습니다. 결론적으로 소유권 이동없이 배열의 각 항목에 레퍼런스로 접근합니다. c 변수의 타입은 &String이 됩니다.

이터레이터에 대한 팁을 한가지 드리자면 iter와 into_iter가 각각 사용하는 경우가 다릅니다.

- iter: 루프 후에도 계속 데이터를 사용할 경우에 사용함
- into_iter: 벡터를 해지하고 완전히 새로운 타입의 데이터를 만들때 사용함

기존 언어를 사용해보면 배열을 만들어서 여러개의 데이터를 모아서 관리하다가 프로그램 마지막에 모든 데이터를 해지하는 패턴을 흔하게 사용합니다. 이때 루프를 돌면서 각각의 데이터를 해지하고, 마지막에 배열 자체를 해지합니다. 이럴 때 into_iter를 사용하면, 각각의 항목을 따로 해지하지않아도 각 루프의 스코프가 끝나면서 자동으로 해지됩니다. for 루프에서 만든 스코프가 끝날때 각 데이터의 스코프가 끝나기 때문입니다.

참고로 iter메소드는 불변 레퍼런스를 만듭니다. for루프안에서 user데이터를 읽기만하고 쓸수는 없습니다. 만약 user데이터를 수정하고 싶다면 iter_mut 메소드를 써서 가변 레퍼런스를 만들면됩니다. iter_mut 메소드의 메뉴얼(<https://doc.rust-lang.org/std/slice/struct.IterMut.html>)을 참고하세요.

### Clone과 소유권

좀전에 into_iter를 사용한 경우 컴파일 에러를 보면 user.clone().into_iter()로 고쳐보라는 에러 메세지가 있습니다.

```rust
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.clone().into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

이렇게 바꾸면 컴파일 에러 없이 잘 동작합니다. clone은 말 그대로 데이터를 똑같이 복사해서 사본을 만드는 것입니다. Deep copy를 한다고 생각할 수도 있습니다. 위에서는 루프를 돌기전에 user의 이름없는 복사본을 만들고 그 복사본의 into_iter 메소드를 호출해서 인터레이터를 만듭니다. 그래서 user 객체는 그대로 존재하고, 복사본만 해지됩니다. 하지만 실제 제품을 이렇게 만드는 경우는 별로 없겠지요. iter메소드를 사용하는게 더 현실적입니다.

### 변수가 저장되는 위치와 소유권의 관계

지금 단계에서 중요한 내용은 아니지만 러스트의 내부를 이해하기 위한 약간의 추가 설명을 해보겠습니다.

이전에 만들었던 피보나치 함수를 다시 읽어보겠습니다.

```rust
fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}
```

t = a+b라는 코드에서 t변수는 a와 b중 어느 변수의 소유권을 가지게되는 것일까요?

사실 정수 타입의 변수는 소유권 이동이 일어나지 않습니다. 정수나 부동 소수점 타입등과 Boolean타입은 소유권의 이동이 일어나지않습니다. 함수 호출에 인자로 사용되면 값이 복사됩니다. 또 위 코드와 같이 대입이 될때도 값이 복사됩니다.

그럼 소유권 이동이 일어나는 타입과 그런 규칙에서 예외되는 타입의 구분은 무엇일까요? 바로 변수가 스택에 할당되는가 힙에 할당되는가에 따라 결정됩니다. 스택에 할당되는 변수는 소유권 이동이 일어나지않고 대신 복사가 됩니다. 힙에 할당되는 변수는 (명시적으로 복사를 하도록 강제하지않으면) 소유권이 이동됩니다. 그럼 어떤 변수가 스택에 할당되고 어떤 변수는 힙에 할당될까요?

일반적인 숫자 (정수와 부동 소수점)과 참/거짓의 Boolean 타입은 메모리 크기가 정해져있습니다. i32이면 4바이트이고 u8이면 1바이트입니다. 이렇게 컴파일 시점에 이미 메모리 크기가 정해진 변수는 스택에 할당됩니다. 스택 메모리에 할당하는 것이 빠르고 관리가 쉽기 때문입니다. 스택에 저장된 변수들은 함수가 종료될 때 스택 영역 전체를 해지하면서 한꺼번에 해지됩니다. 따라서 메모리 누수에 대한 염려도 없고 메모리 크기가 작으므로 복사하는데도 시간이 오래 걸리지 않습니다. 따라서 굳이 소유권을 설정하지 않아도 되는 것입니다.

그와 다른게 String과 같은 구조체 타입을 들 수 있습니다. 구조체의 크기만큼 힙 영역에 메모리를 할당해서 객체를 만듭니다. 왜냐면 컴파일 시점에 String 객체에 얼마만큼의 문자데이터를 넣을지 모르기 때문입니다. 리터럴로 String 객체를 만들 때는 데이터 크기를 알 수 있겠지만, 나중에 데이터를 추가할 수 있습니다. 또 사용자 입력을 받아서 String 객체를 만들거나 네트워크에서 받은 데이터로 객체를 만들 때도 프로그램이 실행 중일 때만 데이터의 크기를 알 수 있습니다.

```rust
fn main() {
    let s = String::new();
}
```

위와 같이 s라는 변수를 만들었습니다. 이 s는 스택에 생성된 포인터 변수입니다. 64비트 CPU를 가진 시스템에서 동작한다면 스택에 8바이트 메모리 영역을 할당하고, 힙 영역에 String객체를 생성한 후 스택에 있는 8바이트 메모리 영역에 힙 영역의 주소를 저장한 것입니다. 우리가 s라는 변수를 통해 객체에 저장된 데이터를 읽으면

1. s라는 변수에서 힙 영역의 주소 값을 읽음
2. 힙 영역에서 데이터를 읽음

이와 같이 2번의 메모리 접근이 일어납니다. String객체를 변수 대입이나 함수 호출을 통해 소유권이 이동된다는 것은 물리적으로 따지면 포인터 값 (64비트 정수 값)을 복사하는 것 뿐입니다. 컴파일러가 변수 대입이나 함수 호출 등 소유권 규칙에 따른 동작이 일어날 때마다 포인터 값의 이동을 감시하고 규칙에 부합하는지를 따지는 것 뿐입니다. 결과적으로 안정적인 메모리 관리를 할 수 있으면서도 성능 감소가 없는 프로그램을 만들 수 있는 것입니다.

정리를 하자면 러스트에서 원시 타입 Primitive type으로 분류된 타입들은 이동이 아니라 복사가 일어나입니다. 어떤 타입들이 원시 타입인지는 Rust의 Standard Library 메뉴얼을 참고하시기 바랍니다.

<https://doc.rust-lang.org/std/#primitives>

C나 예전 C++을 사용해본 개발자라면 이렇게 생각하면 쉽습니다.

>
> malloc/new 등으로 할당하고 free로 해지해줘야되는 메모리나 객체를 자동으로 해지해주는 대신 소유권을 관리해줘야 한다. Primitive type은 복사가 일어나고 그 외는 이동이 발생한다.
>

모던 C++을 아는 개발자는 이렇게 생각하면 더 이해하기 쉬울 것입니다.

>
> RAII가 권장이 아니라 강제 사항이다. 모든 포인터는 스마트 포인터이다.
>

나중에 Copy trait라는게 나오는데, 미리 간단하게 말씀드리면 데이터 타입의 크기를 컴파일러가 알기 때문에 데이터의 이동이 아니라 복사를 해주는 데이터 타입들의 속성이라고 생각하면 됩니다. 컴파일러가 크기를 안다는 것은 Primitive type은 기본적으로 Copy trait를 구현하고 있다는 말입니다. 그 외의 타입들은 동적으로 크기가 바뀔 수도 있으므로 컴파일러가 Copy trait를 자동으로 구현해주지 못합니다. 동적으로 크기가 바뀌거나 또다른 객체를 포함하고있는 등의 데이터는 Clone을 사용해야합니다.

## 구조체

러스트에는 클래스가 없고 구조체만 있습니다. 구조체에 메소드를 추가할 수 있지만, 상속 기능이 없기 때문에 완전한 OOP언어는 아닙니다. 구조체는 형태는 대부분의 언어들과 크게 다를게 없습니다.

아래 예제는 다양한 구조체의 형태들을 소개하고 있습니다.

```rust
// src/struct/main.rs
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

출처: <https://doc.rust-lang.org/rust-by-example/custom_types/structs.html>

조금이라도 프로그래밍을 해보신 분들이라면 이미 잘 알고계실만한 구조체와 튜플의 모습 그대로입니다. 그나마 유닛 구조체라는게 좀 특이합니다. 아무런 내부 데이터가 없는 구조체입니다. 이건 나중에 트레이트Trait라는 클래스의 메소드와 같은 것을 사용하기 위한 구조체입니다. 클래스인데 내부 변수는 없고 메소드만 있는 클래스라고 생각할 수도 있습니다.

다른 언어와 확실히 다른게 있다면 구조체를 만들 때 인자로 사용된 객체의 소유권이 이동한다는 것입니다. 다음 예제를 실행해보겠습니다.

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{}", peter.name);
    println!("{}", name);
}
```

```rust
error[E0382]: borrow of moved value: `name`
 --> src/main.rs:6:20
  |
2 |     let name = String::from("Peter");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
3 |     let age = 27;
4 |     let peter = Person { name, age };
  |                          ---- value moved here
5 |     println!("{}", peter.name);
6 |     println!("{}", name);
  |                    ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let peter = Person { name.clone(), age };
  |                              ++++++++
```

이전에 소유권의 이동에 대해서 설명하면서 소유권이 없는 변수에 접근했을 때 보여드린 에러 메세지와 거의 동일한 형태의 에러 메세지를 다시 보게 됩니다. 각 에러 메세지가 어떤 의미인지 보겠습니다.

1. move occurs because `name` has type `String`, which does not implement the `Copy` trait: String 타입은 Copy trait라는걸 구현하지 않습니다. 컴파일러가 String타입의 메모리 크기가 얼마인지 알 수 없습니다. 지금 예제 코드는 “Peter”라는 리터럴을 String으로 만들기 때문에 메모리 크기를 알 수 있는 것처럼 보이지만, 동적으로 String을 만드는 경우를 생각하면 얼마나 긴 문자열을 생성할 지 알 수 없습니다.
2. value moved here: name 변수의 소유권이 구조체 Person을 만들 때 이동했습니다.
3. value borrowed here after move: println으로 소유권이 없는 변수에 접근했으므로 에러가 발생한 것입니다.
4. consider cloning the value if the performance cost is acceptable: name.clone()으로 복사본을 만들어서 Person에 전달하는 것도 하나의 해결책이긴합니다만 불필요하게 메모리를 더 사용하게됩니다.

요약하자면 Person이라는 객체를 만들기 위해 name이라는 String 객체를 사용했는데, name의 소유권이 peter라는 변수로 넘어갔다는 것입니다. 그래서 peter변수가 생성된 이후로는 name이라는 변수를 사용할 수 없게되었습니다.

### 메소드 정의

구조체를 만드는 방법을 봤으니 이번에는 구조체의 메소드를 정의하는 예제를 보겠습니다.

```rust
// src/struct_define_main.rs
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }
}

fn main() {
    let point1: Point = Point { x: 10.3, y: 0.4 };
    let point2: Point = Point { x: 22.5, y: 2.4 };
    let rect = Rectangle {
        top_left: point1,
        bottom_right: point2,
    };
    println!("area size={}", rect.area());
}
```

Point와 Rectangle이라는 구조체를 만듭니다. 그 다음 Rectangle 구조체의 메소드를 정의하는 impl 구문이 있습니다. 메소드를 정의할 때는 impl 키워드와 구조체 이름을 쓰고 하나의 블럭을 만듭니다. 그리고 그 블럭 안에서 &self를 첫번째 인자로 받는 함수를 만들면 메소드가 됩니다. 다른 언어들의 클래스 메소드를 만드는 것과 비슷합니다.

하나 눈여겨 볼만한건 f32라는 타입의 절대값을 구하는 abs라는 메소드가 2가지 형태로 사용된다는 것입니다.

1. 타입::메소드이름(..인자..)
2. 변수.메소드이름(..인자..)

1번 타입::메소드이름 형태는 보통 정적 메소드라고 하거나 연관 함수 Associated function 라고 부르는 것입니다. 구조체 타입에 종속되는 함수라서 구조체의 객체를 만들지 않아도 호출할 수 있습니다. 2번 변수.메소드이름 형태는 동적 메소드라고해서 객체를 반드시 만든 후에 객체를 이용해서 호출할 수 있는 메소드입니다. 그래서 첫번째 인자가 항상 &self가 됩니다.

메소드의 첫번째 인자에 &self만 사용할 수 있는게 아니라 &mut self를 쓸 수 있습니다. 구조체 내부 값을 변경하는 메소드라면 &mut self를 써야합니다. 그리고 자기 자신의 메모리를 해지하는 (원문으로는 consume이라고 표현합니다.) 메소드라면 self 인자를 갖을 것입니다. self앞에 &표시가 붙지 않으니 메소드가 자기 자신의 소유권을 전달받을거라는 표시입니다.

```rust
// src/struct_method/main.rs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 0.0, y: 0.0 },
        }
    }

    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }

    fn destroy(self) {
        // do nothing but free myself
        println!("destroyer");
    }
}

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        //println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
```

```bash
$ cargo run --bin struct_method
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/struct_method`
destroyer
area size=0 Rectangle { top_left: Point { x: 0.0, y: 0.0 }, bottom_right: Point { x: 0.0, y: 0.0 } }
```

new라는 이름의 메소드는 러스트의 코딩 관례상 빈 객체를 생성하는 메소드의 이름으로 많이 쓰입니다. 그래서 보통 정적 메소드로 구현됩니다.

destroy라는 메소드는 인자를 self로 받아오므로 객체의 소유권을 가져옵니다. 따라서 메소드가 종료된 후부터는 객체를 더 이상 쓸 수 없습니다. new같이 특별히 정해진 이름이 있는 것은 아닙니다. 그리고 destroy와 같이 명시적으로 객체를 해지하는 메소드를 만드는건 특별한 일이 아니라면 잘 쓰지 않는 방법입니다.

메소드에서 self를 이용해서 소유권을 받아오는 것을 확인해보기위해 주석 처리된 부분을 다시 코드로 바꾸고 빌드해보겠습니다.

```rust
......

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin struct_method
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0382]: borrow of moved value: `rect2`
  --> src/struct_method/main.rs:45:39
   |
39 |         let rect2 = Rectangle {
   |             ----- move occurs because `rect2` has type `Rectangle`, which does not implement the `Copy` trait
...
43 |         rect2.destroy();
   |               --------- `rect2` moved due to this method call
44 |
45 |         println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
   |                                       ^^^^^ value borrowed here after move
   |
note: `Rectangle::destroy` takes ownership of the receiver `self`, which moves `rect2`
  --> src/struct_method/main.rs:27:16
   |
27 |     fn destroy(self) {
   |                ^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `my-rust-book` (bin "struct_method") due to 1 previous error
```

이제는 조금 익숙해진 에러 메세지들이 보입니다.

### 구조체 디버깅 방법

이전 예제를 보면 Point 구조체와 Rectangle 구조체의 정의 윗줄에 #[derive(Debug)]라는 코드가 있습니다.

```rust
// src/struct_method/main.rs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

......

fn main() {
    let rect = Rectangle::new();

......

    println!("area size={} {:?}", rect.area(), rect);
}
```

이 예제를 실행하면 구조체 이름과 각 필드의 이름과 값까지 출력해줘서 굉장히 편리합니다.

```bash
$ cargo run --bin struct_method
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/struct_method`
destroyer
area size=0 Rectangle { top_left: Point { x: 0.0, y: 0.0 }, bottom_right: Point { x: 0.0, y: 0.0 } }
```

\#[derive(Debug)]라는 구문은 std::fmt::Debug (Standard library에 속한 fmt라는 모듈에 정의된 Debug라는  trait)를 자동으로 구현하라는 의미입니다. 나중에 Trait에 대해서 설명할 때 정확한 의미를 알아보겠지만, 지금은 일단 “{:?}”라는 표현식을 써서 구조체의 각 필드의 값을 출력한다고 생각하면 됩니다. 구조체의 필드가 String같은 std에 정의된 타입이면 대부분 동작합니다. 만약 구조체의 한 필드가 또 다른 구조체 타입이라면, 그 다른 구조체도 #[derive(Debug)]를 선언해주면 됩니다. Rectangle에만 #[derive(Debug)]을 사용한게 아니라 Point에도 #[derive(Debug)]를 선언한 이유가 Rectangle의 디버깅 메세지를 출력할 때 Point의 디버깅 메세지도 같이 출력되어야하기 때문입니다.

## 열거형 Enums

### 기본 열거형

열거형도 패턴 매칭과 마찬가지로 러스트를 처음 접한 개발자들이 낯설어하는 특징 중에 하나입니다. 하지만 조금만 쓰다보면 너무나 편리하고 하기 때문에 자주 쓰게됩니다.
러스트 언어다운 프로그래밍을 하려면 이 열거형를 잘 활용하는게 중요합니다.
C언어나 자바, Golang등에서 보통 열거형을 쓰는 이유는 특정 값만을 가지는 타입을 새로 만들기 위해서입니다. 아래 C언어 예제를 보겠습니다.

```c
// src/enum_basic/enum.c
#include <stdio.h>

enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
};

int main()
{
    enum WEEK today;
    today = Sunday;
    printf("%d\n", today);
    today = 22;
    printf("%d\n", today);
    return 0;
}
```

위와 같이 WEEK이라는 새로운 타입을 만들었습니다. WEEK 타입의 변수는 Sunday부터  Saturday라는 값만을 갖도록 만드는게 목표입니다. 하지만 사실 C언어의 대부분의 타입이 그렇듯이 Sunday부터 Saturday가 사실상 모두 정수값이기 때문에, today 변수에 아무 정수값이나 넣어도 문제가 없습니다. today변수에 22라는 아무 정수값이나 저장하고 사용해도 컴파일에러도 없고 잘 동작합니다. 에러를 방지할 수 있는 방법이 전혀 없습니다.

러스트의 열거형도 마찬가지로 가장 기본적인 사용법은 특정 값만을 갖는 새로운 타입을 만드는 것입니다.

```rust
// src/enum_basic/main.rs
enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn main() {
    let today: WEEK = WEEK::Sunday;
    // let tomorrow: WEEK = 1; // compile error!!!

    match today {
        WEEK::Sunday => println!("Sunday: Sleep for 10 hours"),
        WEEK::Monday => println!("Monday: Work"),
        WEEK::Tuesday => println!("Tuesday: Work"),
        WEEK::Wednesday => println!("Wednesday: Work"),
        WEEK::Thursday => println!("Thursday: Work"),
        WEEK::Friday => println!("Friday: Work"),
        WEEK::Saturday => {
            println!("Saturday: Party at Club from 22")
        }
    }
}
```

```bash
/my-rust-book$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
warning: variants `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, and `Saturday` are never constructed
 --> src/enum_basic/main.rs:3:5
  |
1 | enum WEEK {
  |      ---- variants in this enum
2 |     Sunday,
3 |     Monday,
  |     ^^^^^^
4 |     Tuesday,
  |     ^^^^^^^
5 |     Wednesday,
  |     ^^^^^^^^^
6 |     Thursday,
  |     ^^^^^^^^
7 |     Friday,
  |     ^^^^^^
8 |     Saturday,
  |     ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "enum_basic") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/enum_basic`
Sunday: Sleep for 10 hours
```

정의하고 사용하는 방법은 C언어와 거의 유사합니다. 하지만 가장 큰 차이는 WEEK타입의 변수에 정말로 WEEK타입의 값인 WEEK::Sunday부터 WEEK::Saturday값 외의 값을 저장하려고하면 컴파일 에러가 발생한다는 것입니다. WEEK타입의 인자를 받는 함수를 사용할 때도 WEEK타입의 값 외에 잘못된 값을 전달할 수 없습니다. 의도하지않은 잘못된 값을 사용하는 것을 방지해줍니다. 주석처리된 13번째 줄을 코드로 바꾸고 빌드해보겠습니다.

```rust
......
fn main() {
    let today: WEEK = WEEK::Sunday;
    let tomorrow: WEEK = 1; // compile error!!!
......
```

```bash
$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0308]: mismatched types
  --> src/enum_basic/main.rs:13:26
   |
13 |     let tomorrow: WEEK = 1; // compile error!!!
   |                   ----   ^ expected `WEEK`, found integer
   |                   |
   |                   expected due to this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `my-rust-book` (bin "enum_basic") due to 1 previous error
```

WEEK타입의 변수에 WEEK타입이 아닌 정수값을 저장할 수 없으므로 에러가 발생합니다. 타입을 확실히 구분하기 때문에 에러가 나는 것입니다.

그리고 컴파일러가 주는 경고 메세지를 보면 WEEK타입의로 선언된 값들 중에 사용되지 않는 값이 있는 것도 알려줍니다. 또 아주 중요한 기능이 있는데 패턴 매칭에서 처리가 안되는 경우가 있으면 컴파일 에러가 난다는 것입니다. 예제에있는 패턴 매칭을 보면 현재는 컴파일이 잘 되도록 만들기위해서 모든 요일을 다 처리하고 있습니다만 그중 하나라도 지우면 어떻게 될까요?

```rust
......
    match today {
        WEEK::Sunday => println!("Sunday: Sleep for 10 hours"),
        //WEEK::Monday => println!("Monday: Work"),
        WEEK::Tuesday => println!("Tuesday: Work"),
        WEEK::Wednesday => println!("Wednesday: Work"),
        WEEK::Thursday => println!("Thursday: Work"),
        WEEK::Friday => println!("Friday: Work"),
        WEEK::Saturday => {
            println!("Saturday: Party at Club from 22")
        }
    }
......
```

```bash
$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0004]: non-exhaustive patterns: `WEEK::Monday` not covered
  --> src/enum_basic/main.rs:15:11
   |
15 |     match today {
   |           ^^^^^ pattern `WEEK::Monday` not covered
   |
note: `WEEK` defined here
  --> src/enum_basic/main.rs:1:6
   |
1  | enum WEEK {
   |      ^^^^
2  |     Sunday,
3  |     Monday,
   |     ------ not covered
   = note: the matched value is of type `WEEK`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
24 ~         },
25 +         WEEK::Monday => todo!()
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `my-rust-book` (bin "enum_basic") due to 1 previous error
```

위와 같이 WEEK타입에 여러가지 값들이 있는데 그 중에 WEEK::Monday가 처리되지않고있다는 에러 메세지를 보여줍니다. 친절하게 어떻게 케이스를 추가하라고도 알려줍니다.

저는 프로젝트가 거대해지고, 다른 사람이 만든 코드를 유지보수할 경우에 실수로 모든 경우에 대한 처리를 하지 않아서 잘 드러나지않는 에러가 나는 경우를 많이 겪어봤습니다. 해결하기에 어려운 문제는 아닙니다만 릴리즈된 후에 이런 문제를 발견하면 새로운 버전을 출시해야하는 번거로움이 있고, 사용자에게 새로운 버전을 설치하라고 안내해야되는 등 후속 처리가 쉽지 않습니다. 이렇게 개발자의 실수를 컴파일러가 방지하는 것이 러스트의 디자인 철학입니다. 사람은 사람이 잘하는 것을 하고 기계는 기계가 잘하는 것을 할 수 있어서 정말 편리합니다.

### 데이터를 포함하는 열거형

Rust 언어의 열거형(Enums)은 다음과 같이 데이터를 포함할 수도 있습니다.

이전에 만든 열거형 예제에서는 각 요일마다 해야할 일이 사용자에게 출력할 메세지안에 저장되어있어서 동적으로 바꿀 수 없게되어있었습니다. 다음과 같이 각 용일마다 해야할 일 등의 정보를 저장하도록 바꿀 수 있습니다.

```rust
// src/enum_data/main.rs
#[derive(Debug)]
enum WEEK {
    Sunday(String, i32),
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday {
        what: String,
        place: String,
        when: i32,
    },
}

fn main() {
    let schedule: [WEEK; 2] = [
        WEEK::Sunday("Sleep".to_string(), 10),
        WEEK::Saturday {
            what: "Party".to_string(),
            place: "Club".to_string(),
            when: 22,
        },
    ];

    for day in schedule.into_iter() {
        match day {
            WEEK::Sunday(todo, hours) => println!("Sunday: do {} for {} hours", todo, hours),
            WEEK::Monday(todo) => println!("Monday: do {}", todo),
            WEEK::Tuesday(todo) => println!("Tuesday: do {}", todo),
            WEEK::Wednesday(todo) => println!("Wednesday: do {}", todo),
            WEEK::Thursday(todo) => println!("Thursday: do {}", todo),
            WEEK::Friday(todo) => println!("Friday: do {}", todo),
            WEEK::Saturday { what, place, when } => {
                println!("Saturday: do {} at {} from {}", what, place, when)
            }
        }
    }
}
```

이제 각 요일에 해당하는 타입은 각기 요일마다 해야할 일에 대한 정보를 String타입으로 저장할 수 있습니다. 토요일, 일요일에는 추가 정보를 저장할 수 있습니다. 각 요일에 할 일을 동적으로 지정할 수 있게 되었습니다.

일요일에는 String과 i32 두가지 데이터를 저장했습니다. 튜플처럼 각 데이터는 이름을 가지지 않습니다. 매턴 매칭에서 todo, hours라고 임시로 이름을 지어서 각 데이터를 지정해줬습니다만 아무 이름이나 사용할 수 있습니다. 하지만 토요일에는 할일, 장소, 시간을 저장하는데 마치 구조체처럼 각 필드마다 이름을 지정했습니다. 패턴 매칭에서 토요일을 패턴 매칭할때 WEEK::Saturday에서 정의된 각 필드 이름 what, place, when을 그대로 똑같이 사용해야한다는 것을 주의하세요.

한가지 더 생각해볼 것은 schedule 배열을 순회할때 into_iter 메소드를 사용했다는 것입니다. C언어였으면 각 요일마다 메세지 출력 후에 내부 데이터를 해지하고, 배열을 해지하는 등 메모리를 일일이 신경써줘야되었지만, 러스트에서는 그냥 이렇게 소유권을 가져가서 처리하고 스코프를 닫기만 하면 사용한 모든 데이터가 다 자동으로 해지됩니다. 여러 쓰레드간에 메세지를 주고받는 경우를 생각해보세요. 최종적으로 메세지를 해지해야되는 쓰레드는 그냥 데이터를 소유권을 전달받으면됩니다. 다른 쓰레드에는 참조만 전달하면 절대로 데이터를 해지할 수 없습니다. 이렇게하면 개발자가 잘못된 쓰레드에서 데이터를 해지하는 실수도 방지되고, 반대로 데이터를 해지해야하는데 해지하지 않는 실수도 방지할 수 있습니다. 소유권을 전달할지, 참조를 전달할지, 참조를 전달하는데 가변 참조를 전달할지 불변 참조를 전달할지 설계단계에서만 잘 결정하면 구현단계에서는 잘못될 일이 없어지는 것입니다.

## 에러 처리를 위한 Result

열거형의 기본 정의에 대해서 알아봤으니 열거형 타입의 데이터 구조 중에 가장 많이 사용되는 Result에 대해서 이야기하겠습니다.

Result가 실제로 어떻게 정의된 것인지 소스 코드부터 보겠습니다.

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

출처: <https://doc.rust-lang.org/std/result/>

Result는 프로그램 실행 중 발생한 에러를 표현하는 타입입니다. 그 중 가장 대표적인 예가 함수의 반환값입니다.  Result에는 2개의 타입이 존재합니다. (영어로는 variant라고 부르지만 이 책에서는 타입이라고 부르겠습니다.) Ok는 함수가 동적에 성공했을 때, 함수가 반환하는 값을 내장하는 타입이고, Err는 함수가 실패했음을 나타내는 값을 내장하는 타입입니다. 함수의 실패를 나타내는 에러메세지가 될 수도 있고, 에러 상태를 나타내는 데이터가 될 수도 있겠지요.

아주 간단한 예제부터 보겠습니다.

```rust
// src/result_enum/main.rs
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}

fn main() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}
```

```bash
$ cargo run --bin result_enum
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/result_enum`
Error: denominator cannot be zero
```

divide함수는 나눗셈이 정상적으로 처리되었으면 Ok안에 결과 값을 전달하고, 나눗셈을 실행할 수 없는 에러 상황을 만나면 Err타입에 에러 메세지를 넣어서 전달합니다. main함수는 반환값의 타입을 보고, divide함수가 반환한 값이 정상적인 결과인지 문제가 발생한 상황인지를 알 수 있습니다. 타입을 확인하는 것은 패턴 매칭을 이용하면 항상 모든 에러 값을 놓치지 않고 처리할 수 있습니다. 여기서 패턴 매칭의 편리함과 강력함을 다시 느끼게 됩니다.

사실 C/C++언어에서 포인터를 반환하는 함수들이 에러 상황에 NULL (사실은 정수 0을 다른 이름으로 바꾸기만 한 것)을 반환하는게 보통인데 이게 에러 상황인 것은 나타낼 수 있지만, 왜 에러가 발생했는지를 표현할 수도없고, 실수하기도 쉬운 불편한 방식이었습니다. NULL이라는 개념을 처음 만들었다는 Tony Hoare님의 후회한다고(<https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/>) 이야기한 것도 그렇고 모던C++ (C++ 17)에서 optional, expected 등을 도입하는 것 등을 보면 Result를 잘 활용하는 것이 얼마나 프로그램의 안정성에 필수적인지 알 수 있습니다.

반드시 반환값을 갖는 함수는 최대한 전부 Result타입으로 반환하도록 작성하려고 노력해보세요. 참고로 Result에서는 한가지 타입의 에러만 반환할 수 있습니다. divide함수에서 반환할 수 있는 에러는 String타입 뿐입니다. 만약에 좀더 긴 함수를 작성하고있고, 이 함수가 몇가지 라이브러리를 호출하는데, 각 라이브러리마다 반환하는 에러의 타입이 다르다면 어떻게 해야할까요? 각 라이브러리마다 자신의 에러를 표현하기 위한 구조체를 만들어서 사용한다면, 모든 에러 값들을 하나의 타입으로 또다시 바꿔야할까요? 뒤에나올 trait라는 것을 사용해서 다양한 에러 타입들을 하나의 타입으로 표현할 수 있습니다. 지금은 어떤 상황에서도 Result를 사용할 수 있다는 것만 기억하시기 바랍니다.

### 반환값이 없는 함수에서 Result를 사용하는 방법

그럼 반환값이 없는 함수는 Result를 쓸 필요가 없을까요? 다음과 같은 경우를 생각해보겠습니다.

```rust
fn check_command_valid(cmd: &str) -> Result<(), String> {
    match cmd {
        "good" => Ok(()),
        "unsupported" => Err("Unsupported command".to_owned()),
        "bad" => Err("Bad command".to_owned()),
        _ => Err("Wierd command".to_owned()),
    }
}

fn main() {
    match check_command_valid("blabla") {
        Ok(_) => (),
        Err(error_msg) => println!("Command failed because it is a {}", error_msg),
    }
}
```

```bash
$ cargo run --bin result_noreturn
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/result_noreturn`
Command failed because it is a Wierd command
```

cmd로 전달받은 명령어에 문제가 있다면 에러 메세지를 반환하는 함수입니다. 그리고 문제가 없을 때는 아무 반환값도 없습니다. 이렇게 반환값이 없는 함수라 하더라도 성공했는지 실패했는지, 실패했으면 어떤 에러 상황인지 등의 정보를 전달해야할 때가 많습니다. 이럴 때는 위 예제와 같이 비어있는 값 ()를 반환하도록 하면 됩니다. 그리고 패턴 매칭에서는 ()와 매칭되도록하면 아무런 처리도 하지 않게됩니다.

## 함수 결과값 반환을 위한 Option

Result는 특정 처리가 성공했냐 실패했냐를 표현할 수 있었습니다. 그런데 모든게 다 성공과 실패로 판단되는 것은 아닙니다. 예를 들어 어떤 프로그램이 이전에 기록했던 파일을 다시 읽는 경우를 생각해보겠습니다. 프로그램이 종료될때마다 어디까지 실행했었고, 결과값이 뭐였었는지 등을 기록합니다. 그리고 프로그램을 다시 시작하면 이전 결과 파일을 읽어서 이어서 처리를 하게됩니다. 그런데 프로그램이 설치된 후 최초로 실행되를 경우는 어떡할까요? 프로그램이 처음으로 실행되는 경우에는 파일이 없을 수 있습니다. 그런 경우는 실패도 아니고 에러 상황도 아닙니다. 굳이 따지자면 에러 상황으로 처리할 수도 있지만 좋은 방법은 아닙니다. 왜냐면 프로그램의 설치가 잘못되어서 파일시스템을 못읽거나 다른 에러때문에 파일이 있어도 못읽는 것과는 다른 것이기 때문입니다. 이와같이 에러는 아니지만 예외적인 경우가 있을 수 있습니다. 러스트는 이런 경우의 처리를 위해 Option이라는 열거형 타입을 제공합니다.

Option의 정의는 값이 있고 없고를 표현하는 타입입니다. 실제로 어떻게 정의된 것인지 소스 코드를 먼저 확인해보겠습니다.

```rust
enum Option<T> {
    Some(T),
    None,
}

```

출처: <https://doc.rust-lang.org/std/option/enum.Option.html>

값이 있을 때는 Some타입 안에 존재하는 값을 저장하고, 값이 없을 때는 None으로 표현합니다. 가장 많이 사용하는 경우가 함수 반환 값을 Option으로 반환하는 것입니다. Result와 마찬가지로 되도록 모든 함수의 반환값을 Option으로 처리할 수 있도록 노력해야합니다.

이제 사용 예제를 한번 보겠습니다.

```rust
fn second(s: &[i32]) -> Option<i32> {
    if s.len() == 0 {
        None
    } else {
        Some(s[1])
    }
}

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is not present"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is not present"),
    }

    if let Some(n) = x {
        println!("x is {}", n);
    }

    if let Some(n) = y {
        println!("y is {}", n);
    } else {
        println!("y is not present");
    }

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap());
    //println!("y is {}", y.unwrap()); // panic!!!

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap_or(-1));
    println!("y is {}", y.unwrap_or_default());

    let y: Option<i32> = second(&[]);
    let item = y.expect("An argument of second should not be empty");
    println!("This line is not reachable because item is {}", item);
}
```

```bash
$ cargo run --bin option_enum
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/option_enum`
x is 5
y is not present
x is 5
y is not present
x is 5
x is 5
y is 0
thread 'main' panicked at src/option_enum/main.rs:46:18:
An argument of second should not be empty
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

사용법 자체는 크게 어렵지 않습니다. 패턴 매칭을 사용해서 결과 값을 확인하는 것도 Result에서 해본 방식입니다. 패턴 매칭을 해서 Some타입이면 내부 데이터를 꺼내서 사용하면 됩니다. 만약에 None이면 아무런 데이터도 없는 것이므로, 데이터가 없는 경우에 대한 처리를 하면 됩니다.

```rust
......
    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is not present"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is not present"),
    }
......
```

x는 Some타입이면서 5라는 값을 내장하고 있습니다. 그러므로 5를 출력하면 됩니다. y는 아무런 값도 없는 None타입입니다. 어떤 값도 들어있지 않으므로, 값이 없다는 안내 메세지를 출력합니다.

>
> C언어를 오래 사용하다보면 에러 값과 값이 없는 것이 혼동될 수도 있습니다. C언어에는 초기화되지 않는 변수라해도 0이나 쓰레기값이 들어있을 수밖에 없습니다. 하지만 그것이 실제 데이터가 있는 것인지, 아니면 초기화되지 않은 상태인건지 구분할 수 없다는 것은 이해하실 것입니다. 러스트는 초기화되지 않은 변수를 허용하지않습니다. 만약 데이터가 없는 상태일 수도 있는 변수나 함수의 결과값을 저장할 때는 Option을 사용하면 됩니다.
> 저는 이와 관련해서 인터넷에 떠도는 화장실 유머가 가장 이런 상태를 잘 표현한다고 생각합니다. 화장실에 갔는데 화장지가 1칸 남아있다면 결과 값은 1입니다. 만약 화장지를 앞사람이 다 써서 화장지 심만 남아있다면 결과 값은 0입니다. 만약에 화장지 심도 없고 심지어 화장지 걸이도 없다면? 결과값은 None입니다. C언어에서는 화장지 걸이가 없는 상태를 나타낼 수 있는 값이 없습니다. NULL은 사실상 정수 0입니다. 0은 0이라는 값이고요. 흔하게 에러를 표시할 때 쓰는 -1은 화장지라는 물질을 나타낼 수 없으므로 사용할 수 없는 값입니다. 이런 상태를 가장 정확하게 표현하는 것은 None뿐입니다.
>

그 외에 처음 소개되는 방법이 if let을 사용해서 값을 확인하는 방법입니다. if let 을 사용하면 값이 존재할 때의 처리를 할 수 있고, else에서는 값이 없을 때의 처리를 할 수 있습니다.

```rust
    if let Some(n) = x {
        println!("x is {}", n);
    }

    if let Some(n) = y {
        println!("y is {}", n);
    } else {
        println!("y is not present");
    }
```

x는 값이 있을때만을 처리할 때 사용합니다. 값이 없다면 무시합니다. y는 값이 들어있다면 그 값을 출력하고, 없다면 값이 없다는 메세지를 출력합니다. if let 구문에서 Some안에 있는 값을 n으로 표시하게되므로 내부 스코프에서 n은 i32타입입니다. n이라는 변수는 언제나 반드시 어떠한 값을 가집니다. 또다시 None인지 확인할 필요가 없이 확실하게 데이터를 가지는 변수가 됩니다.

### Option이 제공하는 메소드들

제가 처음 러스트를 접하면서 겪은 바로는 처음에 Option을 사용하면 값을 여러번 읽을 때마다 매번 if let이나 패턴 매칭을 사용해서 값이 있는지 없는지를 확인하게되는게 번거로웠습니다. 그래서 저는 간단한 코드를 만들 때는 unwrap이라는 메소드를 자주 사용했었습니다.

>
> 열거형도 구조체와 같이 메소드를 가질 수 있습니다. C언어에서는 열거형을 그다지 많이 사용하지 않지만, 러스트언어에서는 최대한 자신만의 타입을 만들어서 데이터를 표현하도록 권하고있고, 열거형도 아주 자주 사용됩니다.
>

unwrap 메소드는 Option 안에 존재하는 값을 꺼내주는 일을 합니다. 만약 Some안에 값이 있다면 값을 반환해주는데, None이라면 패닉을 발생시키고 프로그램을 멈춥니다. 따라서 반드시 값이 있는 상황에서만 사용해야 합니다. C언어의 assert와 같은 일을 한다고도 생각할 수 있습니다. 하지만 assert를 너무 많이 사용하거나 실제품에 사용하는건 좋지 않겠지요.

```rust
let x: Option<i32> = Some(5);
let _y: Option<i32> = None;

println!("x is {}", x.unwrap());
//println!("y is {}", y.unwrap()); // panic!!!

```

사용법은 간단합니다. unwrap이라는 메소드를 호출하기만 하면 됩니다. 물론 실제 제품을 개발하는데 unwrap을 사용하면 안됩니다. 사용한다고해도 최대한 가장 상위 레이어나 main함수에서만 사용하는게 좋습니다. 그리고 굳이 Option에서 값을 꺼내는게 필요하다면 unwrap_or나 unwrap_or_default 등을 사용하면 됩니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

println!("x is {}", x.unwrap_or(-1));
println!("y is {}", y.unwrap_or_default());

```

i32타입의 디폴트 값은 0입니다. 따라서 “y is 0”이라는 메세지가 출력됩니다.

그리고 unwrap보다 더 권장되는 방식이  expect메소드입니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
    
let item = y.expect("slice should not be empty");
```

unwrap은 패닉만 발생시킵니다. 패닉이 발생한 소스 코드 위치는 알 수 있지만 어떤 상황인지 판단할 정보가 부족할 때가 많습니다. expect를 사용하면 직접 에러 메세지를 추가할 수 있습니다. 여기에 다양한 정보를 추가한다면 문제 해결에 큰 도움이 될 수 있습니다.

>
> 제가 굳이 Option에서 값을 꺼내야한다면이라고 표현한 것은 거의 모든 경우에 Option에서 값을 꺼낼 필요가 없기 때문입니다. 그냥 변수에 Option이 들어있는 그대로 사용하지면 됩니다. 내부 값을 읽을때는 if let이나 패턴 매칭으로 값을 읽으면 됩니다. 하지만 나중에 설명한 map같은 메소드를 사용해서 내부값을 가지고 연산을 한 후 다시 결과값을 Option으로 저장하면 됩니다. 최대한 많은 변수들이 i32, String같은 객체를 직접 저장하는게 아니라 Some이나 Ok타입의 내부에 값을 갖도록 관리하는게 좀더 러스트다운 프로그램입니다.
>

## ? 연산자

Result와 Option타입을 배우고 나면 보통 이렇게 프로그래밍을 하게됩니다.

```rust
fn foo() -> Result<i32, String> {
    let r = bar();
    match r {
        Ok(n) => {
            println!("Do something with {}", n);
            return Ok(1);
        }
        Err(s) => {
            println!("Do error handling with {}", s);
            return Err(s);
        }
    }
}

fn bar() -> Result<i32, String> {
    let r = foobar();
    match r {
        Ok(n) => {
            println!("Do something with {}", n);
            return Ok(1);
        }
        Err(s) => {
            println!("Do error handling with {}", s);
            return Err(s);
        }
    }
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}

fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}
```

매번 함수나 라이브러리를 호출할 때마다 패턴 매칭을 실행하고, 하위 레벨 함수에서 받은 에러 값을 그대로 다시 상위 레벨로 전달하는 일을 한다는게 이상하지 않나요? 저는 러스트를 접하기 전까지 저는 C/C++이나 파이썬 프로그래밍을 하면서 지금까지 이렇게 하위 레이어에서 발생한 에러를 하나하나 확인해가면서 상위 레이러로 전달하는게 어쩔 수 없는 필요악이라고 생각했습니다. 개발자가 하위 레벨의 에러를 상위 레벨로 전달하는 것을 깜빡하는 실수를 하는 것을 원천적으로 방지할 방법이 없다고 생각했었습니다.

그런데 러스트 언어에서는 ? 연산자(보통은 try 연산자라고 부르는데 물음표(question mark) 연산자라고 부르기도 합니다)를 제공해줘서, Result나 Option에서의 에러값(Err나 None 모두)를 전달하는 것을 편리하게 해줬습니다. 러스트같이 함수의 반환값을 암묵적으로 무시하지 못하는 언어에서는 정말 필수적인 연산자라고 생각합니다. 이 연산자가 없었으면 위의 예제처럼 아주 많은 함수들이 똑같은 패턴매칭 코드를 반복해서 사용할 수 밖에 없었을 것입니다.

위의 예제를 ? 연산자를 이용해서 아래와 같이 바뀔 수 있습니다.

```rust
// src/try_operator/main.rs
fn foo() -> Result<i32, String> {
    let r = bar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn bar() -> Result<i32, String> {
    let r = foobar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}

fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}
```

```bash
$ cargo run --bin try_operator
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/try_operator`
Do error handling with foobar error
```

에러를 확인하는 패턴 매칭 코드를 모두 없앨 수 있었습니다. ?연산자는 값이 None이거나 Err타입이면 바로 현재 함수의 반환값으로 반환해버립니다. 아니면 Ok나 Some안에 저장된 원래 값을 꺼내서 반환해서 계속 처리를 진행하게 해줍니다. 에러 처리뿐 아니라 unwrap이 하는 일까지 같이 해주는 것입니다. 코드로 생각하면 아래와 같은 일을 ? 한문자로 처리하는 것입니다.

```rust
let r = match expr {
    Ok(value) => value,
    Err(err) => return Err(err),
}
```

?연산자의 전형적인 사용 예제를 하나 더 보겠습니다.

```rust
use std::fs::File;
use std::io::prelude::*;

fn read_file_contents(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}
```

read_file_contents 함수는 std::fs::File 라이브러리의 함수를 호출 할 때마다 ?연산자로 에러 처리를 합니다. 에러가 발생하면 에러가 무엇이든 상관없이 바로 상위 함수로 반환됩니다. ?연산자가 없었다면 read_file_contents 함수에 최소 2개의 match 표현식이 들어갔을 것입니다.

물론 단점도 있습니다. 지금까지의 예제처럼 에러값을 그대로 상위 레벨로 전달하는 경우보다는, 중간 중간 에러 처리를 하는 경우도 많기 때문입니다. 위의 예제에서도 파일을 닫거나 새로 생성하거나 하는 에러 처리를 해야한다면 ?연산자는 쓸 수 없습니다. 그래도 제 경험상 에러 처리 코드가 절반가까이 줄어들 수 있었습니다. ?연산자를 더 잘 활용하기 위해 함수를 더 잘게 쪼개고 디자인을 바꾸다보면 더 유연한 코드가 되기도 하니까 적극적으로 활용하시기 바랍니다.

## 프로젝트 관리

이번 장에서는 여러개의 파일에 코드를 나누어서 관리하는 방법을 알아보겠습니다.

### 크레이트(Crate)와 패키지(Package)

러스트 컴파일러(rustc)가 한번에 처리하는 코드를 크레이트라고 정의한다고 합니다. (출처: <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>) 사실 정의만 보면 잘 이해가 안되는데 쉽게 말해서 지금 내가 만들고있는게 하나의 실행 파일이나, 하나의 라이브러리이면 각각이 바로 하나의 크레이트입니다.

우리는 지금까지 하나의 실행파일이 생성되는 예제들을 만들었습니다. 그럼 지금까지 하나의 크레이트를 만들었다는 것입니다.바이너리 크레이트( Binary crate)는 말 그대로 실행 파일 하나를 만드는 코드입니다. 라이브러리 크레이트(Library crate)는 라이브러리를 만들기 위한 코드입니다. 코드가 파일 하나에만 있던지 여러개에 있던지는 상관없습니다. 여러 코드 파일들이 하나의 결과물을 만들면, 모든 파일이 하나의 크레이트를 구현하는 것입니다.

패키지는 필요에 따라 여러 크레이트를 모아놓은 것입니다. Cargo를 이용해서 빌드를 하면 Cargo.toml파일에 가장 먼저 [package]라고 패키지 정보를 셋팅합니다. 그것은 내가 Cargo를 이용해서 하나의 패키지를 만든다는 뜻입니다.

그런데 왜 패키지일까요? 예제 프로그램만 만들다보면 다른 라이브러리를 사용할 일이 없었을 것입니다. 그럼 하나의 크레이트만 있는 패키지를 만드신 것입니다. 그리고 그 하나의 크레이트가 하나의 패키지입니다. Cargo.toml파일의 [dependendies] 섹션에 외부 라이브러리를 추가하게되면, 하나의 바이너리 크레이트와 여러개의 외부 라이브러리 크레이트로 이루어진 패키지를 만들게되는 것입니다. 당연히 여러개의 바이너리를 하나의 Cargo.toml에서 빌드할 수 있습니다. 그럼 여러개의 라이브러리 크레이트와 여러개의 바이너리 크레이트로 구성된 패키지를 만드는 것입니다.

이전에 Cargo를 사용해서 패키지 디렉토리를 생성하는 방법을 이야기했었습니다. cargo new &lt;package-name&gt; 명령을 사용하면 된다고 이야기했었는데요 사실은 —bin옵션을 생략한 것입니다.

```bash
$ cargo new mybin --bin
     Created binary (application) `mybin` package
$ ls -R mybin
Cargo.toml  src

mybin/src:
main.rs
```

내가 지정한 mybin이라는 이름의 디렉토리를 만들고, mybin이라는 패키지를 만드는 Cargo.toml을 생성합니다. 최종 생성할 실행 파일의 이름도 mybin이 됩니다.

라이브러리 패키지를 만들때는 —-lib옵션을 사용합니다. src/main.rs대신에 src/lib.rs를 만들어줍니다.

```bash
$ cargo new mylib --lib
     Created library `mylib` package
$ ls -R mylib
Cargo.toml  src

mylib/src:
lib.rs
```

새로 생성되는 디렉토리 이름도 mylib이고 패키지의 이름도, 라이브러리 파일의 이름도 mylib이 됩니다.

### Modules 모듈

패키지와 크레이트는 라이브러리나 실행 파일등의 최종 결과물을 생성하는 단위입니다. 하나의 프로젝트 안에서 여러개의 파일들이 있을때, 다른 파일의 코드를 참조하는 방법은 모듈이라는 방식을 사용합니다. 네임스페이스에 익숙한 분들은 비슷한 것이라고 생각해도 될듯합니다.

아래 예제를 보면 네임스페이스나 기타 언어들이 다른 파일의 함수나 변수등에 접근하는 방식과 유사하다는 것을 알 수 있습니다.

```rust
fn main() {
    my_module::test_my_mod();
}

mod my_module {
    pub fn test_my_mod() {
        println!("This is my_module::test_my_mod()");
    }
}
```

예제에서 my_module이라는 모듈안에 구현된 test_my_mod 함수는 pub이라는 키워드를 붙여야 모듈 밖에서도 참조가 가능합니다. 그리고 특정 모듈안의 함수 등을 참조할 때는 <모듈이름>::<이름> 같은 방식으로 접근이 가능합니다.

만약 모듈 이름이 길거나 모듈안에 다른 모듈이 있거나 해서 이름이 길어지는 경우 아래와같이 use 키워드를 사용해서 모듈 경로를 생략할 수도 있습니다.

```rust
use my_module::test_my_mod;

fn main() {
    test_my_mod();
}

mod my_module {
    pub fn test_my_mod() {
        println!("This is my_module::test_my_mod()");
    }
}
```

그럼 다른 파일에 있는 함수 등은 어떻게 접근할까요? 실험을 위해 아래와 같이 my_module.rs 파일을 새로 추가합니다.

```bash
% ls src
main.rs         my_module.rs
```

my_module.rs 파일에 아래와같이  my_module에 정의했던 함수들을 옮겨줍니다. 주이할 것은 mod my_module 선언을 따로 해주지않고 바로 함수 정의를 시작한다는 것입니다. 파일 하나가 하나의 모듈이 되기 때문입니다. 파일 이름이 my_module.rs이기때문에 my_module이라는 모듈이 자동으로 선언된 것입니다.

```rust
pub fn test_my_mod() {
    println!("This is my_module::test_my_mod()");
}
```

main.rs에서 my_module을 참조하기 위해서는 아래와같이 mod <모듈 이름>을 사용합니다.

```rust
mod my_module;

fn main() {
    my_module::test_my_mod();
}
```

use 키워드를 사용할 수도있는데 mod로 모듈 참조를 선언한 이후에 use 키워드를 사용할 수 있습니다.

```rust
mod my_module;
use my_module::test_my_mod;

fn main() {
    test_my_mod();
}
```

만약 소스 디렉토리를 분리하고 싶다면 아래와 같이 각 하위 디렉토리마다 mod.rs라는 파일을 만들어야 합니다. 그리고 mod.rs에 같은 디렉토리에 있는 파일들을 참조해야합니다.

간단한 실험을 위해 아래와 같이 src/second_mod 라는 디렉토리를 만듭니다. 그리고 src/second_mod 디렉토리 안에 mod.rs파일과 sec_mod_file.rs 파일을 만듭니다.

```bash
% ls -R
main.rs      my_module.rs second_mod

./second_mod:
mod.rs          sec_mod_file.rs
```

mod.rs 파일을 자신과 같은 디렉토리에 있는 모듈들을 모아서 참조하는 일을 합니다. 현재는 sec_mod_file.rs파일뿐이므로 아래와 같이 sec_mod_file.rs 파일을 public으로 참조합니다.

```rust
// src/project/second_mod/mod.rs
pub mod sec_mod_file;
```

sec_mod_file.rs에는 main에서 호출된 함수를 하나 만들어줍니다.

```rust
// src/project/second_mod/sec_mod_file.rs
pub fn second_module() {
    println!("Here second-module");
}
```

이제 main.rs에서 어떻게 참조할 수 있는지 확인해보겠습니다.

```rust
// src/project/main.rs
mod my_module;
mod second_mod;

use my_module::test_my_mod;

fn main() {
    test_my_mod();
    second_mod::sec_mod_file::second_module();
}
```

```bash
$ cargo run --bin project
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/project`
This is my_module::test_my_mod()
Here second-module
```

main.rs에서 가장 먼저 “mod second_mod” 와 같이 디렉토리 이름으로 모듈을 선언해줍니다. 그리고 main함수안에서 second_module함수를 호출하기 위해 <모듈이름>::<파일이름>::<함수이름>으로 호출해줍니다.
