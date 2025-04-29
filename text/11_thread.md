# Thread

## Thread를 사용하기 위해 필요한 동기화 기법들

Arc
Mutex
Condvar
AtomicBool

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

