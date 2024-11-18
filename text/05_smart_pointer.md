# 데이터를 자동으로 관리하는 스마트 포인터

일반 포인터는 단순히 데이터의 주소를 가지는 타입이지만, 스마트 포인터는 가리키는 데이터를 관리하기 위한 메타데이터나 메모리 관리 기능을 가지고 있기 때문에 중요합니다. 스마트 포인터는 트레이트와 제네릭을 활용해서 구현되기 때문에 스마트 포인터를 잘 활용하기 위해서는 트레이트와 제네릭을 잘 이해할 필요가 있습니다.

스마트 포인터는 사실은 일반적인 구조체입니다. 하지만 다음 2개의 트레이트를 구현하고있기 때문에 자동으로 메모리 관리를 할 수 있게되는 것입니다.

1. Deref 트레이트: 메모리 주소나 레퍼런스를 읽어서 원본 데이터에 접근하는 것이 역참조(Dereference)입니다. 이런 스마트 포인터는 사실상 구조체라고 말씀드렸습니다. 구조체는 포인터가 아니니 구조체를 통해서 데이터에 접근하려면 뭔가 추가적인 구현이 필요하겠지요. 이게 바로 Deref 트레이트입니다. *와 같은 역참조 연산자를 스마트 포인터와 사용할 수 있게 해줍니다. 잠시 후에 직접 구현해보면서 다시 이야기하겠습니다.
2. Drop 트레이트: 스마트 포인터는 자신의 스코프를 벗어나게되면 자동으로 해지되어야합니다. 러스트는 스마트 포인터가 스코프를 벗어나면 Drop 트레이트의 drop 메소드를 호출해서 스마트 포인터가 가리키는 데이터를 해지합니다. 그 외에도 파일을 닫거나하는 자원 정리 기능들을 실행할 수도 있습니다. 마찬가지로 잠시 후에 직접 구현해보겠습니다.

## 데이터를 힙 영역에 저장하는 `Box<T>`

스마트 포인터 타입이 여러가지가 있습니다만 가장 핵심적인 스마트 포인터는 `Box<T>`타입입니다. `<T>`라는 표시는 이 데이터 타입 내부에 저장되는 타입이 제네릭 타입이라는 것입니다. 자신이 원하는 어떤 데이터 타입도 저장할 수 있다는 의미입니다.

`Box<T>`의 가장 핵심적인 역할은 메모리를 힙 영역에 저장한다는 것입니다. 여기에서 러스트 언어에서 아주 중요한 개념이 나옵니다. 러스트 언어로 개발하기위해서는 데이터가 힙 영역에 저장되느냐 스택 영역에 저장되느냐를 항상 생각해야합니다. 스택과 힙 영역은 똑같은 메모리이지만 아주 중요한 차이점을 가지고 있습니다.

1. 스택 영역: 데이터의 크기를 컴파일 시점에 알 수 있어야합니다. 컴파일러가 함수 호출에 관한 스택 프레임을 만드는 컴파일 시점에 데이터를 저장할 메모리 크기를 계산합니다. 그리고 데이터의 크기는 변할 수 없습니다. 만약 스택에 저장된 데이터의 크기가 늘어난다면 스택 프레임은 망가져버릴 것입니다. 대표적으로 u32같은 기본 타입들은 항상 크기가 고정되어있습니다. 따라서 따로 스택에 저장될 수 있습니다.
2. 힙 영역: 데이터의 크기를 컴파일러가 알 수 없고, 프로그램이 동작하던 중에 결정됩니다. 데이터의 크기가 바뀔 수 있습니다.

이제 Box&lt;T&gt;의 동작 방식을 이해할 수 있습니다.

1. 소유권 관리: `Box<T>`를 사용하면 데이터의 타입이 무엇이든 상관없이 데이터를 힙 영역에 저장합니다. 그리고 이 데이터를 가리키는 포인터와 메타데이터를 스택에 저장합니다. 순수하게 Box라는 구조체 타입이 있을 것입니다. 이 Box라는 구조체타입의 각 필드는 고정된 크기를 가지는 필드들입니다. 따라서 Box라는 구조체의 객체는 스택에 저장될 수 있습니다. 그리고 T라는 데이터는 힙 영역에 저장합니다. Box는 T의 포인터와 메타데이터를 가지는 것입니다. 그리고 Box타입의 객체의 소유권이 이동하게되고, 최종적으로 Box 객체의 소유권이 사라져서 해지될 때 Drop 트레이트가 사용되어서 T타입의 객체도 같이 해지되는 것입니다.
2. 메모리 역참조: Box 객체 안에 포인터가 들어있을 것입니다. *라는 역참조 연산자는 사실 포인터 타입에 사용하는 것입니다. Box라는 구조체에 사용하면 아무것도 안됩니다. 하지만 Deref 트레이트를 구현하기 때문에 역참조 연산자를 사용했을 때 Deref 트레이트를 통해서 힙 영역에 있는 데이터를 반환하는 것입니다.

