
# 러스트 프로그래밍 빠르게 시작하기

## 목차

* [러스트 언어 소개](text/00_intro.md)
* [러스트 개발 환경 설치](text/01_start.md)
* [러스트 기본 문법](text/02_basic.md)
* [함수형 프로그래밍](text/03_functional.md)
* [트레이트](text/04_trait.md)
* [제네릭과 수명](text/05_generic_lifetime.md)
* [스마트 포인터](text/06_smart_pointer.md)
* [표준 라이브러리와 표준 트레이트](text/07_std.md)

## 책 소개

*업데이트: 함수형 프로그래밍 챕터를 추가했습니다.*

이미 프로그래밍을 조금이라도 해보신 분들을 대상으로 합니다.
프로그래밍의 기본적인 문법들을 처음부터 설명하지 않습니다. for루프가 뭔지 if가 뭔지 등을 설명하지않고 바로 Rust에서는 이렇게 사용합니다로 설명합니다.

책의 범위는 싱글 쓰레드로 터미널 기반의 간단한 프로그램을 만들 정도의 내용을 가지고 있습니다.
멀티쓰레드를 위한 동기화 기업과 락킹 등에 대해서는 다루지 않습니다.

## 예제 실행 방법

Cargo.toml파일에 각 예제의 실행 파일을 빌드하는 설정이 있습니다. 
```
[[bin]]
name = "function_for"
path = "src/function_for/main.rs"
```

다음과 같이 cargo를 이용해서 빌드하고 실행할 수 있습니다.
```
gurugio@AL01945427:~/my-rust-book$ cargo build --bin function_for
   Compiling my-rust-book v0.1.0 (/home/gurugio/my-rust-book)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
gurugio@AL01945427:~/my-rust-book$ cargo run --bin function_for
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/function_for`
Hello, function_for!
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
```

## License

이 책의 컨텐츠와 예제 코드는 다음 라이선스를 따릅니다.

This project is distributed under the following licenses:

* The code samples and free-standing Cargo projects contained within this book are licensed under the terms of both the [MIT License] and the [Apache License v2.0].
* The written prose contained within this book is licensed under the terms of the Creative Commons [CC-BY-SA v4.0] license.

Copies of the licenses used by this project may also be found here:

* [MIT License Hosted]
* [Apache License v2.0 Hosted]
* [CC-BY-SA v4.0 Hosted]

[MIT License]: ./LICENSE-MIT
[Apache License v2.0]: ./LICENSE-APACHE
[CC-BY-SA v4.0]: ./LICENSE-CC-BY-SA
[MIT License Hosted]: https://opensource.org/licenses/MIT
[Apache License v2.0 Hosted]: http://www.apache.org/licenses/LICENSE-2.0
[CC-BY-SA v4.0 Hosted]: https://creativecommons.org/licenses/by-sa/4.0/legalcode

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Resources team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-resources-team
