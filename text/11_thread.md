# Thread

## Thread를 사용하기 위해 미리 알아두어야할 것들

먼저 Rust에서 제공하는 동기화 기법들을 소개하고, 그 다음 쓰레드를 만들어서 사용해보도록 하겠습니다.

### 여러 쓰레드가 공유해서 사용할 수 있는 스마트 포인터 Arc

Arc는 Atomic Reference Counter의 약자입니다.
여러 쓰레드가 하나의 데이터를 공유할 때 몇번 공유가 되고 있나를 관리합니다.
사실 이미 C/C++ 계열에서 직접 구현해서 사용하는 경우도 많고, 자바등 가비지 콜렉터가 있는 언어에서는 가비지 콜렉터 내부에서 메모리를 관리할 때 사용하는 기법이라 어떤 개념인지는 많이 알고 있을 것입니다.

러스트에서 Arc가 아주 중요하고 쓰레드를 사용할 때 필수적인 이유는 러스트의 소유권을 관리하기 위해서입니다.
C/C++ 언어에서는 사실 뮤텍스같은 락만 있으면 여러 쓰레드에서 데이터를 공유할 수 있습니다.
하지만 러스트는 소유권을 관리해야하기때문에, 다르게 말하면 언제 이 데이터가 해지되어야하는지를 추적해야되기 때문에 별도의 레퍼런스 카운터가 필요합니다.

사용법은 간단합니다.
다음과 같이 Arc의 new메소드를 이용해서 공유하고자하는 데이터를 위한 Arc 객체를 만듭니다.
그리고 clone를 호출하면 레퍼런스 카운터가 증가하면서 각 쓰레드가 데이터에 접근할 수 있는 포인터가 생성됩니다. 
```rust
fn main() {
    let myarc: std::sync::Arc<Vec<i32>> = std::sync::Arc::new(vec![1, 2, 3, 4, 5]);
    let myarc_ref: std::sync::Arc<Vec<i32>> = myarc.clone();
    println!("{:?}", myarc_ref);
}
```
```bash
[1, 2, 3, 4, 5]
```

주의해야할 것은 Arc로 공유하는 데이터는 변경할 수 없다는 것입니다.
러스트의 메모리 관리에서 가장 중요한 규칙은 가변 레퍼런스는 하나만 존재해야한다는 것입니다.
여러 쓰레드가 Arc로 공유되는 데이터를 변경하려한다면, 가변 레퍼런스가 여러개가 존재한다는 것이므로, 러스트의 메모리 관리 규칙을 어기게 됩니다.

Arc의 메뉴얼을 보시면 가변 레퍼런스를 반환하는 get_mut 메소드가 있긴 합니다.
하지만 get_mut 메소드의 설명을 보면 같은 데이터에 대한 다른 Arc 공유가 없을 때만 가변 레퍼런스를 얻을 수 있다고 써있습니다.
위의 예제에서 myarc만 만들었을 때는 데이터가 공유되지 않은 상태이므로 가변 레퍼런스를 얻을 수 있지만, myarc_ref가 생긴 후에는 가변 레퍼런스를 얻을 수 없습니다.
당연히 여러 쓰레드가 하나의 데이터를 수정하려면 단 하나만의 가변 레퍼런스가 존재하도록 보장해주는 락을 사용해야합니다.

### 기본 자료형을 공유할 수 있는 Atomic 타입

usize, i32등 기본 자료형을 공유하는데 사용하는 아토믹 타입은 AtomicUSize, AtomicI32 등이 있습니다.
다음과 같은 단계로 아토믹 변수의 공유 객체를 생성해서 사용할 수 있습니다.
1. AtomicUsize::new 등 new 메소드를 이용해서 아토믹 변수를 만듭니다.
2. 쓰레드간 공유를 위해서 Arc::new 메소드를 이용해서 아토믹 변수를 공유하기 위한 Arc 객체를 만듭니다.
3. Arc의 clone 메소드를 사용해서 각 쓰레드에 Arc 객체를 전달합니다.
4. Arc타입 변수에 아토믹 타입의 메소드 store, load등을 그대로 사용합니다.