### 기본 사용법

사용법은 간단합니다. Box라는 구조체의 new 메소드에 자신이 원하는 데이터를 전달하는것 뿐입니다.

```rust
fn main() {
    let my_data = Box::new(10);
    println!("What is in the heap?: {}", my_data);
}
```

```bash
$ cargo run
   Compiling pyalgo v0.1.0 (/Users/user/study/pyalgo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.88s
     Running `target/debug/pyalgo`
What is in the heap?: 10
```

그럼 my_data라는 스마트 포인터가 만들어졌습니다. my_data라는 `Box<i32>`타입 구조체는 스택에 생성되었고, 10이 저장된 i32타입 데이터는 힙에 저장되었습니다. 크게 중요한건 아니지만 한가지 더 실험을 해보겠습니다. my_data가 스택에 저장되어있고, i32타입 데이터가 힙에 저장되어있는 것을 증명해보겠습니다. 다음 예제를 실행해보겠습니다.

```rust
fn main() {
    let my_data = Box::new(10);
    println!("What is in the heap?: {}", my_data);
    println!("At stack: {:p}", &my_data);
    println!("At heap: {:p}", my_data.as_ref());
}
```

```bash
$ cargo run
   Compiling pyalgo v0.1.0 (/Users/user/study/pyalgo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.88s
     Running `target/debug/pyalgo`
What is in the heap?: 10
At stack: 0x7ffef0c2e9b8
At heap: 0x5d21262e2b80
```

결과값을 보면 my_data라는 변수가 위치한 곳의 주소값이 0x7ffef0c2e9b8이고 Box의 as_ref메소드가 반환한 값이 0x5d21262e2b80입니다. as_ref는 메뉴얼을 찾아보면 `Box<T>`에서 &T를 반환한다고 써있습니다. 힙에 저장된 데이터의 포인터를 반환하는 것입니다. 리눅스에서는 이 메모리 값으로 힙인지 스택인지 확인할 수 있는 방법이 있습니다. 아래와같이 어플리케이션의 메모리 영역을 출력해볼 수 있습니다.

```bash
$ cat /proc/$$/maps
563bb63d6000-563bb6404000 r--p 00000000 fc:02 16909610                   /usr/bin/bash
563bb6404000-563bb64df000 r-xp 0002e000 fc:02 16909610                   /usr/bin/bash
563bb64df000-563bb6518000 r--p 00109000 fc:02 16909610                   /usr/bin/bash
563bb6518000-563bb651c000 r--p 00141000 fc:02 16909610                   /usr/bin/bash
563bb651c000-563bb6525000 rw-p 00145000 fc:02 16909610                   /usr/bin/bash
563bb6525000-563bb6530000 rw-p 00000000 00:00 0
563bb7090000-563bb71d1000 rw-p 00000000 00:00 0                          [heap]
...생략
7fffdb5c4000-7fffdb5e5000 rw-p 00000000 00:00 0                          [stack]
7fffdb5f2000-7fffdb5f6000 r--p 00000000 00:00 0                          [vvar]
7fffdb5f6000-7fffdb5f8000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]
```

어플리케이션마다 세부 값들은 다르지만 제가 실행한 쉘의 스택 영역의 주소값은 7fffdb5c4000-7fffdb5e5000이고, 힙 영역의 주소값은 55fb3f245000-55fb3f4a9000인 것을 확인할 수 있습니다. 물론 예제의 메모리 영역과 정확한 값은 다르겠지만, 모든 어플리케이션의 스택 영역의 주소값이 0x7fff로 시작하고 힙 영역의 주소값이 0x55fb로 시작하는 것은 다르지 않을 것입니다. 왜냐하면 리눅스 커널이 프로그램을 실행할 때 그렇게 지정하기 때문입니다.

어쨌든 결론적으로 예제를 실행해보면 Box라는 데이터 타입은 스택 영역에 존재하고, 포인터가 가리키는 데이터는 힙 영역에 존재하고 있음을 확인할 수 있습니다.

### `Box<T>`를 직접 구현해보기

`Box<T>`를 잘 이해하기 위해서 아주 단순한 스마트 포인터를 직접 만들어보겠습니다. `Box<T>`는 다양한 메소드들이 있지만, 우리는 스마트 포인터의 가장 핵심 기능인 역참조와 자동 메모리 해지만을 구현해보겠습니다.

