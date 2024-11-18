Anyhow

anyhow

clap: command line argument parser

- <https://rust-cli.github.io/book/index.html>

serde: derive(Serialize, Deserialize)

stringfy

async + http server with Tokio

# 쓰레드간 공유 메모리 Rc와 Arc - WIP

# 스마트 포인터 Box - WIP

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

fn main() {
    let book_in_stack = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
        isbn: String::from("1617294551"),
    };
    let boxed_book = Box::new(Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
        isbn: String::from("718503105"),
    });

    println!("stack variable pointer: {:p}", &book_in_stack);
    println!("boxed_book variable pointer: {:p}", &boxed_book);
    println!("boxed_book variable address: {:p}", boxed_book);
}
```

# std::pin - WIP

std::pin은 self-reference가 있는 (예를 들면 linked list) 구조체의 객체가 어떤 상황에서도 메모리 위치가 변하지 않도록 해주는 것입니다.

어떤 상황에 메모리 위치가 변하는지는 다음 예제 코드를 보면 알 수 있습니다.

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

struct MyStruct {
    data: String,
    ptr: NonNull<String>,
}

struct MyStructPinned {
    data: String,
    ptr: NonNull<String>,
    _pin: PhantomPinned, // https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html
}

fn get_moved(moved: MyStruct) {
    println!("func-moved addr: {:p}", &moved);
}

fn get_pinned(moved: Pin<Box<MyStructPinned>>) {
    println!("func-pinned addr: {:p}", &*moved);
}

fn main() {
    // https://os.phil-opp.com/async-await/#pinning
    // "This could (memory copy and move) happen when we pass the struct as a function argument
    // or assign it to a different stack variable.""

    // move variable from stack to stack ==> memory copy occurs
    let mut first = MyStruct {
        data: "dummy".to_string(),
        ptr: NonNull::dangling(),
    };
    let slice = NonNull::from(&first.data);
    first.ptr = slice;

    println!("moved addr: {:p} {:p}", &first, first.ptr);
    let second = first;
    println!("moved addr: {:p} {:p}", &second, second.ptr); // second.ptr == first.ptr
    get_moved(second);

    // move pinned variable ==> NO memory copy
    // The address of the variable is not heap area.
    // I guess Pin allocates variable in the heap memory to fix memory address.
    let first = MyStructPinned {
        data: "dummy".to_string(),
        ptr: NonNull::dangling(),
        _pin: PhantomPinned,
    };
    let mut first_pin = Box::pin(first); // pin the local variable
    let slice = NonNull::from(&first_pin.data);
    unsafe {
        let mut_ref: Pin<&mut MyStructPinned> = Pin::as_mut(&mut first_pin);
        Pin::get_unchecked_mut(mut_ref).ptr = slice;
    }

    println!("pinned addr: {:p} {:p}", &*first_pin, first_pin.ptr);
    let second_pin = first_pin;
    println!("pinned addr: {:p} {:p}", &*second_pin, second_pin.ptr); // same address
    get_pinned(second_pin); // same address
}

/* Result
moved addr: 0x7ffe3b6c0670 0x7ffe3b6c0670
moved addr: 0x7ffe3b6c0700 0x7ffe3b6c0670
func-moved addr: 0x7ffe3b6c0780
pinned addr: 0x561855b5bae0 0x561855b5bae0
pinned addr: 0x561855b5bae0 0x561855b5bae0
func-pinned addr: 0x561855b5bae0
*/
```

첫번째 first변수는 스택에 생성된 로컬 변수입니다. 이 객체를 다른 로컬 변수에 할당하면 복사가 일어나서 second 객체에 first객체가 가진 값들이 복사됩니다. 만약  MyStruct의 멤버 변수중에 bar 변수에 대한 포인터 값이 있었다면, second에 복사된 포인터는 second.bar가 아닌 first.bar를 가르키게되므로 깨진 포인터가 됩니다.