```rust
fn main() {
    let atomic_usize = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(1));
    let arc_usize = std::sync::Arc::clone(&atomic_usize);

    arc_usize.store(0, std::sync::atomic::Ordering::Relaxed);
    assert_eq!(arc_usize.load(std::sync::atomic::Ordering::Relaxed), 0);
}
```

주의해야할 것은 store, load 등의 메소드에 메모리 오더링을 위한 std::sync::atomic::Ordering 타입을 전달한다는 것입니다.
일반적으로 하나의 아토믹 변수를 읽고 쓰는 상황에서는 std::sync::atomic::Ordering::Relaxed를 지정해서 사용할 수 있습니다.

#### Memory Ordering에 대한 짧은 소개

메모리 오더링은 컴파일러가 아토믹 변수에 접근하는 명령어를 어떻게 배치하는가를 지정하는 것입니다.
```rust
fn main() {
    let mut a;
    let mut b;
    let mut c;
    a = 1;
    b = 2;
    c = 3;
    println!("{}", a + b + c);
}
```
우리는 위와 같이 a, b, c 순서로 메모리에 값을 쓰려고 합니다.
하지만 컴파일러는 컴파일을 하면서 굳이 a, b, c 순서대로 값을 쓸 필요가 없다는 것을 압니다.
c, b, a 순서로 값을 써도 프로그램은 동일하게 동작을 하기 때문입니다.
사실 위와 같이 아주 단순한 경우는 없지만, 여러가지 변수를 사용하고, 변수들이 다른 데이터를 참조하는 레퍼런스일 때, 컴파일러는 더 높은 성능을 위해서 메모리 접근 순서를 바꿉니다.

이런 메모리 접근 순서를 완전히 컴파일러에게 맞길때 사용하는 것이 std::sync::atomic::Ordering::Relaxed 입니다.

```rust
fn main() {
    let atomic_usize = std::sync::atomic::AtomicUsize::new(1);
    atomic_usize.store(0, std::sync::atomic::Ordering::Relaxed);
    atomic_usize.store(1, std::sync::atomic::Ordering::Relaxed);
    atomic_usize.store(2, std::sync::atomic::Ordering::Relaxed);
    println!(
        "{}",
        atomic_usize.load(std::sync::atomic::Ordering::Relaxed)
    );
}
```
위와같이 하나의 아토믹 변수에 0, 1, 2를 쓰고 마지막에 값이 2인 것을 확인하는 프로그램이 있다면, 컴파일러는 1, 0, 2 순서로 값을 쓸 수도 있습니다.
어떤 경우에 순서가 바뀌는지는 컴파일러에게 맡겼으므로 우리는 알 수 없습니다.

