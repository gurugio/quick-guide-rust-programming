# Cargo 툴 소개

C++에서 cmake는 빌드 과정만을 관리하고 실제 프로그램 빌드는 g++이라는 툴을 사용합니다. Python에서 pip는 외부 라이브러리를 다운받고 python툴로 실행합니다. C언어에서는 라이브러리 관리를 위해 yum, dnf, apt등 각 리눅스 배포판마다 다른 툴을 사용하고 컴파일러는 gcc입니다. cargo는 이런 일들을 한가지 툴로 실행하게됩니다. 사실 실제 러스트 언어의 컴파일러는 cargo가 아니라 rustc라는 툴입니다. 하지만 rustc를 직접 실행하는 일은 거의 없습니다. cargo는 rustc를 호출해서 프로그램을 빌드하는 것 외에도 다양한 일들을 해주기 때문입니다.

Cargo의 help 메세지를 보면 러스트의 패키지를 관리를 위한 툴이라고 설명합니다.

```bash
user@AL02279337 quick-guide-rust-programming % cargo help
Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]
       cargo [+toolchain] [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...

Options:
  -V, --version             Print version info and exit
      --list                List installed commands
      --explain <CODE>      Provide a detailed explanation of a rustc error message
  -v, --verbose...          Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet               Do not print cargo log messages
      --color <WHEN>        Coloring: auto, always, never
  -C <DIRECTORY>            Change to DIRECTORY before doing anything (nightly-only)
      --locked              Assert that `Cargo.lock` will remain unchanged
      --offline             Run without accessing the network
      --frozen              Equivalent to specifying both --locked and --offline
      --config <KEY=VALUE>  Override a configuration value
  -Z <FLAG>                 Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                Print help

Commands:
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
    install     Install a Rust binary
    uninstall   Uninstall a Rust binary
    ...         See all commands with --list
See 'cargo help <command>' for more information on a specific command.
```

여기에서 패키지는 일반적으로 특정한 하나의 소프트웨어 설치 파일을 말하는 패키지가 아닙니다. 러스트에서는 우리가 개발하는 프로젝트를 패키지라고 부릅니다. 패키지를 관리하는 툴이라는 의미는 프로젝트의 개발하는 모든 단계에서 사용되는 툴이라는 의미입니다. 프로젝트 개발을 위한 컴파일, 프로젝트 전체 빌드, 외부 크레이트(러스트에서는 외부 라이브러리를 크레이트라고 부릅니다) 다운로드, 실행 파일 설치 등등 개발 과정의 모든 일을 처리할 수 있습니다.

그리고 help 메세지에 나오는 명령어 리스트가 모든 명령어의 리스트가 아닙니다. 주로 많이 사용되는 명령어들만 보여주고 있습니다. 전체 명령어 리스트를 보려면 `cargo --list` 명령을 사용합니다.

```bash
user@AL02279337 quick-guide-rust-programming % cargo --list
Installed Commands:
    add                  Add dependencies to a Cargo.toml manifest file
    b                    alias: build
    bench                Execute all benchmarks of a local package
    build                Compile a local package and all of its dependencies
    c                    alias: check
    check                Check a local package and all of its dependencies for errors
    clean                Remove artifacts that cargo has generated in the past
    clippy               Checks a package to catch common mistakes and improve your Rust code.
    config               Inspect configuration values
    d                    alias: doc
    doc                  Build a package's documentation
    expand
    fetch                Fetch dependencies of a package from the network
    fix                  Automatically fix lint warnings reported by rustc
    fmt                  Formats all bin and lib files of the current crate using rustfmt.
    generate-lockfile    Generate the lockfile for a package
    git-checkout         This command has been removed
    help                 Displays help for a cargo subcommand
    info                 Display information about a package in the registry
    init                 Create a new cargo package in an existing directory
    install              Install a Rust binary
    locate-project       Print a JSON representation of a Cargo.toml file's location
    login                Log in to a registry.
    logout               Remove an API token from the registry locally
    metadata             Output the resolved dependencies of a package, the concrete used versions including overrides, in machine-readable format
    miri
    new                  Create a new cargo package at <path>
    owner                Manage the owners of a crate on the registry
    package              Assemble the local package into a distributable tarball
    pkgid                Print a fully qualified package specification
    publish              Upload a package to the registry
    r                    alias: run
    read-manifest        Print a JSON representation of a Cargo.toml manifest.
    remove               Remove dependencies from a Cargo.toml manifest file
    report               Generate and display various kinds of reports
    rm                   alias: remove
    run                  Run a binary or example of the local package
    rustc                Compile a package, and pass extra options to the compiler
    rustdoc              Build a package's documentation, using specified custom flags.
    search               Search packages in the registry. Default registry is crates.io
    t                    alias: test
    test                 Execute all unit and integration tests and build examples of a local package
    tree                 Display a tree visualization of a dependency graph
    uninstall            Remove a Rust binary
    update               Update dependencies as recorded in the local lock file
    vendor               Vendor all dependencies for a project locally
    verify-project       Check correctness of crate manifest
    version              Show version information
    yank                 Remove a pushed crate from the index
```

특정 명령어에 대한 보다 자세한 설명을 보고싶으면 `cargo help 명령어`를 사용합니다. 리눅스의 매뉴얼 페이지와 동일한 형태의 매뉴얼을 볼 수 있습니다.
```
user@AL02279337 quick-guide-rust-programming % cargo help add
CARGO-ADD(1)                           General Commands Manual                           CARGO-ADD(1)

NAME
       cargo-add — Add dependencies to a Cargo.toml manifest file

SYNOPSIS
       cargo add [options] crate…
       cargo add [options] --path path
       cargo add [options] --git url [crate…]

DESCRIPTION
       This command can add or modify dependencies.
......
```

다음부터 제가 개발하면서 자주 사용하는 명령어들을 소개하겠습니다.

## cargo init

cargo init --bin, --lib

## cargo check



## cargo build

release
debug


## cargo run




## cargo add




## cargo doc

cargo rustdoc
cargo doc


## cargo test

cargo test test_name -- --nocapture