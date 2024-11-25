# 트레이트(Trait) 소개

트레이트는 어떤 타입이든 상관없이 공통된 동작을 하도록 지정하는 방법입니다. 예를 들어 서로 다른 타입의 객체들이라고 해도 같은 트레이트을 구현하도록 만들면 결국 공통된 특징을 갖도록 만들 수 있습니다. 함수 인자로 객체를 넘길 때, 타입을 지정하는게 보통이지만, 특정한 트레이트을 구현한 타입이면 무엇이든지 전달할 수도 있게 됩니다. 함수 인자에 구조체 이름이 아니라, 어떤 트레이트의 이름을 쓸 수도 있습니다.

다른 객체지향 언어를 경험해봤다면 추상 클래스나 인터페이스와  비슷한 것이라고 이해해도 좋습니다. 다양한 타입의 객체들을 묶는 추상화를 할 수 있기도 하고, 코드를 재사용할 수도 있는 등 러스트로 규모있는 프로그램을 만들기 위해서는 필수적으로 잘 활용할 수 있어야되는 문법입니다.

아주 간단한 예제를 가지고 시작해보겠습니다. 다음 예제는 서로 다른 두 구조체 Person과 Book에 공통의 트레이트 Printable을 구현하는 예제입니다.

```rust
// src/trait/main.rs
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.get_age());
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}

struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title,
            self.author,
            self.get_age()
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}

fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}

fn main() {
    let person = Person::new("Alice", 22);
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    print_info(&person);
    print_info(&book);
}
```

```bash
$ cargo run --bin trait
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/trait`
Name: Alice, 22 years old
Title: The Rust Programming Language
Author: Steve Klabnik and Carol Nichols
Published: 20230228
```

첫번째로 Printable이라는 트레이트를 선언하는 코드가 나옵니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

Printable이라는 트레이트에는 2개의 함수와 1개의 타입이 포함됩니다. 이제 이 트레이트를 구현할 구조체들은 2개의 함수를 정의해야합니다. 그리고 Age라는 타입을 무슨 데이터 타입으로 사용할 것인지를 정해야합니다. i32같은 기본 타입을 사용할 수도있고, 새로운 구조체를 만들어서 사용할 수도있습니다. 그러니 트레이트라는 것이 반드시 특정 함수를 구현하도록 하는 것만이 아니라, 어떤 데이터 타입을 쓸지도 정할 수 있다는 것을 주의하시기 바랍니다.

그리고 Person 구조체를 정의합니다.

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}
```

우선 Person타입이 트레이트와 상관없이 가지는 Person타입만의 고유한 함수 new를 구현합니다. 그 다음으로 Person을 위해 Printable 트레이트를 구현합니다.

```rust
impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.get_age());
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}
```

사람의 나이를 의미하는 Age라는 타입을 정의합니다. 타입 이름은 Age이지만, 사실은 u32타입의 별칭(alias)이라고 생각하면 됩니다. 사람 나이는 음수가 없으니까 u32를 사용했습니다. 그리고 print 함수와 get_age함수의 구현이 있습니다. print함수는 사람의 이름과 나이를 터미널에 출력합니다. get_age함수는 Age타입을 반환해야하니까 결과적으로는 u32타입을 반환하는 것이 됩니다.

그리고 Book 구조체의 정의와, Book 구조체를 위한 Printable 트레이트의 구현이 나옵니다.

```rust
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title, self.author, self.get_age()
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}
```

Age타입을 u32로 정의했습니다. 그리고 print함수와 get_age함수의 구현이 나옵니다. print는 책의 제목과 저자, 출판날짜를 출력합니다. 사람을 위한 Printable 트레이트와 책을 위한 트레이트는 동일한 인터페이스를 가지고 내부 동작만 다릅니다.

이제 정말 중요한게 나옵니다. 바로 dyn이라는 키워드를 쓰는 트레이트 객체입니다.

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

print_info라는 함수인데, 인자의 타입이 특이합니다. dyn이라는 키워드가 있는 참조 타입입니다. “dyn Printable”는 Printable 트레이트를 구현한 모든 타입을 인자로 받는다는 의미입니다. 참조 키워드 &가 있으므로 Printable 트레이트를 구현한 타입의 참조를 인자로 받는다는 뜻이 됩니다. 그런데 주의할 것이 하나있는데 <Age=u32>라는 구문도 있습니다. 이것의 의미는 Age타입을 u32로 정의한 객체만 참조하겠다는 것이 됩니다.

정리하자면 Printable 트레이트를 구현하되 Age타입을 u32로 정의한 타입들의 참조를 인자로 받는 함수입니다. print_info함수를 호출할 때 서로 다른 타입 Person과 Book의 참조를 전달할 수 있게되는 이유가 바로, Person과 Book이 Printable 트레이트를 구현했고, Age타입을 u32로 정의했기 때문입니다. 만약 Printable 트레이트를 구현했다해도, Age의 타입이 u32이 아닌 다른 타입이었다면 print_info함수에 전달할 수 없습니다.

만약에 여러개의 트레이트를 모두 다 구현하는 타입들을 지정하고 싶다면 아래와같이 사용하면 됩니다.

```rust
fn some_function(param: &(dyn Trait1 + Trait2)) {
    // Function body
}
```

Trait1과 Trait2를 모두 구현한 타입의 레퍼런스를 인자로 받는 함수가 됩니다.

이와같이 dyn 키워드를 사용하고 트레이트 이름으로 나타내는 것을 트레이트 객체라고 부릅니다. 특정한 트레이트를 구현한 객체들을 지정한다고 이해하면 될듯합니다.

## 연관 타입(Associated Type)과 연관 함수 (Associated Function)

이전 예제에서 Printable 트레이트에 있는 Age 타입을 연관 타입이라고 부릅니다. 트레이트 구현에 공통적으로 필요한 변수가 있는데 어떤 타입이 될지는 구현에 따라 달라질 수 있습니다. print_info 함수에서와 연관 타입을 동일하게 구현한 객체들을 공통적으로 사용할 수 있습니다.

그리고 Person구조체에 있는 new 함수를 연관 함수라고 부릅니다. 자세히보면 함수의 인자에 self가 없습니다. 그러므로 특정 구조체 객체에 종속되서 동작하는게 아니라 객체가 없는 상태에서 Person::new와 같이 Person이라는 타입 이름으로만 호출할 수 있습니다. 보통 new라는 이름으로 객체를 생성하는 연관 함수를 만드는게 관례입니다.

## 트레이트 객체(Trait object)

dyn키워드는 트레이트 객체를 나타내는 키워드라고 설명했습니다. 이 트레이트 객체를 좀더 자세히 이해할 필요가 있습니다.

```rust
dyn TraitName
```

위에서본 print_info함수를 다시 보겠습니다. print_info함수는 item이라는 트레이트 객체의 print함수를 호출합니다. 타입 이름이 없는데 어떻게 구현하는 함수를 호출할 수 있을까요?

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

Person타입의 print함수는 0x1000_0000에 있고, Book타입의 print함수는 0x3000_0000에 있다고 생각해봅시다. 컴피일러는 vtable이라는 것을 만들어서 트레이트에서 구현된 함수들의 포인터를 관리합니다. (이것은 자바나 C++의 인터페이스와 동일합니다.)

