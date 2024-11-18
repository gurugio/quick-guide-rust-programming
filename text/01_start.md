# 처음 시작하기

Rust를 경험해보기 위한 환경을 준비하고, 앞으로 소개할 예제들을 실행해볼 수 있는 간단한 프로젝트를 만들어보겠습니다.

## 개발환경

### Rust 컴파일러 설치

최신 언어 트랜드에 맞게 컴파일러 설치나 개발 환경 셋팅은 아주 간단합니다. Rust언어의 개발 환경을 설치하기 위한 rustup이라는 설치 관리자가 있고, <https://rustup.rs/> 에 접속하면 어떻게 rustup이라는 툴을 설치할 수 있는지를 알려줍니다.

사이트에 접속하시면 사용중인 운영체제에 따라 조금씩 다른 메세지나 명령을 보여줍니다. 리눅스/유닉스 계열이라면 다음과 같은 명령을 알려줄 것입니다.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

다음은 윈도우에서 WSL을 설치한 후 위 명령을 실행한 결과를 보여줍니다.

```bash
gurugio@AL01945427:~$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/gurugio/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/gurugio/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/gurugio/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/gurugio/.profile
  /home/gurugio/.bashrc

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:

   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2024-02-08, rust version 1.76.0 (07dca489a 2024-02-04)
info: downloading component 'cargo'
  8.5 MiB /   8.5 MiB (100 %)   1.2 MiB/s in  7s ETA:  0s
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)   1.4 MiB/s in 12s ETA:  0s
info: downloading component 'rust-std'
 23.9 MiB /  23.9 MiB (100 %) 485.6 KiB/s in 41s ETA:  0s
info: downloading component 'rustc'
 62.3 MiB /  62.3 MiB (100 %)   2.6 MiB/s in 40s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)  10.4 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 23.9 MiB /  23.9 MiB (100 %)  18.0 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 62.3 MiB /  62.3 MiB (100 %)  19.6 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.76.0 (07dca489a 2024-02-04)

Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

중간에 설치 환경 셋팅을 디폴트로 진행할 것인지, 본인이 수정할 것인지 물어보는데 1번을 입력해서 디폴트 환경으로 설치를 진행했습니다. 디폴트 환경을 선택하면 rustc같은 컴파일러와 cargo, clippy, rustfmt 등 개발에 필수적인 툴들을 자동으로 설치하고 환경 변수를 자동으로 설정해줍니다.

설치가 완료되면 아래와같이 .cargo/env 파일이 생성됩니다.

```bash
gurugio@AL01945427:~$ cat .cargo/env
#!/bin/sh
# rustup shell setup
# affix colons on either side of $PATH to simplify matching
case ":${PATH}:" in
    *:"$HOME/.cargo/bin":*)
        ;;
    *)
        # Prepending path in case a system-installed rustc needs to be overridden
        export PATH="$HOME/.cargo/bin:$PATH"
        ;;
esac
```

실행파일을 찾는 환경변수 PATH에 .cargo/bin을 추가하는 파일입니다. .cargo/bin 디렉토리는 cargo, rustc등 Rust개발에 필요한 실행 파일들이 저장되어있습니다.

이제 `source .bashrc` 명령을 실행하면 새로운 환경 변수가 적용되서 cargo등 실행파일을 사용할 수 있게 됩니다.

```bash
gurugio@AL01945427:~$ source .bashrc
gurugio@AL01945427:~$ cargo --version
cargo 1.76.0 (c84b36747 2024-01-18)
gurugio@AL01945427:~$ rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
```

## Cargo 툴 소개

최신 프로그래밍 언어들은 대부분 프로젝트를 쉽게 빌드할 수 있게 해주고, 라이브러리를 관리할 수 있는 툴을 제공합니다. C++은 cmake가 있고 C에는 make가 있긴 하지만, 사용하기 편리하다고 하기는 어렵습니다. Makefile이나 CMakeLists.txt 문법이 강력하다고는 할 수 있지만 직관적이라고 하기는 어렵다는 것은 많은 분들이 동의하실겁니다.

Rust 언어로 개발할 때 가장 많이 사용하게되는 툴은 Cargo라는 툴 입니다. 프로젝트를 생성과 빌드, 외부 라이브러리 다운로드 등 프로젝트 관리를 위한 모든 일을 하는 툴입니다. 도움말을 확인해보면 new, build, clean등 이름만 봐도 무슨 일을 하는 것인지 알수있는 명령들이 보입니다.

```bash
% cargo help
Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]

