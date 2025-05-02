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
제품을 출시하기 위한 마지막 단계에서 반드시 고려하게될 내용이므로 꼭 기본 개념은 이해할 필요가 있습니다.
러스트의 기본 개념들을 설명하는 책이므로 짧게만 소개하겠습니다만 반드시 여러 자료들을 찾아보고 상세하게 이해하시길 추천합니다.
특히 C++의 Memory Ordering에 대한 자료나 리눅스 커널의 Memory Barrier에 대한 자료 등을 찾아보시길 추천합니다.

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

뮤텍스는 말 그대로 하나의 쓰레드만 공유 데이터에 접근하도록 해주는 락입니다.
그런데 사용법이 여러 프로그래밍 언어들과 차이가 있습니다.
대부분의 프로그래밍 언어들은 공유할 데이터를 만들고, 해당 데이터를 위한 뮤텍스를 따로 만든 후, 뮤텍스를 잠그고 데이터에 접근합니다.
하지만 러스트의 뮤텍스는 공유할 데이터와 따로 존재하는게 아닙니다.
뮤텍스를 만들 때 뮤텍스 내부에 데이터를 저장합니다.
이런 방식의 장점은 락을 걸지 않고 데이터에 접근하는 것을 원천적으로 방지할 수 있다는 것입니다.
아래 예제를 보면 뮤텍스를 만드는 new 메소드에 데이터의 소유권을 넘기는 것을 볼 수 있습니다.

```rust
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    // let data_lock_share = Arc::new(Mutex::new(0));
    let data: usize = 0;
    let data_lock = Mutex::new(data);
    let data_lock_share = Arc::new(data_lock);

    println!("Original data is {}", data);

    let data_thr1 = data_lock_share.clone();
    {
        // Thread-1
        let mut data: MutexGuard<usize> = data_thr1.lock().unwrap();
        *data = 1;
        println!("Thread-1: data is {}", *data);
        // The lock is unlocked automatically
    }

    let data_thr2 = data_lock_share.clone();
    {
        // Thread-2
        let mut data: MutexGuard<usize> = data_thr2.lock().unwrap();
        *data = 2;
        println!("Thread-2: data is {}", data);
        // The lock is unlocked automatically
    }
}
```

락을 만든 후에는 락을 여러 쓰레드가 공유할 수 있도록 Arc의 new 메소드에 넘겨줍니다.
그래서 최종적으로 Arc 객체가 생성되고, Arc 객체 안에 뮤텍스가 있고, 그 뮤텍스 안에 공유할 데이터가 존재합니다.

락을 사용할 때는 lock 매소드를 호출하면 락을 잡게됩니다.
물론 try_lock도 있습니다.
그런데 꼭 기억해야할 것은 unlock 메소드가 없다는 것입니다.
뮤텍스의 lock 메소드가 반환하는 값의 타입은 MutexGuard라는 또 다른 타입입니다.
이 MutexGuard 타입은 Deref, DerefMut 트레이트를 구현하고 있어서, 스마트 포인터와 같이 이 타입이 감싸고 있는 데이터에 직접 접근할 수 있습니다.
그리고 이 MutexGuard라는 타입의 객체가 사라지는 시점에서 락이 해지되는 것입니다.
위의 예제를 보면 Thread-1으로 가정한 블럭이 있습니다.
이 블럭 안에서 MutexGuard가 생성되었으므로, 이 블럭이 끝나는 지점에서 MutexGuard가 자동으로 해지되고 동시에 락도 풀어집니다.

이렇게 락도 아니고 공유 데이터도 아닌 새로운 타입을 만들어서 자동으로 락이 해지되도록하는 기법을 Resource Acquisition Is Initialization (RAII) 패턴이라고 부릅니다.
MutexGuard라는 객체가 생성되고 소멸되는 시점에 락이 잠기고 풀어지도록해서 락을 불필요하게 오래 잡고있거나, 락을 해지하지 않는 버그를 미리 방지하는 것입니다.

러스트에서 락을 사용하는 것만큼 중요한게 락을 사용하던 쓰레드가 락을 잠근 채로 죽어서 다른 쓰레드가 락을 사용하지 못하도록 만드는 Lock Poisoning 문제입니다.
다음 예제 처럼 Thread-1에서 락을 잡고 사용하다가 어떤 오류로 인해 쓰레드가 죽었다고 생각해보겠습니다.