```rust
book.vtable = {
    print = 0x1000_0000;
    ... 다른 함수들의 시작 주소들도 들어갈 수 있음
}
person.vtable = {
    print = 0x3000_0000;
    ... 다른 함수들의 시작 주소들도 들어갈 수 있음
}

fn print_info(item: &dyn Printable<Age = u32>) {
    item.vtable.print()
}
```

위와같이 트레이트를 구현하는 객체마다 vtable을 추가해서 트레이트 함수들의 포인터를 저장합니다. 이렇게 추가적인 테이블을 만들어서 함수 포인터를 저정하고, 요청받은 함수를 호출하는 방식을 Dynamic dispatch라고 부릅니다만 용어보다는 공통된 특성 혹은 함수들의 구현을 위해 추가적인 데이터가 들어간다는 것을 알고있는게 중요할 것입니다. 물론 메모리를 한번 더 읽게되는 단점도 있습니다만 특별하게 CPU를 많이 사용하는 연산을 수행하는 부분에서 호출되지 않는한 성능에 차이가 나지 않습니다.

## 표준 라이브러리에 포함된 트레이트

개발자들이 직접 필요한 트레이트를 만들 수 있지만, 러스트의 표준 라이브러리(The Rust Standard Library: <https://doc.rust-lang.org/std/index.html>)에서 미리 만들놓은 편리한 트레이트들이 있습니다. 표준 라이브러리라고 부르는 만큼 어떤 상황에서도 사용할 수 있을만큼 성능이나 범용적이 좋은 트레이트들입니다. 그중에서 초보 단계에서도 자주 사용하게되는 몇가지만 예제를 만들어보겠습니다.

### 객체의 내부 데이터를 출력하는 트레이트 std::fmt::{Display, Debug}

객체를 터미널에 출력하는 방법을 이야기해보겠습니다. 특히 디버깅할 때 가장 많이 사용할 것입니다. 그리고 사용자에게 데이터를 보여주는 인터렉티브한 어플리케이션을 만들다보면 객체의 값을 터미널에 출력해야될 일이 자주 생깁니다.

객체를 출력하기위해서 이미 우리가 여러번 사용해본게 있습니다. 구조체에  derive(Debug)구문을 추가해서 디버그 메세지를 출력하는걸 해봤었습니다.

```rust
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{:?}", dot);
}
```

derive라는 구문은 사전적으로는 유래하다, 파생하다라는 뜻을 가지고있는데 Point라는 구조체의 데이터를 출력하기 위한 코드를 자동으로 생성해달라는 의미가 됩니다. 물론 자동으로 생성하지않고 내가 직접 만들어서 쓸 수도 있습니다. 우리가 이전에 trait의 구현체를 만들어본것 처럼 직접 구현해볼 수 있습니다. 일단 std::fmt::Debug라는 trait가 있다는 것을 안다면 다음과 같이 구현을 시작할 수는 있겠는데, 이 트레이트의 어느 메소드를 어떻게 구현해야하는지를 어디에서 알아볼 수 있을까요?

```rust
impl fmt::Debug for Point {

}
```

아래 Debug 트레이트의 메뉴얼을 확인하면 어느 메소드를 가지고있고, 어떤 역할을 가지는지를 알 수 있습니다.

<https://doc.rust-lang.org/std/fmt/trait.Debug.html>

가장 윗부분을 보면 트레이트의 정의와 필수 메소드를 확인할 수 있습니다.

```rust
pub trait Debug {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

“Required method”라고 써놓은 것을 보니 fmt라는 함수를 구현하면 되겠네요. 그런데 사실 구현을 막상 하려고해도 f라는 인자를 어떻게 써야할지 정의만 봐서는 모를 수밖에 없습니다. 매뉴얼을 좀더 읽어보면 답이 나옵니다.

```rust
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point [{} {}]", self.x, self.y)
    }
}
```

이런 형태로 얼마든지 자유롭게 구현만하면 됩니다. fmt::Formatter라는 객체에 내가 지정한 메세지를 써주기만 하면 됩니다. 단지 주의할 것은 write!매크로 호출하는 코드에서 줄의 마지막에 “;” 세미콜론을 쓰지 않는 다는 점입니다. fmt함수의 반환값이 fmt::Result입니다. write! 매크로 함수가 fmt::Result 값을 반환하므로 “;” 세미콜론을 쓰면 안됩니다. 만약 ";"를 쓰게되면 아무런 값도 반환하지 않게되서 컴파일 에러가 발생할 것입니다.

이제 코드를 실행해보면 아래와같이 우리가 지정한 형태로 객체의 값이 출력됩니다.

```rust
// src/trait_display_debug/main.rs
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT({} {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT [{} {}]", self.x, self.y)
    }
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{:?}", dot);
    println!("For your information: {}", dot);
}
```

```bash
$ cargo run --bin trait_display_debug
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/trait_display_debug`
POINT [1.2 3.4]
For your information: POINT(1.2 3.4)
```

그런데 디버깅 용도외에도 사용자에게 특정 구조체의 값들을 보여줄 필요가 있을때가 있습니다. 단순히 내가 만든 데이터 객체의 값을 확인하기위해 디버깅 용도로 출력할 때도 있겠지만, 내가 만든 라이브러리를 다른 개발자가 가져다쓸 때, 객체의 값을 출력해볼 수도 있는 것입니다.

>
> 파이썬을 경험해보신 분들은 __repr__이나 __str__메소드과 유사한 것들이라고 생각하시면 이해하기 좋습니다.
>

예를 들어 내가 Point라는 라이브러리를 만들어서 좌표상의 점에 대한 처리를 하는 기능들을 구현했다면, 이 라이브러리의 사용자는 다음과 같이 어플리케이션을 만들 것입니다.

```rust
fn main() {
    let first_dot: Point = get_user();
    let second_dot: Point = get_user();
    println!("The distance between {} and {} is {}", 
              first_dot, second_dot, get_distance(first_dot, second_dot));
}
```

사용자가 출력하려는 메세지는 디버깅 메세지가 아닌 정식으로 터미널의 stdout에 출력하려는 메세지입니다. 이런 경우를 위해서 Debug 트레이트가 아닌 std::fmt::Display 트레이트가 존재합니다. 그런데 사실 내부 구현은 Debug 와 완전히 동일합니다. 단지 println!등에서 사용하는 출력 형태가 {:?}가 아니라 {}라는 차이 뿐입니다.

Debug와 Display는 사실상 동일한 일을 합니다만, 사용하는 의도가 다른 것입니다. 일반적으로 특정 타입을 문자열로 변환해서 사용자에게 보여주고자할 때는 Display를 사용합니다. 서로 다른 모듈이나 객체들간의 통신 인터페이스로도 사용할 수 있습니다. 하지만 개발자가 임시로 디버깅 용도로 타입의 데이터를 출력하고자할 때나 에러 메세지 등에서는 Debug를 사용하는게 의도에 맞게 사용하는 것입니다. 의도에 맞게 사용하면 코드를 읽는 다른 개발자나 미래의 나 자신에게 더 읽기 쉬운 코드가 될 것입니다.