Options:
  -V, --version             Print version info and exit
      --list                List installed commands
      --explain <CODE>      Run `rustc --explain CODE`
  -v, --verbose...          Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet               Do not print cargo log messages
      --color <WHEN>        Coloring: auto, always, never
  -C <DIRECTORY>            Change to DIRECTORY before doing anything (nightly-only)
      --frozen              Require Cargo.lock and cache are up to date
      --locked              Require Cargo.lock is up to date
      --offline             Run without accessing the network
      --config <KEY=VALUE>  Override a configuration value
  -Z <FLAG>                 Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                Print help

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.
```

가장 처음 사용하게되는 명령은 cargo new 명령입니다. 새로운 프로젝트를 생성하는 명령입니다. Rust는 하나의 프로젝트(여러개의 실행파일과 실행 파일에 필요한 여러개의 라이브러리가 함께있는)를 패키지라고 부릅니다. 그래서 new 명령의 설명에 새로운 cargo package를 만든다는 설명이 있는 것입니다.
new 명령을 아래와 같이 실행하면 새로운 디렉토리와 하위 디렉토리 및 파일들을 만들어줍니다. Rust언어의 패키지나 프로젝트 관리에 대해서는 나중에 다시 설명하겠습니다.

```bash
% cargo new my-rust-book
l     Created binary (application) `my-rust-book` package
% cd my-rust-book
% ls
Cargo.toml  src
% ls -R
Cargo.toml  src

./src:
main.rs
```

cargo new 명령으로 my-rust-book이라는 디렉토리와, my-rust-book/src라는 디렉토리가 생성됩니다. 그리고 my-rust-book/Cargo.toml, my-rust-book/src/main.rs 파일들이 생성됩니다.

참고로 이미 존재하는 디렉토리를 Rust언어 프로젝트 디렉토리로 만들고 있다면 아래와 같이 cargo init 명령을 사용하면 됩니다. Cargo.toml 파일을 생성하고 그 외에 src디렉토리를 만들지 않는 것을 알 수 있습니다.

Cargo.toml은 현재 디렉토리에 있는 main.rs라는 파일을 빌드해서 pyalgo라는 실행파일을 만들 것입니다.

```bash
user@AL02279337 pyalgo % ls
main.rs
user@AL02279337 pyalgo % cargo init
     Created binary (application) package
user@AL02279337 pyalgo % ls
Cargo.toml main.rs
user@AL02279337 pyalgo % cat Cargo.toml
[package]
name = "pyalgo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[[bin]]
name = "pyalgo"
path = "main.rs"
```

지금은 개발 환경이 잘 설치되었는지를 확인하는 단계이므로 여기에서 cargo의 모든 명령어를 자세히 설명하지는 않겠습니다. 프로그래밍 경험이 있고, apt나 yum등의 패키지 관리자를 사용해본 경험이 있다면 굳이 자세하게 설명할 필요없이 cargo help의 도움말 메세지만 봐도 충분히 이해할 수 있을만큼 쉽고 강력한 툴입니다. 또 널리 사용되고 문법이 간단한 toml 포맷을 사용하므로 쉽게 적응할 수 있으실겁니다.

## Hello, world! 구현하기

“cargo new” 명령을 사용해서 간단한 바이너리 파일을 빌드하기 위한 패키지를 생성해보겠습니다. 다음은 my-rust-book이라는 패키지를 생성하는 방법입니다.

```bash
% cargo new my-rust-book
l     Created binary (application) `my-rust-book` package
% cd my-rust-book
% ls
Cargo.toml  src
% ls -R
Cargo.toml  src