```rust
// src/smart_pointer_basic/main.rs
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer");
    }
}

fn main() {
    let my_pointer = MySmartPointer::new(5);
    println!("Value: {}", *my_pointer);
    let mut mut_pointer = MySmartPointer::new(1);
    *mut_pointer = 2;
    println!("Value: {}", *mut_pointer);
}
```

```bash
$ cargo run --bin smart_pointer_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/smart_pointer_basic`
Value: 5
Value: 2
Dropping MySmartPointer
Dropping MySmartPointer
```

가장 먼저 Deref, DerefMut 두가지 트레이트를 구현하도록 선언합니다.

```rust
use std::ops::{Deref, DerefMut};
```

그리고 MySmartPointer라는 구조체를 만듭니다. 제네릭을 사용해서 제네릭 타입은 T입니다. 구조체 자체는 가장 단순하게 데이터만 포함하도록 만듭니다.

```rust
struct MySmartPointer<T>(T);
```

사실은 이 구조체는 스마트포인터가 아닙니다. 데이터를 힙에 저장하는게 아니라 스택에 저장히기 때문입니다. 하지만 힙에 메모리 할당하기 위한 라이브러리를 소개하지않았으니 흉내만 내기 위해서 스택에 데이터를 저장합니다.

MySmartPointer를 생성하는 new 메소드를 구현합니다. 우리가 원하는 데이터 x를 받아서 그대로 저장할 뿐입니다.

```rust
impl<T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}
```

이제 Deref 트레이트를 구현합니다.

```rust
impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
```

deref라는 메소드가 있는데 스마트포인터가 가리키는 데이터의 레퍼런스를 반환하도록 구현하면 됩니다. 그러면 \*연산자와 결함해서 \*&T가 되므로 결국 힙에 있는 데이터를 읽게되는 것입니다.

다음은 DerefMut 트레이트입니다. mut이라는 이름이 붙은 것에서 알 수 있듯이 데이터를 바꿀 수 있는 mutable 레퍼런스 &mut T를 반환해줍니다.

```rust
impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
```

DerefMut트레이트의 사용법은 main함수에서 볼 수 있습니다.

다음은 Drop 트레이트의 구현입니다.

```rust
impl<T> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer");
    }
}
```

사실 우리 데이터는 스택에 저장되므로 실제로 하는 일은 없습니다. 만약 메모리 할당 라이브러리를 사용해서 힙에 메모리를 할당했으면, drop 메소드에서 메모리 해지를 하면 됩니다.

다음은 main함수를 보겠습니다. 먼저 Deref트레이트를 이용해서 스마트포인터가 가리키는 데이터를 읽기만 하는 방법입니다.

```rust
fn main() {
    let my_pointer = MySmartPointer::new(5);
    println!("Value: {}", *my_pointer);
```

my_pointer는 포인터입니다. 그러므로 *연산자를 붙이면 데이터를 읽을 수 있습니다. 만약 Deref트레이트를 구현하지않았으면 그냥 일반 구조체 이름에 \*를 붙인 것이므로 아무 동작도 하지 않을 것입니다.

그 다음에는 DerefMut 트레이트를 사용하는 방법입니다.

```rust
    let mut mut_pointer = MySmartPointer::new(1);
    *mut_pointer = 2;
    println!("Value: {}", *mut_pointer);
}
```

mut_pointer라는 변수는 데이터가 바뀔 것이므로 반드시 mut 선언이 있어야합니다. 데이터를 바꾸는 방법은 포인터를 사용하는 것과 동일합니다. "*포인터이름" 형태로 사용하면 됩니다.

그리고 마지막으로 main함수의 스코프가 끝나면 두개의 스마트포인트가 모두 사라지게 됩니다. 이때 Drop 트레이트의 drop 메소드가 호출됩니다.

### Box 스마트 포인터를 활용하는 예제

스마트 포인터는 데이터를 힙 영역에 저장하기 위해 사용합니다. 데이터의 크기가 크거나 동적으로 데이터의 크기가 변한다면 스마트 포인터를 사용할 수밖에 없습니다. 대표적으로 네트워크로부터 데이터를 받아오는 경우가 있습니다. 데이터의 크기도 클 것이고, 데이터의 크기가 얼마나될지는 컴파일 시점에는 절대 알 수 없습니다.

그리고 그 외에 `Box<T>`라는 타입을 반드시 사용해야하는 경우가 하나 더 있습니다. 이전에 트레이트를 설명할 때 트레이트 객체에 대해서 이야기했었습니다. 다시 한번 이야기하면 어떤 함수가 특정한 트레이트를 구현한 타입만을 전달받을 수 있도록 만드는 방법입니다. 여러개의 구조체가 있는데 모두 A라는 트레이트를 구현했습니다. 그럼 이 구조체들을 벡터나 배열에 모아놓고 싶을 때가 있을 것입니다. 예를 들어서 GenSerialData라는 특정한 트레이트를 구현한 타입들을 하나의 벡터에 모아서 한꺼번에 처리되도록 만들고 싶으면 다음과 같이 만들 수 있습니다.

```rust
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