### 같은 객체를 만들어주는 Clone

Clone 트레이트는 clone이라는 메소드를 구현하는 것인데, 간단하게 설명하면 바로 deep copy를 수행하는 것입니다. 다음은 Clone 트레이트의 정의를 매뉴얼에서 가져온 것입니다.

```rust
pub trait Clone: Sized {
    // Required method
    fn clone(&self) -> Self;

    // Provided method
    fn clone_from(&mut self, source: &Self) { ... }
}
```

한가지 먼저 알아야되는게 있습니다. self와 Self의 차이입니다. self는 객체 자신을 가르키는 변수입니다. 그리고 Self는 객체의 타입입니다. Self는 아무런 객체와 상관없은 단지 현재 트레이트를 구현하는 타입을 가르킵니다. 트레이트를 구현하는 것이 구조체이면 구조체의 이름이라고 생각할 수 있습니다.

그래서 clone함수는 자기 자신을 참조하면서, 같은 타입을 반환하는 함수입니다. 그럼 예제를 한번 만들어보겠습니다.

```rust
// src/trait_clone/main.rs
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title.clone(),
            author: self.author.clone(),
            published: self.published,
        }
    }
}

//fn print_info(item: &dyn Clone) {
//    println!("item implements Clone trait");
//}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
    book_clone.clone_from(&another);
    println!("{:?}", book_clone);

    //print_info(&book);
}
```

```bash
$ cargo run --bin trait_clone
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/trait_clone`
Book { title: "The Rust Programming Language", author: "Steve Klabnik and Carol Nichols", published: 20230228 }
Book { title: "The another book", author: "Unknown", published: 20111111 }
```

clone함수의 내용을 보면 새로운 Book 객체를 만듭니다. title필드는 self.title을 그대로 복사해야합니다. self.title은 String타입의 객체이므로 String타입의 clone메소드를 호출하면 똑같은 객체를 만들 수 있습니다. String 타입의 clone 메소드 또한 Clone 트레이트를 구현한 것이겠지요. 표준 라이브러리에 정의된 타입들은 대부분 clone 메소드를 가지고 있습니다. 만약 clone메소드를 호출하지않고 다음과 같이 만든다면 어떻게될까요?

```rust
impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title,
            author: self.author,
            published: self.published,
        }
    }
}
```

소유권에 대해서 설명할 때 이야기한 바와 같이 힙 영역 메모리에 저장된 객체들은 이와같이 대입이 될때 소유권의 이동이 일어납니다. 그래서 다음과 같은 컴파일 에러가 발생합니다.

```rust
error[E0507]: cannot move out of `self.title` which is behind a shared reference
  --> src/main.rs:43:20
   |
43 |             title: self.title,
   |                    ^^^^^^^^^^ move occurs because `self.title` has type `String`, which does not implement the `Copy` trait
```

만약 String타입에 Copy 트레이트가 구현되어있다면 자동으로 copy 메소드를 호출해서 자동으로 객체 복사를 해주게되지만, String 타입은 Copy 트레이트를 구현하지않았으므로 위와같은 에러가 발생합니다.

그리고 published는 u32타입의 변수이므로 clone이 필요없이 값이 복사됩니다.

이전 예제에서 트레이트를 구현 후 트레이트 객체를 인자로 전달하는 함수를 만들어봤습니다. 그럼 Clone도 트레이트니까 비슷하게 Clone을 구현한 객체들을 인자로 받는 함수를 만들 수 있을까요? 예제에서 주석으로된 print_info 함수를 코드로 만들어서 다시 빌드해보겠습니다.

```rust
...
fn print_info(item: &dyn Clone) {
    println!("item implements Clone trait");
}
...
```

```rust
error[E0038]: the trait `Clone` cannot be made into an object
  --> src/main.rs:50:22
   |
50 | fn print_info(item: &dyn Clone) {
   |                      ^^^^^^^^^ `Clone` cannot be made into an object
   |
   = note: the trait cannot be made into an object because it requires `Self: Sized`
   = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
```

컴파일러 메세지가 약간 이해하기 어렵지만 이렇게 생각해보면 힌트가 될 수 있습니다. 트레이트 객체로 전달한다는 것은 원래의 타입이 뭔지를 숨기고 여러 타입이 공통으로 구현한 함수를 호출하는 것입니다. 만약 구현하는 함수가 clone 메소드처럼 Self를 반환한다면, 트레이트 객체의 원래 타입이 뭔지 알아야됩니다. 따라서 clone 메소드를 구현해야하는 Clone 트레이트는 트레이트 객체로 사용할 수 없는 것입니다.

에러 메세지에서 알려주는 Object safety(<https://doc.rust-lang.org/reference/items/traits.html#object-safety>) 라는 규칙들이 있는데 트레이트 오브젝트를 만들어서 쓸 수 있는 트레이트가 가져야하는 규칙들입니다. 여기에 Sized 트레이트를 구현하면 안된다고 써있는데 Clone 트레이트는 상위 트레이트로 Sized 트레이트를 구현하고 있습니다. 위에 Clone트레이트의 정의를 보면 Clone:Sized라고 써있는데 Sized 트레이트를 상위 트레이트로 가진다는 표시입니다. Sized 트레이느는 바로 트레이트에서 사용하는 모든 타입들이 컴파일 시점에 어느 크기를 갖는지 알 수 있다는 표시입니다 컴파일 타임에 Self 타입의 크기를 알아야 메모리 복사에서 안정성을 보장할 수 있기 때문입니다. 참고로 이와같이 어떤 메소드를 구현하고자하는 트레이트가 아니라, 속성만 부여하고자하는 트레이트도 있습니다. 그리고 그런 트레이트들을 Marker trait라고 부릅니다. 그래서 Sized트레이트가 정의된 위치가 std::marker입니다.

러스트는 이와같이 메모리 안정성을 위해서 복잡해보이는 규칙들을 가지고 있습니다. 과하다싶을 때도 있지만, 이런 규칙들을 개발자가 생각하면서 개발해야하는 언어들의 문제점을 해결하는게 이토록 쉽지않은 일이라는 것을 알 수 있습니다. 처음에는 복잡해보이고 의미를 알 수 없는 규칙들이지만, 메모리 안정성을 염두해두고 연습을 해나간다면 이해할 수 있습니다. 시작단계에서는 모든 트레이트가 다 트레이트 객체를 만들 수 있지 않다는 것만 기억해두고 점차 익숙해지면서 더 이해하면 됩니다.

>
> Sized나 Copy 트레이트 등은 메모리를 세밀하게 제어할때 사용됩니다. 그 외 다양한 마커 트레이트가 있지만, 이 책의 수준을 넘어서기때문에 자세히 설명하지 않겠습니다.
>

지금까지 길게 Clone 트레이트의 구현을 설명했지만, 사실은 이렇게 직접 구현할 필요가 없습니다. derive[Clone]속성을 추가해주면 자동으로 Clone트레이트의 구현이 생성됩니다.

```rust
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    published: u32,
}
...
```

### 디폴트 값으로 객체를 생성하는 Default

Default 트레이트는 구조체의 각 필드를 디폴트값으로 초기화해서 객체를 생성해줍니다. 다음 예제는 Default 트레이트를 직접 구현하지않고 derive(Default) 속성을 추가해서 자동으로 생성된 코드를 사용한 예제입니다.

```rust
// src/trait_default/main.rs
#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book = Book::default();
    let book_clone = book.clone();
    println!("{:?}", book_clone);
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin trait_default
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/trait_default`
Book { title: "", author: "", published: 0 }
```