이렇게 하나의 변수만을 사용할 때는 문제가 없습니다.
컴파일러가 변수를 사용하기 직전에 어떤 상태가 되어야하는지를 잘 판단할 수 있기 때문입니다.
하지만 2개 이상의 쓰레드에서 여러 아토믹 변수의 값을 읽고 쓸 때는 메모리의 접근 순서가 중요해집니다.
가장 흔하게 볼 수 있는 예제가 다음과 같이 어떤 플래그의 값에 따라서 동작하는 경우입니다.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let mut handles = vec![];
    let atomic_flag = Arc::new(AtomicUsize::new(0));
    let atomic_data = Arc::new(AtomicUsize::new(0));

    let thr1_flag = atomic_flag.clone();
    let thr2_flag = atomic_flag.clone();
    let thr1_data = atomic_data.clone();
    let thr2_data = atomic_data.clone();

    // Thread-1
    let handle_thr1 = thread::spawn(move || {
        thr1_data.store(1234, Ordering::Relaxed);
        thr1_flag.store(1, Ordering::Relaxed);
    });
    handles.push(handle_thr1);

    // Thread-2
    let handle_thr2 = thread::spawn(move || {
        loop {
            if thr2_flag.load(Ordering::Relaxed) == 1 {
                println!(
                    "Do something with data {}",
                    thr2_data.load(Ordering::Relaxed)
                );
                break;
            } else {
                // wait
            }
        }
    });
    handles.push(handle_thr2);

    for h in handles {
        let _ = h.join();
    }
}
```

아직 쓰레드를 생성하는 것은 
1번 쓰레드를 생산자, 2번 쓰레드를 소비자 쓰레드라고 생각하면 편합니다.
우리가 기대하는 것은 다음과 같은 순서생산자가 데이터를 생산한 후 소비자에게 알려주고, 소비자는 생산자의 데이터를 가지고 뭔가를 실행하는 것입니다.
1. Thread-1: data = 1234
2. Thread-1: flag = 1
3. Thread-2: if flag == 1
4. Thread-2: use data

하지만 Ordering::Relaxed는 컴파일러에게 메모리 접근 순서를 바꿀 수 있도록 허가해주는 것이므로, 컴파일러는 상황에 따라 다음과 같이 메모리 접근 순서를 바꿀 수 있습니다.
1. Thread-1: flag = 1
2. Thread-2: if flag == 1
3. Thread-2: use data
4. Thread-1: data = 1234

1234라는 값으로 데이터 처리를 실행해야되지만, 0이라는 값으로 데이터 처리가 실행될 수도 있습니다.
제가 여기에서 실행될 수도 있다고 말하는 것은 어떻게 될지 우리는 알 수 없기 때문입니다.
컴파일러가 어떻게 컴파일하냐에 따라 달라집니다.
만약에 아주 드물에 처리 순서가 섞여서 잘못된 데이터 처리가 발생한다면 얼마나 디버깅하기 어려울지 상상해볼 수 있습니다.

그래서 Ordering에 Release와 Acquire 라는 타입이 있습니다.
Release는 Release타입으로 실행되는 메모리 쓰기 위치보다 먼저 실행되는 메모리 쓰기 명령들이 절대로 Release 타입 메모리 쓰기 다음에 실행될 수 없도록 합니다.

```rust
fn main() {
    let atomic_usize = std::sync::atomic::AtomicUsize::new(1);
    atomic_usize.store(0, std::sync::atomic::Ordering::Relaxed);
    atomic_usize.store(1, std::sync::atomic::Ordering::Release);
    atomic_usize.store(2, std::sync::atomic::Ordering::Relaxed);
    println!(
        "{}",
        atomic_usize.load(std::sync::atomic::Ordering::Relaxed)
    );
}
```
이렇게 여러개의 메모리 쓰기 연산중에 Release타입 연산을 둔다면 컴파일러는 언제나 0, 1, 2 순서로만 메모리를 쓰게됩니다.

Acquire는 Release와 마찬가지로 메모리 접근 순서를 지키는 것인데, 차이가 있다면 메모리 읽기 연산에 대한 순서를 지키도록 하는 것입니다.

```
fn main() {
    let a = std::sync::atomic::AtomicUsize::new(1);
    let b = std::sync::atomic::AtomicUsize::new(1);
    let c = std::sync::atomic::AtomicUsize::new(1);
    a.load(std::sync::atomic::Ordering::Relaxed);
    b.load(std::sync::atomic::Ordering::Acquire);
    c.load(std::sync::atomic::Ordering::Relaxed);
}
```

이렇게 3개의 아토믹 변수를 모두 Relaxed타입으로 실행한다면 a, b, c 순서가 아니라 c, b, a등 어떤 순서로도 실행될 수 있습니다.
하지만 예제와 같이 b를 읽을 때 Acquire타입으로 실행하면, 메모리 읽기 순서는 언제나 a, b, c가 됩니다.

그래서 2개의 쓰레드에서 항상 플래그 값을 옳바르게 전달하기 위해서 다음과 같이 플래그 값을 쓸 때는 Release 타입으로, 플래그 값을 읽을 때는 Acquire타입으로 실행해야됩니다.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut handles = vec![];
    let atomic_flag = Arc::new(AtomicUsize::new(0));
    let atomic_data = Arc::new(AtomicUsize::new(0));

    let thr1_flag = atomic_flag.clone();
    let thr2_flag = atomic_flag.clone();
    let thr1_data = atomic_data.clone();
    let thr2_data = atomic_data.clone();

    // Thread-1
    let handle_thr1 = thread::spawn(move || {
        thr1_data.store(1234, Ordering::Relaxed);
        thr1_flag.store(1, Ordering::Release);
    });
    handles.push(handle_thr1);

    // Thread-2
    let handle_thr2 = thread::spawn(move || {
        loop {
            if thr2_flag.load(Ordering::Acquire) == 1 {
                println!(
                    "Do something with data {}",
                    thr2_data.load(Ordering::Relaxed)
                );
                break;
            } else {
                // wait
            }
        }
    });
    handles.push(handle_thr2);

    for h in handles {
        let _ = h.join();
    }
}
```