```rust
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let data: usize = 0;
    let data_lock = Mutex::new(data);
    let data_lock_share = Arc::new(data_lock);

    println!("Original data is {}", data);

    let data_thr1 = data_lock_share.clone();
    {
        // Thread-1
        let mut data: MutexGuard<usize> = data_thr1.lock().unwrap();
        panic!();
        // The lock is not unlocked
    }

    let data_thr2 = data_lock_share.clone();
    {
        // Thread-2
        let mut data = match data_thr2.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        *data = 2;
        println!("Thread-2: data is {}", data);
        // The lock is unlocked automatically
    }
}
```

이전 예제에서는 Thread-2에서 lock메소드의 반환값을 따로 확인하지 않았습니다만 이번 예제에서는 lock 메소드의 반환값이 정상인지 에러인지를 확인합니다.
정상이라면 data에는 MutexGuard타입의 객체가 저장될 것입니다.
하지만 에러가 발생했다면 어디에선가 락을 해지하지 않는 에러가 발생한 것이므로 에러 값을 받고, 그 에러값을 into_inner 메소드로 MutexGuard 타입으로 바꿔서 계속 데이터를 사용하게 됩니다.
그러면 락을 풀지 않은 에러가 발생했더라도, 다른 쓰레드에서 락을 계속 사용하고 락을 풀어줄 수 있으므로 더 이상 에러가 전파되는 일이 없습니다.

참고로 위 예제에서 poisoned 변수의 타입은 PoisonError 타입이고, 이 타입에서 into_inner 메소드를 구현한 것입니다.
into_inner 메소드는 에러 값을 소모(consume)시키고 원래 공유 데이터를 반환하는 일을 합니다.

제품에 들어가는 코드를 만들 때는 반드시 이런 Lock Poisioning 에러에 대한 방지 코드를 작성하도록 주의하시기 바랍니다.

### Channel

채널은 쓰레드간에 데이터를 주고 받는데 아주 편리한 툴입니다.
기본적으로 여러 쓰레드가 데이터를 보내고, 하나의 쓰레드가 데이터를 받는 Multi Producer Single Consumer (MPSC) 방식으로 동작합니다.
물론 반대로 Single Producer Multi Consumer 동작을 지원하는 크레이트도 있습니다.
우리는 표준 크레이트에 들어있는 mpsc 채널의 사용법을 알아보겠습니다.
SPMC 채널 크레이트의 사용법은 MPSC 채널과 동일하므로 사용법은 완전히 동일하므로 따로 설명하지 않겠습니다.

사용법은 아주 간단합니다.
채널을 생성하면 전송 객체 (Producer)와 수신 객체 (Consumer)가 생성됩니다.
여러 쓰레드가 전송을 할 것이므로, 각 쓰레드는 전송 객체의 클론을 받아서 사용합니다.
데이터를 받는 쓰레드는 유일하므로 별다른 처리없이 recv 메소드만 호출하면 데이터를 받게됩니다.

```rust
use std::sync::mpsc::{channel, Sender};

fn main() {
    let (sender, receiver) = channel();

    let sender_thr1: Sender<i32> = sender.clone();
    {
        // Thread-1
        sender_thr1.send(1).unwrap();
    }

    let sender_thr2: Sender<i32> = sender.clone();
    {
        // Thread-2
        sender_thr2.send(2).unwrap();
    }

    for _ in 0..2 {
        let t: i32 = receiver.recv().unwrap();
        println!("Main received {}", t);
    }
}
```

아직 쓰레드의 사용법을 알아보지 않았으므로 쓰레드 생성없이 데이터를 주고 받는 예제를 만들어봤습니다.
채널은 아주 간단하게 데이터를 주고받을 수 있도록 잘 구현된 크레이트이므로 최대한 활용하시는게 좋습니다.
우리가 멀티쓰레드를 활용하는 경우가 대부분 여러 쓰레드에서 처리를 하고, 결과를 하나의 쓰레드나 메인 프로세스에서 받는 경우이거나, 메인 프로세스에서 명령을 입력받아서 여러 쓰레드레 동작을 지시하는 경우입니다.
그래서 표준 크레이트에 MPSC 채널을 넣어준것 같습니다.

## Thread를 사용하기 위해 필요한 트레이트

Send
Sync
Copy는 안되는 이유

## Thread 생성 방법

Mutex, Arc를 이용한 쓰레드 생성 예제

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
let data = Arc::new(Mutex::new(0));

let (tx, rx) = channel();
for _ in 0..N {
    let (data, tx) = (data.clone(), tx.clone());
    thread::spawn(move || {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        let mut data = data.lock().unwrap();
        *data += 1;
        if *data == N {
            tx.send(()).unwrap();
        }
        // the lock is unlocked here when `data` goes out of scope.
    });
}

rx.recv().unwrap();
```

### Thread의 결과값을 받는 방법

Result<쓰레드함수의반환값, Box<dyn Any + Send>>
Trait Any에 대한 설명