Default트레이트는 default라는 메소드를 가지고 있습니다. default 메소드는 새로운 객체를 만들 때 사용하므로 정적 메소드입니다. 따라서 “타입이름::default()” 형태로 호출합니다. 간혹 자동으로 생성되는 값들이 개발자의 의도와 다를 때만 직접 구현해주면 됩니다.

### 값이 같은지 비교하는 PartialEq

PartialEq는 두 객체가 같은 값을 가지고 있는지를 확인하는 트레이트입니다.

```rust
// src/trait_partialeq_first/main.rs
#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

fn main() {
    let second = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let first = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20190812,
    };

    if second == first {
        println!(
            "Yes, they are same book but different release {} != {}.",
            first.published, second.published
        );
    }
}
```

```bash
$ cargo run --bin trait_partialeq_first
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/trait_partialeq_first`
Yes, this book is writtend by Person { name: "Steve Klabnik", age: 30 }
```

위와 같이 두 책이 같은 책인지 확인하려면 책 제목과 저자 이름을 확인하게 됩니다.

트레이트 이름이 PartialEq라고해서 왜 Partial이라는 이름이 들어갔는지 의아하게 생각할 수도 있습니다. 하지만 완전하게 동일한 객체를 비교하는게 아니라 위와같이 좀더 넓은 의미에서 일부 같은 값을 가진 객체도 비교할 수 있다는 유연성을 갖는다고 이해하면 쉬울듯합니다. 바로 아래 예제를 생각해보면 됩니다.

```rust
// src/trait_partialeq_second/main.rs
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl PartialEq<Person> for Book {
    fn eq(&self, other: &Person) -> bool {
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        other.author.contains(&self.name)
    }
}

fn main() {
    let second = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    let steve = Person {
        name: "Steve Klabnik".to_string(),
        age: 30,
    };
    if second == steve {
        println!("Yes, this book is writtend by {:?}", steve);
    }
}
```

```bash
$ cargo run --bin trait_partialeq_second
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/trait_partialeq_second`
Yes, this book is writtend by Person { name: "Steve Klabnik", age: 30 }
```

Book 타입과 Person 타입은 동일한 객체가 될 수는 없습니다. 하지만 일부 같은 값을 갖는지를 비교할 수는 있겠지요. 아직 제너릭에 대한 이야기를 하지 않았지만, PartialEq에 어떤 타입 바인딩을 쓰는지에 따라서 비교할 대상의 타입을 지정할 수 있다는 것만 생각해보시면 되겠습니다.

PartialEq 외에도 Eq 트레이트가 있습니다. 하지만 보통 우리가 == 연산자로 비교할 때는 PartialEq를 사용한다는 것을 기억하시기 바랍니다.

#### PartialEq와 Eq 트레이트의 차이