두 쓰레드의 메모리 접근 순서를 다시 생각해보면 다음과 같습니다.
1. Thread-1: data = 1234 (Relaxed)
2. Thread-1: flag = 1 (Release)
3. Thread-2: if flag == 1 (Acquire)
4. Thread-2: use data (Relaxed)

쓰레드1에는 항상 데이터에 값을 쓰고, 그 다음에 플래그에 1을 쓰게됩니다.
절대로 플래그 다음에 데이터 값을 쓸 수 없습니다.
그리고 쓰레드 2에서는 항상 플래그 값을 읽고 그 다음에 데이터 값을 읽게 됩니다.
따라서 우리가 생각한데로 항상 1, 2, 3, 4 순서로 실행됩니다.

참고로 std::sync::atomic::Ordering은 enum 타입으로 5가지의 메모리 오더링을 지정하고 있습니다. 
```
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

AcqRel은 Acquire와 Release를 합친 것입니다. 메모리 읽기 쓰기의 순서를 모두 지켜주는 것입니다.
SeqCst는 메모리 접근에 대한 최적화를 모두 없애는 것입니다.
1. a = 1, Relaxed
2. b = 2, Relaxed
3. c = 3, AcqRel
4. d = 4, Relaxed

이런 코드가 있다면 1, 2, 3, 4 순서로 실행되거나 2, 1, 3, 4 순서로 실행될 수 있습니다.
하지만 SeqCst를 사용해서 이런 순서 변화를 모두 없앨 수 있습니다.
1. a = 1, SeqCst
2. b = 2, SeqCst
3. c = 3, SeqCst
4. d = 4, SeqCst

이제는 언제나 1, 2, 3, 4 순서로만 실행됩니다.

정리하자면 Relaxed는 컴파일러에게 최대한의 최적화를 할 수 있도록 자유를 주는 것입니다.
하지만 여러 쓰레드가 메모리의 접근 순서에 따라 동작이 달라질 수 있을 때는 적당한 메모리 연산 순서를 지정해줄 필요가 있습니다.
만약 잘 판단이 되지 않는다면 모든 아토믹 연산을 SeqCst로 실행한 후에, 조금씩 바꿔보는 것도 방법일 수 있습니다.

### Mutex


### Channel



## Thread를 사용하기 위해 필요한 트레이트

Send
Sync
Copy는 안되는 이유

## Thread 생성 방법

Mutex, Arc를 이용한 쓰레드 생성 예제

### Thread의 결과값을 받는 방법

Result<쓰레드함수의반환값, Box<dyn Any + Send>>
Trait Any에 대한 설명