trait GenSerialData {
    fn get_input(&mut self);
    fn generate(&self) -> Option<&str>;
}

struct UserID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for UserID {
    fn get_input(&mut self) {
        println!("Please input {}-digits User ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}

struct ProductID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for ProductID {
    fn get_input(&mut self) {
        println!("Please input {}-digits Product ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}

fn collect_data(items: &mut [&dyn GenSerialData]) {
    for item in items.iter_mut() {
        item.get_input();
    }
}

fn generate_serial(items: &[&dyn GenSerialData]) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(item.generate().unwrap());
    }
    data
}

fn main() {
    let userid = UserID { digit: 4, id: None };
    let product = ProductID { digit: 8, id: None };

    let mut items = [&userid, &product];

    collect_data(&mut items);
    let serial = generate_serial(&items);
    println!("Serial generated: {}", serial);
}
```

```bash
$ cargo run --bin smart_pointer_application
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0308]: mismatched types
  --> src/smart_pointer_application/main.rs:73:31
   |
73 |     let mut items = [&userid, &product];
   |                               ^^^^^^^^ expected `&UserID`, found `&ProductID`
   |
   = note: expected reference `&UserID`
              found reference `&ProductID`

error[E0596]: cannot borrow `**item` as mutable, as it is behind a `&` reference
  --> src/smart_pointer_application/main.rs:57:9
   |
57 |         item.get_input();
   |         ^^^^ cannot borrow as mutable

Some errors have detailed explanations: E0308, E0596.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `my-rust-book` (bin "smart_pointer_application") due to 2 previous errors
```

이 예제는 사용자 ID과 제품 ID를 조합해서 제품의 시리얼번호를 만드는 예제입니다. UserID라는 구조체와 ProductID라는 구조체를 만들었습니다. 이 두가지 구조체는 똑같은 인터페이스를 가지고 있습니다. 프로그램을 실행하는 사용자로부터 (아마도 제품을 관리하는 영업 부서의 직원될것 같네요) 입력을 받아서 문자열로 저장합니다. 그리고 시리얼 번호를 만들 때 자신의 값을 반환해주는 것입니다. 그러므로 GenSerialData라는 트레이트를 UserID와 ProductID가 구현할 수 있습니다. 나중에 사용자ID와 제품ID외에도 정수형태의 사용 만기 날짜나 판매 날짜 등 다양한 타입의 데이터를 시리얼 번호에 넣을 수 있도록 확장이 가능합니다. 새로운 데이터의 타입을 만들고 GenSerialData라는 트레이트를 구현하기만 하면 되니까요.

핵심적인 역할을 하는 함수가 바로 collect_data와 generate_serial 함수입니다.

```rust
fn collect_data(items: &mut [&dyn GenSerialData]) {
    for item in items.iter_mut() {
        item.get_input();
    }
}

fn generate_serial(items: &[&dyn GenSerialData]) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(item.generate().unwrap());
    }
    data
}
```

트레이트 객체의 배열을 전달받아서 모든 트레이트 객체의 공통 메소드 get_input과 generate를 호출하는 함수입니다. 시리얼 번호에 들어가야되는 모든 데이터 타입에 데이터를 한꺼전에 입력하고 출력하는 루프를 가지고 있습니다.

그런데 유감스럽게도 이 예제 코드는 빌드되지 않습니다. userid와 productid의 레퍼런스를 저장하는 items에서 빌드 에러가 발생합니다. 배열이나 벡터는 동일한 타입의 데이터를 저장하는 데이터 구조입니다. 하지만 이 예제에서는 동일한 타입을 저장하는게 압니다. &userid는 UserID 구조체 타입의 레퍼런스이고, &productid는 ProductID의 레퍼런스이기 때문에 서로 다른 타입입니다. 하나의 배열에 저장될 수 없습니다. 비록 두 레퍼런스가 모두 같은 트레이트를 구현한 타입의 레퍼런스이지만, 동일한 타입은 아닙니다. 각자 따로 트레이트 객체로 사용될 수는 있지만, 하나의 배열에 저장될 수 없습니다.

하지만 이 두 객체를 `Box<T>`에 담는다면 이야기는 달라집니다. 배열에 저장되는 것은 Box타입입니다. 그럼 Box를 사용하도록 바꿔보겠습니다.

```rust
// src/smart_pointer_application/main.rs
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

trait GenSerialData {
    fn get_input(&mut self);
    fn generate(&self) -> Option<&str>;
}

struct UserID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for UserID {
    fn get_input(&mut self) {
        println!("Please input {}-digits User ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}

struct ProductID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for ProductID {
    fn get_input(&mut self) {
        println!("Please input {}-digits Product ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}

fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
    for item in items.iter_mut() {
        item.get_input();
    }
}

// &[&dyn GenSerialData] is wrong!
fn generate_serial(items: &Vec<Box<dyn GenSerialData>>) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(item.generate().unwrap());
    }
    data
}

fn main() {
    println!("hello");

    let userid = UserID { digit: 4, id: None };
    let productid = ProductID { digit: 8, id: None };

    // Vec<&dyn GenSerialData> is wrong!
    //let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(productid)];
    let mut items: [Box<dyn GenSerialData>; 2] = [Box::new(userid), Box::new(productid)];

    collect_data(&mut items);
    let serial = generate_serial(&items);
    println!("Serial generated: {}", serial);
}
```

```bash
$ cargo run --bin smart_pointer_application
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/smart_pointer_application`
hello
Please input 4-digits User ID: 
1234
Please input 8-digits Product ID: 
qwerasdf
Serial generated: 1234qwerasdf
```

이전 예제에서는 아래와 같이 배열을 사용했습니다.

```rust
fn collect_data(items: &mut [&dyn GenSerialData]) {
...
fn generate_serial(items: &[&dyn GenSerialData]) -> String {
...
    let mut items = [&userid, &product];
```

배열을 사용했던 모든 함수 인자와 items 변수 선언 부분을 Box로 바꿨습니다.

```rust
fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
...
fn generate_serial(items: &Vec<Box<dyn GenSerialData>>) -> String {
...
    let mut items: [Box<dyn GenSerialData>; 2] = [Box::new(userid), Box::new(productid)];
```

그리고 items을 배열이 아니라 벡터로 사용하고 싶다면 아래와 같이 벡터를 사용할 수도 있습니다.

```rust
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(productid)];
```

한가지 주의해야할 것이 타입 지정을 반드시 해야한다는 것입니다.

```rust
    let mut items = vec![Box::new(userid), Box::new(productid)];
    let mut items = [Box::new(userid), Box::new(productid)];
```

만약 위와 같이 타입 지정을 지운다면 컴파일러는 배열에 UserID 타입의 포인터가 저장되는 것인지, ProductID 타입의 포인터가 저장되어야하는 것인지를 판단할 수 없습니다. 반드시 트레이트 객체를 저장한다고 타입을 써주어야합니다.

## 다른 스마트 포인터 타입들

그 외에도 몇가지 스마트 포인터가 더 있습니다. 각기 다른 용도를 가지고 있습니다. 하지만 다른 스마트 포인터들은 보통 비동기 프로그래밍이나 동시성 프로그래밍에서 사용되는 것들이라서 이 책에서는 자세히 소개하지 않겠습니다. 간단히 이름과 용도만 소개하겠습니다.

1. `Rc<T>`, `Arc<T>`: 소유권을 여러개 만들어서 데이터를 공유할 수 있습니다. `Rc<T>`는 싱글쓰레드에서 트리나 그래프와 같이 서로가 서로를 참조하는 형태에 사용합니다. `Arc<T>`는 멀티쓰레드에서 여러 쓰레드가 공유하는 데이터에 사용합니다.
2. `RefCell<T>`: 어떤 데이터의 불변형(Immutable) 레퍼런스를 가지고있는 상황에서도, 데이터를 수정가능하도록 만들어줍니다. 보통은 수정하지 않지만, 정말 예외적인 경우에 수정가능하도록 만들어야될때 사용합니다.
3. `Cell<T>`: 거의 사용될 일이 없지만, 특정 객체에 대해 완전히 수정가능하면서 동시에 공유 가능한 포인터를 만듭니다. 보통 C/C++ 언어와 러스트 언어를 같이 사용할 때 C/C++의 포인터 변수에 접근하기 위해 사용합니다.
4. `Mutex<T>`, `RwLock<T>`: 이름에서 알 수 있듯이 락을 구현한 타입들입니다.