러스트에서 일반적으로 두 객체가 같은지 비교하는 트레이트가 PartialEq라는 것이 이상하게 느껴질 것입니다. 왜 이름에 일부분(Partial)이라는 의미가 들어가는지, 왜 Eq가 기본이 아닌지 이상합니다. 사실 정의만 놓고 본다면 PartialEq는 Partial-equivalence 관계(<https://en.wikipedia.org/wiki/Partial_equivalence_relation>)를 확인하는 것입니다. 수학적인 표현들을 단순화시켜서 생각해보면

1. a=b일때 b=a를 만족하고
2. a=b이고 b=c일때 a=c를 만족하는지를

이렇게 2가지를 확인하는 것입니다.

그리고 Eq 트레이트는 equivalence 관계(<https://en.wikipedia.org/wiki/Equivalence_relation>)를 확인하는 것인데 마찬가지로 수학적인 표현들을 단순화시켜보면

1. a==a를 만족하는지

확인하는 것입니다.

그런데 둘다 수학적인 내용들이라 사실 의미를 알기가 어렵습니다. 사실 일반적으로 값을 비교할 때는 PartialEq를 쓰는 것이 맞습니다. 값을 비교할 수 없는 예외적인 케이스들이 있기 때문에 PartialEq라는 이름을 사용한다고 합니다. 그런 예외적인 케이스들 중에 대표적으로 소개하는 것이 부동소수점 연산에서 뭔가 연산이 잘못되었음을 나타내는 std::f32::NAN이라는 값을 비교할때 PartialEq를 사용할 수 없다는 것입니다.

assert_eq매크로를 이용해서 std::f32::NAN을 비교해보면 보기에는 서로 같으므로 문제가 없을것 같지만, 서로 같다는 것을 확인할 수 없으므로 결국 같지 않다는 결과가 되고, 패닉을 발생시킵니다.

```rust
fn main() {
    assert_eq!(std::f32::NAN, std::f32::NAN);
}
```

수학적으로 숫자 아님과 숫자 아님을 비교한다는 것이 정의되지 않았기 때문에 두 값이 같지 않다고 판단하도록 만든 것입니다. 파이썬등 대부분의 언어들도 마찬가지입니다.

그런데 사실 이게 무슨 의미인지 쉽게 이해하기 어렵습니다. 그래서 위에 책과 저자를 비교하는 예제를 조금 더 확장해보면서 생각해보겠습니다.

```rust
//src/trait_partialeq_eq/main.rs
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl PartialEq<Person> for Book {
    fn eq(&self, other: &Person) -> bool {
        if self.author.contains("Unknown") {
            return false;
        }
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        if self.name.contains("Unknown") {
            return false;
        }
        other.author.contains(&self.name)
    }
}

fn main() {
    let second = Book {
        title: String::from("Necronomicon"),
        author: String::from("Unknown"),
        published: 20230228,
    };
    let unknown = Person {
        name: "Unknown".to_string(),
        age: 30,
    };
    if second == unknown {
        println!("Yes, this book is writtend by {:?}.", unknown);
    } else {
        println!("No, we don't know who wrote it.")
    }
}
```

```bash
$ cargo run --bin trait_partialeq_eq
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `/home/gkim/study/my-rust-book/target/debug/trait_partialeq_eq`
No, we don't know who wrote it.
```

책의 저자도 Unknown이고, 사람의 이름도 Unknown입니다. 그럼 이 책 저자와 사람의 이름이 같은 것일까요? 이 책을 이 사람이 썼다고 생각할 수 없는 것입니다. 표기는 같지만 비교할 수 없는 값들이기 때문입니다. 예, 이런 경우가 있기 때문에 PartialEq이라는 이름을 사용하는 것입니다.

그럼 Eq가 필요한 경우는 언제일까요? Eq는 항상 정확하게 비교할 수 있는 경우에 사용해야합니다. "Unknown"같이 같다고 비교할 수 없는 경우가 존재하지않고 항상 어떤 값이든 비교할 수 있는 타입만 Eq를 구현할 수 있습니다. 가장 대표적으로 키와 값을 저장하는 HashMap같은 자료구조에서 키로 사용하는 타입은 반드시 Eq 트레이트가 구현되어야합니다.

#### PartialEq의 사용 예제

아래와 같이 사용자에게 한 문자로된 명령을 입력받는 프로그램을 만들어보겠습니다.  

```rust
// src/trait_partialeq_assert.main.rs
use std::io;

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r/e: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().strip_suffix("\n") {
            Some("h") => return Ok(Command::Help),
            Some("q") => return Ok(Command::Quit),
            Some("r") => return Ok(Command::Run),
            Some("e") => return Ok(Command::Execute(format!("NOT IMPLEMENTED YET!"))),
            _ => return Err(format!("Wrong input: {input}")),
        },
        Err(error) => return Err(format!("Wrong input: {error}")),
    }
}

fn main() {
    let com: Command = user_input().expect("Wrong input");
    let p = String::new();

    assert_ne!(com, Command::Execute(p));
    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
        Command::Execute(path) => println!("execute {path}"),
    }
}
```

```bash
$ cargo run --bin trait_partialeq_assert
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/trait_partialeq_assert`
input h/q/r/e: 
r
do something
```

프로그램을 실행하면 h/q/r 중에 하나를 입력하라는 메세지를 출력합니다. 만약 사용자가 r을 입력하고 엔터를 치면 “do something”이라는 메세지를 출력합니다. 실제 프로젝트라면 “do something”을 출력하게 아니라 실제로 뭔가를 실행하는 코드가 있을 것입니다.user_input함수는 사용자에게 e라는 명령을 입력받으면 Command::Execute(추가 메세지)를 반환합니다. 그리고 main함수는 거기에 맞는 처리를 실행합니다. 그런데 아직 모든 기능이 완료된게 아니라서 main함수에서 assert_ne! 매크로를 사용해서 Command::Execute 입력을 받으면 프로그램이 중단되도록 하겠습니다. 추후에 Command::Execute명령에 따라 실행될 코드를 먼저 구현한 후에 main함수에서 실행되도록 만들 계획입니다. 보통 작업중인 코드가 실행되지 않도록 할때 assert나 assert_eq, assert_ne등의 매크로를 많이 사용합니다. 다른 언어에서도 자주 사용하는 기능입니다.

이런 assert관련 매크로를 사용할 때 PartialEq의 구현이 반드시 필요합니다. 왜 필요한지 이해하기 위해 일단 Command 타입에 있는 derive를 빼고 빌드해보겠습니다. 아래와같이 이상한 에러가 발생합니다.

```Rust
...
//#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}
...
```

```bash
$ cargo run --bin trait_partialeq_assert
   Compiling ex v0.1.0 (/Users/user/ex)
error[E0369]: binary operation `==` cannot be applied to type `Command`
  --> src/main.rs:29:5
   |
29 |     assert_ne!(com, Command::Execute(p));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     Command
   |     Command
   |
note: an implementation of `PartialEq` might be missing for `Command`
  --> src/main.rs:3:1
   |
3  | pub enum Command {
   | ^^^^^^^^^^^^^^^^ must implement `PartialEq`
   = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Command` with `#[derive(PartialEq)]`
   |
3  + #[derive(PartialEq)]
4  | pub enum Command {
   |

error[E0277]: `Command` doesn't implement `Debug`
  --> src/main.rs:29:5
   |
29 |     assert_ne!(com, Command::Execute(p));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Command` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Command`
   = note: add `#[derive(Debug)]` to `Command` or manually `impl Debug for Command`
   = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Command` with `#[derive(Debug)]`
   |
3  + #[derive(Debug)]
4  | pub enum Command {
   |

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `ex` (bin "ex") due to 3 previous errors
```

가장 먼저 발생한 에러 메세지를 보면 assert_ne! 매크로가 실행될 때 ‘==’ 연산자를 이용해서 두개의 값이 같은지를 확인하는데, Command 타입은 ‘==’ 연산자를 실행할 수 없다는 말을 하고 있습니다. 그리고 친절하게도 Command타입에 PartialEq 트레이트를 구현하는게 필요하다는 설명을 해주면서 #[derive(PartialEq)]를 타입에 추가하는 것을 추천해주고 있습니다.

그 다음에는 마찬가지로 assert_ne! 매크로가 Debug 트레이트를 사용하고 있으니 PartialEq와 같이 #[derive(Debug)]를 추가해주라는 안내 메세지가 출력되었습니다.

일단 컴파일러가 알려주는대로 Debug와 PartialEq의 구현을 다시 추가해보겠습니다.

```rust
...
#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}
...
```

Debug와 PartialEq 트레이트를 구현해주었더니 문제없이 빌드되고 있습니다. 이와 같이 일반적으로 두개의 값을 비교하는 경우에 PartialEq 트레이트를 사용합니다.

#### PartialEq와 Eq의 사용 예제

그럼 일반적으로 값을 비교할 때 PartialEq를 사용하는 것으로 생각한다면 굳이 Eq 트레이트를 사용해야만 하는 경우는 무엇을까요? 바로 구조체를 HashMap의 키 값으로 사용할 때 입니다.

```rust
// src/trait_partialeq_eq_hashmap/main.rs
#[derive(PartialEq, Eq, Hash)]
struct MyKey {
    x: i32,
    y: i32,
}

struct MyVal {
    distance_from_co_origin: f32,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(
        MyKey { x: 3, y: 4 },
        MyVal {
            distance_from_co_origin: 5.0,
        },
    );
}
```

MyKey는 점의 좌표를 나타냅니다. MyVal은 원점으로부터 점의 거리를 나타냅니다. 그래서 여러 점들의 원점으로부터 거리를 관리하는 해시맵을 만들어봤습니다. 만약 MyKey 구조체에 PartialEq와 Eq 트레이트의 구현이 없다면 빌드가 안됩니다. PartialEq는 Eq의 소집합이므로 Eq구현을 위해서는 PartialEq가 항상 같이 구현되어야합니다. 그리고 HashMap에서 키 값으로 사용된다는 표시를 하기위해 Hash 트레이트도 구현합니다.

>
> HashMap에 대해서는 다음에 다시 설명하겠습니다. 이 예제에서는 Eq만 봐주시기 바랍니다.
>

### 크기를 비교하는 std::cmp::Ordering타입과 std::cmp::Ord트레이트

두 객체가 동일한지를 판단하는 std::cmp::PartialEq 트레이트를 알아봤으니 이번에는 크기를 비교하는 std::cmp::PartialOrd 트레이트에 대해서 알아보겠습니다.

그럼 간단하게 두 객체를 비교하는 예제를 보겠습니다.

```rust
// src/trait_partialord/main.rs
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {
    let alpha = Person {
        name: "alpha".to_owned(),
        age: 10,
        height: 110,
    };
    let beta = Person {
        name: "beta".to_owned(),
        age: 10,
        height: 100,
    };

    if alpha > beta {
        println!("{} is taller.", alpha.name);
    }
    if alpha.partial_cmp(&beta).unwrap() == Ordering::Greater {
        println!("{} is taller.", alpha.name);
    }
}
```

우리가 구현해야할 것은 PartialOrd 트레이트의 partial_cmp 메소드입니다. 이 메소드는 Option&lt;Ordering&gt;을 반환합니다. 왜 Option을 사용하는지는 예제 코드를 보면 알 수 있습니다. height 값이 0이거나 음수인 경우, 아직 객체가 제대로 초기화되지 않았거나 버그로 인해 객체의 데이터가 깨졌을 수 있습니다. 그렇게 단순히 비교만 하는게 아니라 비교할 수 없는 상황이 있을 수도 있기 때문에 그런 에러 상황을 처리하기위해 Option을 반환하도록 했습니다. Rust언어가 얼마나 에러 처리에 철저한지 알 수 있습니다.
그리고 PartialOrd 트레이트를 사용하기 위해 한가지 더 새롭게 알아야할 것이 있습니다.

partial_cmp메소드에서 가장 먼저 에러 상황을 확인한 이후에는 실제로 비교해야할 데이터를 비교합니다. std::cmp::PartialOrd라는 트레이트의 partial_cmp 메소드의 반환값이 std::cmp::Ordering이라는 타입입니다. Ordering은 enum 타입으로 Greater, Less, Equal이라는 3가지 타입을 가지고 있습니다. 메소드를 호출하는 객체가 더 크다면 Ordering::Greater를 반환합니다. 작으면 Ordering::Less, 같으면 Ordering::Equal을 반환합니다.

main함수에서는 객체를 비교할 수 있는 2가지 방법을 보여주고 있습니다. 첫번째는 직관적으로 ‘>’ 연산자를 사용하는 것입니다. PartialOrd 트레이트가 구현되었기 때문에 객체간에 ‘>’와 ‘<’ 연산자를 사용하는 것이 가능합니다. 그리고 ‘==’ 연산자는 PartialEq 트레이트가 구현되었기 때문(derive(PartialEq)로 구현했습니다)에 사용할 수 있습니다.

그 다음 마지막에는 partial_cmp 메소드를 직접 호출하는 것을 보여줍니다. '<', '>', '==' 연산자를 사용할 수 없는 경우가 있습니다. 대표적으로 연산의 결과값을 변수에 저장하거나, 다른 함수에 전달해야할 때 ‘<’같은 연산자를 사용할 수 없습니다. 그런 경우에는 이렇게 partial_cmp 메소드를 직접 호출해서 결과값을 저장해야합니다.

이제 두 객체를 비교할 수 있게 되었습니다. 비교하면 생각나는게 정렬 알고리즘이지요. 한반에 있는 학생들을 키에 따라 정렬하는 예제를 만들어보았습니다.

```rust
// src/trait_partialord_sort_first/main.rs
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn class_sorting(people: &mut Vec<Person>) {
    let len = people.len();

    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if people[i] > people[j] {
                people.swap(i, j);
            }
        }
    }
}

fn main() {
    let mut class: Vec<Person> = vec![
        Person {
            name: "aaa".to_owned(),
            age: 10,
            height: 110,
        },
        Person {
            name: "bbb".to_owned(),
            age: 10,
            height: 100,
        },
        Person {
            name: "ccc".to_owned(),
            age: 10,
            height: 120,
        },
        Person {
            name: "ddd".to_owned(),
            age: 10,
            height: 90,
        },
    ];

    class_sorting(&mut class);

    for p in class.iter() {
        println!("{} is {}", p.name, p.height);
    }
}
```

```bash
$ cargo run --bin trait_partialord_sort_first
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/trait_partialord_sort_first`
ddd is 90
bbb is 100
aaa is 110
ccc is 120
```

그리고 대부분 프로그래밍 언어의 라이브러리가 그렇듯이 Rust의 벡터 타입에도 정렬을 위한 메소드가 있습니다. 이때 벡터의 각 데이터들을 비교하는 클로저를 전달해야하므로 이때는 ‘>’같은 산술 연산자가 아니라 partial_cmp 메소드를 호출하는 클로저를 사용하게됩니다.

아래 에제는 백터의 sort_by 메소드와 PartialOrd 트레이트를 사용하는 예제입니다.

```rust
// src/trait_partialord_sort_second/main.rs
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {
    let mut class: Vec<Person> = vec![
        Person {
            name: "aaa".to_owned(),
            age: 10,
            height: 110,
        },
        Person {
            name: "bbb".to_owned(),
            age: 10,
            height: 100,
        },
        Person {
            name: "ccc".to_owned(),
            age: 10,
            height: 120,
        },
        Person {
            name: "ddd".to_owned(),
            age: 10,
            height: 90,
        },
    ];

    class.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for p in class.iter() {
        println!("{} is {}", p.name, p.height);
    }
}
```

```bash
$ cargo run --bin trait_partialord_sort_second
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/trait_partialord_sort_second`
ddd is 90
bbb is 100
aaa is 110
ccc is 120
```

>
> 벡터(Vector)는 다음에 다루겠습니다. 이 예제에서는 partial_cmp 메소드를 직접 호출해야하는 경우에 대해서만 봐주세요.
>

### 타입을 바꿔주는 From/Into 트레이트와 TryFrom/TryInto 트레이트

#### From과 Into

특정 타입을 다른 타입으로 바꿀때 사용하는 트레이트입니다.

이 트레이트가 저처럼 C/C++만 사용하시던 분들께는 필요성이 잘 이해가 안될 수 있습니다. 저도 처음에는 이게 굳이 왜 필요한가 생각했었습니다. 그런데 한번 사용해보니 왜 필요한지 이해가되서 점점 더 자주 사용하게되었습니다. 제가 C/C++로 프로그래밍하는 경우, 한 모듈에서 다른 모듈로 데이터를 전달할때 구조체의 각 필드를 쪼개서 전달하는게 보통이었습니다. 그리고 다른 모듈은 여러개의 데이터를 하나의 새로운 데이터 구조로 만들어서 사용했습니다. 이 과정에서 데이터 순서가 바뀌거나, 허용되지 않는 값을 전달하는 등의 문제가 생기면 쉽게 해결하기 어려웠습니다.

그런데 From/Into같은 타입을 변환하는 트레이트가 있으면 모듈간 데이터를 전달할 때 표준 인터페이스를 만들 수 있으므로 데이터 전달 과정에서 오류를 방지할 수 있게됩니다.

다음은 Book타입의 객체를 u32타입의 ISBN 숫자로 바꾸는 예제입니다.

```rust
// src/trait_from/main.rs
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

impl From<Book> for u32 {
    fn from(book: Book) -> u32 {
        book.isbn.parse().unwrap_or(0)
    }
}

fn main() {
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
        isbn: String::from("718503105"),
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
        isbn: String::from("1617294551"),
    };

    let isbn: u32 = the_book.into();
    let isbn2 = u32::from(rust_in_action);
    println!("The book is {isbn} and Rust in Action is {isbn2}");
}
```

```bash
$ cargo run --bin trait_from
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/trait_from`
The book is 718503105 and Rust in Action is 1617294551
```

Book타입을 u32로 변경하기 위한 것이므로 최종 생성되는 데이터는 u32타입이 됩니다. 그래서 u32타입을 위해 From&lt;Book&gt; 트레이트를 구현하는 것입니다. from이라는 메소드를 구현하게되는데 인자를 Book 타입을 받고, 반환값이 u32가 됩니다.

보통 From을 더 많이 사용하는데, From을 구현하면 Into가 자동으로 구현되기 때문입니다. 예제를 봐도 into 메소드를 구현하지 않았는데 into 메소드를 사용할 수 있는 것을 볼 수 있습니다.

>
> From&lt;Book&gt;에서 From은 트레이트의 이름이고, Book은 From 트레이트가 구현될 타입입니다. 이것은 제너릭 프로그래밍 기법입니다. 제너릭 프로그래밍은 다음에 다시 이야기하겠습니다. 이 예제에서는 From 트레이트를 Book 타입을 위해 구현했다고 이해해주시기 바랍니다.
>

#### TryFrom과 TryInto

좀 더 간단한 From/Into를 알아봤으니, 이번에는 조금 더 유연하게 사용할 수 있는 TryFrom/TryInto의 예제를 보겠습니다. 이름에 Try가 들어가는 것을 보면 알 수 있듯이, 시도를 해보고 안되면 실패를 반환할 수 있는 것이 From/Into와 차이점입니다. 이전 예제에서 From/Into의 반환값을 보면 Result타입이 아니라 곧바로 u32를 반환하는 것을 볼 수 있습니다. 실패에 대한 고려가 없습니다.

이번 예제에서는 TryFrom 트레이트를 구현하고, try_from 메소드에서 Result를 반환하도록 만들어보겠습니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

impl TryFrom<Book> for u32 {
    type Error = u32;

    fn try_from(book: Book) -> Result<u32, Self::Error> {
        match book.isbn.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(0),
        }
    }
}

fn main() {
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
        isbn: String::from("718503105"),
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
        isbn: String::from("1617294551"),
    };

    let isbn: Result<u32, u32> = the_book.try_into();
    let isbn2 = u32::try_from(rust_in_action);
    println!("The book is {:?} and Rust in Action is {:?}", isbn, isbn2);
}
```

TryFrom 트레이트를 구현해봤습니다. 트레이트 구현을 보면 가장 처음 한 일이 Error 타입으로 어느 타입을 지정할지 결정하는 일입니다. 책의 ISBN은 0이 될 수 없으니 에러 타입을 u32 타입 정수로 지정하고 에러의 경우 Err(0)을 반환하도록 만들어봤습니다. 보다 친절하게 에러 메세지를 지정하고 싶으면 Error타입을 &’static str 타입이나 String 타입으로 지정할 수도 있겠습니다. 그리고 try_from 메소드를 구현하는데 반환값이 Result<u32, Self::Error>입니다. from메소드에서 u32를 반환하는 것과 차이가 있습니다. 메소드의 구현은 간단합니다. 책 데이터에 있는 isbn값이 정상적인 값이라면 Ok(u32)가 반환될 것이고, 정상적인 값이 아니라서 파싱중에 에러가 발생하면 Err(0)이 반환될 것입니다.

상황에 따라 From을 쓸 것인지, TryFrom을 쓸 것인지 선택하면되겠지만, 보다 유연한 에러처리를 위해서는 TryFrom을 쓰는게 좋을 것입니다.

### Iterators 반복자

지금까지 이터레이터를 여러번 사용해봤는데 사실 이터레이트는 트레이트로 구현됩니다. 러스트의 기본 데이터 타입에는 이터레이터가 미리 구현되어있어서 우리가 직접 구현할 필요가 없었습니다. 하지만 우리가 직접 만든 타입에 이터레이터를 사용하기 위해서는 우리가 직접 구현해야겠지요.

이번 장에서는 내가 만든 구조체 타입에 이터레이터를 사용할 수 있도록 만들어보겠습니다.  

아래 소스는 이전에 fmt::Display 트레이트를 구현해본 예제입니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    println!("{}", book);
}
```

