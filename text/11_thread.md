# Thread

## Thread를 사용하기 위해 미리 알아두어야할 것들

쓰레드를 만드는 것은 사실 아주 간단합니다.
쓰레드를 제대로 사용하기 위해서는 Rust에서 어떤 동기화 기법들을 제공하고 있는가를 알아야 합니다.
먼저 Rust에서 제공하는 동기화 기법들을 소개하고, 그 다음 쓰레드를 만들어서 사용해보도록 하겠습니다.

### 여러 쓰레드가 공유해서 사용할 수 있는 스마트 포인터 Arc

Arc는 Atomic Reference Counter의 약자입니다.


### 기본 자료형을 공유할 수 있는 Atomic 타입


### Mutex


### Channel

### 메모리 오더링의 기본 개념

Memory Ordering
Relaxed...

## Thread를 사용하기 위해 필요한 트레이트

Send
Sync
Copy는 안되는 이유

## Thread 생성 방법

Mutex, Arc를 이용한 쓰레드 생성 예제

### Thread의 결과값을 받는 방법

Result<쓰레드함수의반환값, Box<dyn Any + Send>>
Trait Any에 대한 설명