./src:
main.rs
```

위와같이 생성된 패키지를 확인해보면 최상위 디렉토리에 src 디렉토리와 Cargo.toml 파일이 생성됩니다. src디렉토리에는 main.rs 파일이 생성되었습니다.

Cargo.toml 파일의 가장 중요한 역할은 패키지의 이름과 버전 등을 관리하고, 패키지에서 참조할 라이브러리를 관리하는 것입니다. Cargo.toml 파일을 한번 열어보겠습니다.

```bash
% cat Cargo.toml
[package]
name = "ch01"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

package 섹션에 있는 name과 version은 패키지의 이름과 버전입니다. edition이라는 항목은 Rust 언어의 버전입니다. 숫자를 보면 알수있듯이 2021년도에 정의된 버전입니다. 이 책을 쓰는 2024년 기준으로 가장 최신 버전은 2021입니다. 그 이전에는 2018 버전이 있었습니다. 지금 단계에서는 Cargo툴이 기본으로 지정해주는 값을 그대로 쓰면 됩니다.

dependencies 섹션은 이 패키지에서 사용될 외부 라이브러리를 지정합니다. 이 책에서는 표준 라이브러리만 사용하지만, 만약에 외부 라이브러리를 사용하고 싶다면 다음과 같이 “cargo add” 명령으로 라이브러리를 dependencies 섹션에 추가할 수 있습니다. 다음은 anyhow라는 라이브러리를 추가해본 것입니다.

```bash
gurugio@AL01945427:~/my-rust-book$ cargo add anyhow
    Updating crates.io index
      Adding anyhow v1.0.80 to dependencies.
             Features:
             + std
             - backtrace
    Updating crates.io index
gurugio@AL01945427:~/my-rust-book$ cat Cargo.toml
[package]
name = "my-rust-book"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.80"
```

crate.io라는 메세지가 나오는데 <https://crates.io/> 라는 사이트에서 anyhow라이브러리를 다운받기 때문입니다. crates.io사이트에 접속해서 anyhow를 검색해보면 가장 최신 버전이 1.0.80인것을 확인할 수 있습니다.

그럼 개발 환경 셋팅이 끝났으니 간단하게 Hello, World!를 출력해보겠습니다. 다음과 같이 main.rs를 만들어보겠습니다.

```rust
// src/main.rs
fn main() {
    println!("Hello, World!");
    println!("{}", "Hello, World again!");
    println!("{:x}", 65535);
}
```

```bash
$ cargo run --bin hello
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/hello`
Hello, World!
Hello, World again!
ffff
```

main함수는 일반 함수입니다. 위의 예제에서는 인자도 없고 반환값도 없어서 모두 생략되었지만, 필요하다면 인자나 반환값을 추가할 수 있습니다. println!()은 이름 그대로 문자열을 출력하는 매크로 함수입니다. 함수 이름 끝에 !가 붙은 것은 매크로로 정의된 함수를 호출한다는 것입니다. println!() 이라는 이름을 보면 ln은 line의 약자라는 것을 짐작할 수 있습니다. 저는 printf를 사용할 때마다 ‘\n’을 빼놓는 실수를 자주하곤 했는데 Rust에는 아예 println을 만들어놔서 그런 실수를 하지 않게되었습니다. (하지만 println!에서 println이라고만 쓰는 실수를 하게되었습니다.)

출력되는 메세지의 포맷을 지정하는 것은 “{}”를 사용합니다. 파이선과 유사합니다. “{”와 “}” 사이에 형식을 지정하는 표시가 들어가는 것도 파이선과 유사합니다. {:x}는 다른 언어들에서 많이 쓰듯이 16진수를 출력한다는 의미입니다.