이 예제에서 우리가 만든 Book 타입에 이터레이트를 적용해보겠습니다.

우선 이터레이터가 어떤 트레이트인지 러스트 메뉴얼을 찾아보겠습니다.

```rust
pub trait Iterator {
    type Item;

    // Required method
    fn next(&mut self) -> Option<Self::Item>;

    // Provided methods
....skip...
}
```

<https://doc.rust-lang.org/std/iter/trait.Iterator.html>

메뉴얼에 따르면 필수로 구현해야할 next라는 메소드가 1개있고, next메소드를 구현하면 자동으로 제공되는 메소드가 74개있다고 합니다. 그렇게 총 75개의 메소드를 가지는 트레이트입니다. next메소드는 자기 자신을 mutable 참조로 받아서 Option에 감싼 결과 값 하나를 반환하게 됩니다. 우리가 해야할 일은 Item이라는 타입에 무엇을 쓸지 정해야합니다. 보통은 자신이 구현한 구조체나 구조체의 참조가 되겠지요. 그리고 주의해야할 것이 이터레이터가 현재 몇개의 값을 반환했는지, 그리고 더 이상 반환할 값이 있는지 없는지 등의 상태 관리를 해야된다는 것입니다. 그렇게 이터레이터가 가진 상태가 변하기때문에 &mut self라는 타입을 사용하는 것입니다.

그럼 Book 타입의 객체들을 처리하기위한 BookSelf라는 이터레이터 타입을 구현해보겠습니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

#[derive(Debug)]
struct BookShelf<'a> {
    books: &'a [Book],
}

impl<'a> Iterator for BookShelf<'a> {
    type Item = &'a Book;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, remains)) = self.books.split_first() {
            self.books = remains;
            Some(first)
        } else {
            None
        }
    }
}

fn main() {
    let mut book_array = [
        Book {
            title: String::from("The Fellowship of the Ring"),
            author: String::from("J. R. R. Tolkien"),
            published: 19540729,
        },
        Book {
            title: String::from("The Two Towers"),
            author: String::from("J. R. R. Tolkien"),
            published: 19541111,
        },
        Book {
            title: String::from("The Return of the King"),
            author: String::from("J. R. R. Tolkien"),
            published: 19551020,
        },
    ];

    let mut mybooks_iter = BookShelf {
        books: &mut book_array,
    };
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());

    let mybooks_for = BookShelf {
        books: &mut book_array,
    };
    for b in mybooks_for {
        println!("{:?}", b);
    }
}
```

```bash
$ cargo run --bin trait_iterator
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/trait_iterator`
Some(Book { title: "The Fellowship of the Ring", author: "J. R. R. Tolkien", published: 19540729 })
Some(Book { title: "The Two Towers", author: "J. R. R. Tolkien", published: 19541111 })
Some(Book { title: "The Return of the King", author: "J. R. R. Tolkien", published: 19551020 })
None
Book { title: "The Fellowship of the Ring", author: "J. R. R. Tolkien", published: 19540729 }
Book { title: "The Two Towers", author: "J. R. R. Tolkien", published: 19541111 }
Book { title: "The Return of the King", author: "J. R. R. Tolkien", published: 19551020 }
```

일단 구현이 어떻게할지는 놔두고 main함수에서 어떻게 사용하는지부터 보겠습니다.

book_array라는 이름의 배열을 만들어서 여러개의 Book 객체를 저장합니다. 그리고 mybooks_iter라는 변수를 만드는데 이 변수는 BookSelf타입이고 books라는 필드에 book_array의 참조 포인터를 저장합니다. mybooks_iter라는 변수가 이터레이터가 되는 것입니다. 이제부터 mybooks_iter의 next()라는 메소드를 호출할 때마다 배열에 저장된 Book 타입 객체들이 하나씩 반환되는 것입니다.

또한 이터레이터를 for 루프에도 사용할 수 있습니다. mybooks_for라는 인터레이터를 만들어서 for 루프에 사용하면 next() 메소드를 자동으로 호출해줍니다.

그럼 BookSelf라는 구조체는 어떻게 구현되었는지 볼까요. Book타입 배열의 참조 포인터를 가지고 있습니다. 그냥 배열의 시작 주소를 가지고 있는것 뿐입니다. 이것이 어떻게 이터레이터 역할을 할 수 있는지를 Iterator 트레이트 구현을 확인해야 알 수 있습니다.

Iterator 구현을 보면 가장 먼저 볼 수 있는게 Item 타입은 Book객체의 참조 포인터라는 것입니다. 이것은 이터레이터가 next() 메소드를 호출하거나 for 루프에서 사용될때 반환하는 값의 타입이 Book 객체의 참조 포인터라는 것입니다. 그 다음을 보면 next() 메소드의 구현이 나옵니다. 메소드 인자는 &mut self로 우리가 만든 BookSelf 구조체 mutable 참조가 됩니다. 그리고 반환값인 Option<&Book>이 되겠지요. 구현은 생각보다 간단합니다. 우선 현재 self.books라는 배열을 첫번째 객체와 나머지로 쪼갭니다. 만약 잘 쪼개진다면 어쨌든 배열안에 데이터(Book객체에 대한 포인터)가 1개 이상 들어있다는 뜻이므로 첫번째 객체는 반환하고 나머지는 self.books에 저장합니다. 그런데 배열을 쪼개려고했는데 None이 반환되었다는 것은 self.books 배열에 아무런 데이터가 없다는 뜻이므로 None을 반환해주면 됩니다.

아마 다른 언어로 이터레이터를 만들어본 경험이 있으시다면 쉽게 익숙해질 수 있을거라 생각합니다. 단지 주의해야할 것은 반환값은 Option이므로 더이상 데이터가 없을때 None을 호출해야한다는 것입니다.

>
> 예제에 impl<'a>와 Bookshelf<'a>같이 새로운 형태의 문법이 사용되었습니다. 이것들을 라이프타임(Lifetime)이라고 합니다. 다음에 제대로 소개하겠습니다. 일단 이 예제와같이 아주 간단하게 라이프타임을 사용할 경우에는, 일단 라이프타임을 지정하지않고 코딩한 후에 컴파일러의 에러 메세지를 참고해서 라이프타임 지정을 추가해주면 됩니다.
>

### 벡터의 각 요소를 수정할 수 있는 반복자

지금까지는 배열이나 벡터의 각 데이터를 읽기만 하는 반복자를 사용했었습니다. 하지만 보통은 아래와 같이 데이터를 수정해야되는 상황이 더 많을 것입니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book_array = [
        Book {
            title: String::from("The Fellowship of the Ring"),
            author: String::from("J. R. R. Tolkien"),
            published: 19540729,
        },
        Book {
            title: String::from("The Two Towers"),
            author: String::from("J. R. R. Tolkien"),
            published: 19541111,
        },
        Book {
            title: String::from("The Return of the King"),
            author: String::from("J. R. R. Tolkien"),
            published: 19551020,
        },
    ];

    for b in book_array.iter() {
        b.author = String::from("John Ronald Reuel Tolkien");
    }

    println!("{:?}", book_array);
}
```

이 예제는 모든 책의 author 필드를 "John Ronald Reuel Tolkien"으로 바꾸려는 예제입니다. 이 예제를 빌드해보면 아래와 같은 에러가 발생합니다.

```bash
$ cargo build --bin trait_iterator_iter_mut
   Compiling pyalgo v0.1.0 (/home/gurugio/pyalgo)
error[E0594]: cannot assign to `b.author`, which is behind a `&` reference
  --> src/main.rs:28:9
   |
27 |     for b in book_array.iter() {
   |              -----------------
   |              |          |
   |              |          help: use mutable method: `iter_mut()`
   |              this iterator yields `&` references
28 |         b.author = String::from("John Ronald Reuel Tolkien");
   |         ^^^^^^^^ `b` is a `&` reference, so the data it refers to cannot be written

For more information about this error, try `rustc --explain E0594`.
error: could not compile `pyalgo` (bin "pyalgo") due to 1 previous error
```

book_array.iter()는 immutable reference를 반환합니다. 따라서 book_array.iter()에서 반환된 불변 참조가 b에 저장되고, b를 통해 데이터에 접근하면 데이터를 수정할 수 없습니다. 컴파일러가 친절하게 iter() 대신에 iter_mut() 메소드를 사용하라고 알려주고 있습니다. 그리고 한가지 더 잊지 말아야하는게있는데 book_array 선언에 mut를 추가하는 것입니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut book_array = [
        Book {
            title: String::from("The Fellowship of the Ring"),
            author: String::from("J. R. R. Tolkien"),
            published: 19540729,
        },
        Book {
            title: String::from("The Two Towers"),
            author: String::from("J. R. R. Tolkien"),
            published: 19541111,
        },
        Book {
            title: String::from("The Return of the King"),
            author: String::from("J. R. R. Tolkien"),
            published: 19551020,
        },
    ];

    for b in book_array.iter_mut() {
        b.author = String::from("John Ronald Reuel Tolkien");
    }

    println!("{:?}", book_array);
}
```

```bash
$ cargo run --bin trait_iterator_iter_mut
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
warning: fields `title` and `published` are never read
 --> src/trait_iterator_iter_mut/main.rs:3:5
  |
2 | struct Book {
  |        ---- fields in this struct
3 |     title: String,
  |     ^^^^^
4 |     author: String,
5 |     published: u32,
  |     ^^^^^^^^^
  |
  = note: `Book` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "trait_iterator_iter_mut") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/trait_iterator_iter_mut`
[Book { title: "The Fellowship of the Ring", author: "John Ronald Reuel Tolkien", published: 19540729 }, Book { title: "The Two Towers", author: "John Ronald Reuel Tolkien", published: 19541111 }, Book { title: "The Return of the King", author: "John Ronald Reuel Tolkien", published: 19551020 }]
```

배열이나 벡터를 사용할 때 반복자를 직접 구현하는 경우는 많지 않을 것입니다. 보통은 네트워크에서 데이터를 받거나 보낼때, 스트림에서 데이터를 읽어올 때 같이 모든 데이터를 한꺼번에 처리하지않고 조금씩 처리하는 경우에 반복자를 사용합니다.
