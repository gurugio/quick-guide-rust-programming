# Asynchronous programming

## Asynchronous programming이 무엇인가?

여러가지 서로다른 태스크를 동시에 실행하는 방식이 제가 알기로 3가지 정도가 있습니다.
첫번째가 병령 프로그래밍 (Parallel programming), 동시성 프로그래밍 (Concurrent programming) 그리고 비동기 프로그래밍 (Asynchronous programming) 입니다.
어떤 차이가 있는지를 간략하게 이야기해보자면
1. 병렬 프로그래밍: 가장 대표적인 것이 GPU연산입니다. 같은 연산을 다른 데이터를 가지고 실행하는데 물리적으로 여러 연산들을 동시에 실행하는 것입니다.
2. 동시성 프로그래밍: 1개 코어를 가진 CPU가 여러가지 쓰레드를 실행하면 사람에게는 마치 모든 쓰레드가 동시에 실행되는 것가 같이 보일 것입니다. 하지만 엄밀히는 각 쓰레드가 동시에 실행되는 것이 아니라 아주 빠르게 스위칭되기 때문에 마치 동시에 실행되는 것처럼 보이는 것입니다. 이렇게 여러 태스크를 스위칭해가면서 실행하는 것을 동시성 프로그래밍입니다. 쓰레드를 사용하는 것이 대표적인 예이고, 쓰레드를 사용하기 위해 다양한 락이나 스케줄링 기법들이 필요하게됩니다.
3. 비동기 프로그래밍: 어떤 태스크를 실행하도록 명령한 후 해당 태스트가 실행되는 것을 기다릴 필요없이 다른 작업을 할 수 있는 것이 비동기 프로그래밍입니다. 실 생활에서도 비동기로 처리되는 것들이 많습니다. 은행에 대출 서류를 제출한 후에 대출을 받을 때까지 우리가 은행에서 기다릴 필요가 없습니다. 우리는 은행에서 대출 준비가 완료되었다는 연락을 받을 때까지 우리 일상 생활을 계속 하면 됩니다. 그러다가 대출 준비가 완료되었다는 연락을 받으면, 그 다음에 대출받은 돈을 가지고 해야 할 일을 하면 됩니다.

거의 모든 운영체제와 언어들이 각자 나름대로의 비동기 처리를 위한 기법을 제공하고 있습니다.
최신 리눅스 커널은 epoll이나 io_uring이라는 비동기적인 파일 입출력 처리 기법을 제공합니다.
보통 프로그래밍을 처음 시작하는 분들은 read/write 시스템 콜을 사용하셨을 것입니다.
이런 전통적인 시스템 콜은 읽기나 쓰기위한 데이터가 모두 처리가 끝나야 함수가 끝나고 함수의 반환값을 받게됩니다.
하지만 epoll이나 io_uring은 파일에 어떤 데이터를 읽고 쓰라는 명령을 내린 후 다른 일을 하면서 데이터 처리가 끝나기를 기다릴 수 있습니다.
그래소 비동기적인 파일 입출력이라고 부릅니다.

쓰레드를 이용하는 동시성 프로그래밍과 크게 다르지 않아보일 수도 있습니다.
비동기적으로 실행할 태스크가 있을 때마다 쓰레드를 만들어서 태스크를 실행하고 쓰레드를 없애면, 비동기 처리와 같아 보일 수 있습니다.
하지만 크게 2가지 관점을 생각해봐야합니다.
1. 쓰레드로 실행하는 비동기 처리는 프로세서의 갯수까지의 태스크만 동시에 실행할 수 있습니다. 프로세서가 8개면 최대 8개의 태스크가 동시에 실행될 수 있고, 그 외의 쓰레드는 기다릴 수밖에 없습니다. 비동기 프로그래밍을 하게되면 이러한 제약없이 더 많은 태스크를 동시에 실행할 수 있습니다.
2. 쓰레드를 생성하고 제거하는 것이 프로그램에 큰 부담이 됩니다. 특히 아주 작은 태스크를 여러개 실행해야할 때 쓰레드를 만들고 제거하는 컨텍스트 스위칭에 들어가는 시간이 태스크의 실행 시간에 비해 점점 더 커지게됩니다. 비동기 프로그래밍을 하게되면 쓰레드를 관리하는 비용없이 순수하게 태스크만 처리할 수 있으므로 더 빨리 원하는 처리를 할 수 있습니다.

## async task 만들기

비동기로 실행되는 태스크라는 것이 무엇인지 알아보기 위해 일단 가상의 태스크를 한번 만들어보겠습니다.
비동기 태스크를 만들고 실행하기 위해서 가장 먼저 필요한 것은 tokio라는 크레이트를 추가하는 것입니다.
아래와같이 `cargo add tokio --features=full` 명령으로 Cargo.toml에 추가할 수도 있고, 직접 Cargo.toml 파일에 입력해서 추가할 수도 있습니다.

```
$ cargo add tokio --features=full

$ cat Cargo.toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.44.2", features = ["full"] }
```

그리고 일단 다음 예제를 한번 실행해봅니다.

```rust
use std::time::Duration;

async fn task_one() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-one");
    1
}

async fn task_two() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    let v1 = task_one().await;
    let v2 = task_two().await;
    println!("v1={} v2={}", v1, v2);
}
```
```bash
 % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/bin-example`
Start task-one
Finish task-one
Start task-two
Finish task-two
v1=1 v2=2
```




```rust
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

async fn task_one() -> i32 {
    println!("Start task-one");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-one");
    1
}

async fn task_two() -> i32 {
    println!("Start task-two");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finish task-two");
    2
}

#[tokio::main]
async fn main() {
    // 1. run each async task
    let v1 = task_one().await;
    let v2 = task_two().await;
    println!("v1={} v2={}", v1, v2);

    // 2. Run two async task concurrently
    let one = task_one();
    let two = task_two();
    tokio::join!(one, two);
    // How to get the return values?
}

```


std::thread::sleep을 사용하는 경우와 tokio::time::sleep를 사용하는 것의 차이
```rust
async fn do_sleep(i: i32) {
    let secs = std::time::Duration::from_secs(5);
    println!("{} do_sleep: sleep", i);
    //std::thread::sleep(secs);
    tokio::time::sleep(secs).await;
    println!("{} do_sleep: end", i);
}

#[tokio::main]
async fn main() {
    let mut v = Vec::new();

    for i in 0..64 {
        let t = tokio::spawn(do_sleep(i));
        v.push(t);
    }

    println!("join");
    for h in v {
        _ = h.await;
    }
}
```

```rust
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

async fn prep_coffee_mug() {
    sleep(Duration::from_millis(100)).await;
    println!("Pouring milk..");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured");
    println!("Putting instant coffee..");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put");
}

async fn make_coffee() {
    println!("Boiling kettle..");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("kettle boiled");
    println!("pouring boiled water..");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured");
}

async fn make_toast() {
    println!("putting bread in toaster..");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("bread toasted");
    println!("Buttering toasted bread");
    thread::sleep(Duration::from_secs(5));
    println!("toasted bread buttered");
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    // 1. run one async function
    prep_coffee_mug().await;

    // 2. run some async functions
    let _ = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug();
        let coffee_step = make_coffee();
        let toast_step = make_toast();
        tokio::join!(coffee_mug_step, coffee_step, toast_step);
    })
    .await;

    // 3. run two async block concurrently
    let person_one = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug();
        let coffee_step = make_coffee();
        let toast_step = make_toast();
        tokio::join!(coffee_mug_step, coffee_step, toast_step);
    });

    let person_two = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug();
        let coffee_step = make_coffee();
        let toast_step = make_toast();
        tokio::join!(coffee_mug_step, coffee_step, toast_step);
    });

    let _ = tokio::join!(person_one, person_two);

    // etc
    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}

```


async test
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post() {
        let post_uri = "/post-data";
        let post_path = &post_uri[1..];
        let req = Request::builder()
            .header("host", "localhost:8080")
            .method("POST")
            .uri(post_uri)
            .body(hyper::Body::from("hello"))
            .unwrap();
        let mut response = Response::new(Body::empty());

        response_post(req, &mut response).await.unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);

        let mut fs = File::open(post_path).unwrap();
        let mut contents = String::new();

        fs.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "hello");

        // cleanup!!
        let _ = fs::remove_file(post_path);
    }
}
```



## 예제

```rust
use anyhow::Result;
use http::{Request, Response};
use hyper::{server::conn::Http, service::service_fn, Body};
use hyper::{Method, StatusCode};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;

///
/// How to test
/// run 2 terminals:
/// terminal-1: cargo run
/// terminal-2: for i in {0..10}; do curl http://127.0.0.1:8080/ & done
/// See that http_response starts immediately for each connection without sleeping 5 seconds.
///
/// $ cargo run
/// start http handling 1668004667.467865888s
/// start http handling 1668004667.467865888s
/// start http handling 1668004667.467865888s
/// start http handling 1668004667.467867354s
/// start http handling 1668004667.467936009s
/// start http handling 1668004667.467867354s
/// start http handling 1668004667.46789599s
///
/// COMMAND: POST
/// curl -d '{"key1":"value1", "key2":"value2"}' -H "Content-Type: application/json" -X POST http://localhost:8080/data
///
/// COMMAND: GET
/// curl -X GET http://localhost:8080/data

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    let tcp_listener = TcpListener::bind(addr).await?;
    loop {
        let (tcp_stream, _) = tcp_listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(http_err) = Http::new()
                .serve_connection(tcp_stream, service_fn(http_response))
                .await
            {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        });
    }
}

async fn http_response(req: Request<Body>) -> Result<Response<Body>> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("start http handling {:?}: {:?}", since_the_epoch, &req);

    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => response_get(req, &mut response).await?,
        &Method::PUT => response_put(req, &mut response).await?,
        &Method::POST => response_post(req, &mut response).await?,
        &Method::DELETE => response_delete(req, &mut response).await?,
        _ => {
            *response.body_mut() = Body::from("Unidentified request-method");
            *response.status_mut() = StatusCode::NOT_IMPLEMENTED;
        }
    }

    Ok(response)
}

async fn response_delete(req: Request<Body>, response: &mut Response<Body>) -> Result<()> {
    let uri = req.uri().to_string();
    let _ = fs::remove_file(&uri[1..]);
    *response.status_mut() = StatusCode::ACCEPTED;
    Ok(())
}

async fn response_post(req: Request<Body>, response: &mut Response<Body>) -> Result<()> {
    let uri = req.uri().to_string();
    let contents = hyper::body::to_bytes(req.into_body()).await?;
    let mut file = File::create(&uri[1..])?;

    file.write_all(&contents)?;
    *response.status_mut() = StatusCode::ACCEPTED;
    Ok(())
}

async fn response_put(req: Request<Body>, response: &mut Response<Body>) -> Result<()> {
    let uri = req.uri().to_string();
    let contents = hyper::body::to_bytes(req.into_body()).await?;
    // open file with write permission
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&uri[1..])?;
    // over-write data
    file.write_all(&contents)?;
    *response.status_mut() = StatusCode::ACCEPTED;
    Ok(())
}

async fn response_get(req: Request<Body>, response: &mut Response<Body>) -> Result<()> {
    let uri = req.uri().to_string();
    let mut fs = File::open(&uri[1..])?;
    let mut contents = String::new();

    fs.read_to_string(&mut contents)?;
    *response.body_mut() = Body::from(contents);
    *response.status_mut() = StatusCode::ACCEPTED;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post() {
        let post_uri = "/post-data";
        let post_path = &post_uri[1..];
        let req = Request::builder()
            .header("host", "localhost:8080")
            .method("POST")
            .uri(post_uri)
            .body(hyper::Body::from("hello"))
            .unwrap();
        let mut response = Response::new(Body::empty());

        response_post(req, &mut response).await.unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);

        let mut fs = File::open(post_path).unwrap();
        let mut contents = String::new();

        fs.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "hello");

        // cleanup!!
        let _ = fs::remove_file(post_path);
    }

    #[tokio::test]
    async fn test_delete() {
        let delete_uri = "/delete-data";
        let delete_path = &delete_uri[1..];
        {
            let mut file = File::create(delete_path).unwrap();
            file.write_all(b"hello").unwrap();
        }

        let req = Request::builder()
            .header("host", "localhost:8080")
            .method("DELETE")
            .uri(delete_uri)
            .body(hyper::Body::from("hello"))
            .unwrap();
        let mut response = Response::new(Body::empty());

        response_delete(req, &mut response).await.unwrap();
        assert!(File::open(delete_path).is_err());

        // cleanup!!
        let _ = fs::remove_file(delete_path);
    }

    #[tokio::test]
    async fn test_get() {
        let get_uri = "/get-data";
        let get_path = &get_uri[1..];
        {
            let mut file = File::create(get_path).unwrap();
            file.write_all(b"hello").unwrap();
        }

        let req: Request<Body> = Request::builder()
            .header("host", "localhost:8080")
            .method("GET")
            .uri(get_uri)
            .body(hyper::Body::empty())
            .unwrap();
        let mut response = Response::new(Body::empty());

        response_get(req, &mut response).await.unwrap();

        // How to convert hyper::Body to String
        // https://stackoverflow.com/questions/63301838/how-to-read-the-response-body-as-a-string-in-rust-hyper
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            String::from_utf8(bytes.to_vec()).unwrap(),
            "hello".to_string()
        );

        let _ = fs::remove_file(get_path);
    }

    #[tokio::test]
    async fn test_put() {
        let put_uri = "/put-data";
        let put_path = &put_uri[1..];
        {
            let mut file = File::create(put_path).unwrap();
            file.write_all(b"hello").unwrap();
        }

        let req: Request<Body> = Request::builder()
            .header("host", "localhost:8080")
            .method("PUT")
            .uri(put_uri)
            .body(hyper::Body::from("xxx"))
            .unwrap();
        let mut response = Response::new(Body::empty());

        response_put(req, &mut response).await.unwrap();

        let mut fs = File::open(put_path).unwrap();
        let mut contents = String::new();

        fs.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "xxxlo");

        // cleanup!!
        let _ = fs::remove_file(put_path);
    }
}
```


```rust
use anyhow::{Context, Result};
use boolinator::Boolinator;
use futures::stream::StreamExt;
use futures::{future, Stream};
use inotify::{Event, EventMask, Inotify, WatchMask};
use std::path::PathBuf;
use tokio::runtime;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // get a future instance because it is "async fn".
    let iwatcher = newwatcher();
    print_type_of(&iwatcher); // check the exact type name

    // create a runtime of tokio
    // that runs the async function.
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // This does not finish but loop on iwatcher
    let _ = rt.block_on(iwatcher);
}

async fn newwatcher() -> Result<()> {
    // just an internal buffer for Inotify stream
    let buf: Vec<u8> = vec![0; 64];
    // PathBuf instance for the current directory
    let path: PathBuf = std::env::current_dir()?.into();
    // Box::pin is mandatory for .next() method
    // because .next() requires unpin.
    let new_files = Box::pin(InotifyTxtStream::new(buf, path.to_owned()).unwrap());

    // add inotify for existing txt files
    let existing_files = FsTxtStream::new(path.to_owned()).unwrap();
    let mut txt_files = existing_files.chain(new_files);
    while let Some(name) = txt_files.next().await {
        println!("{:?}", name);
    }
    Ok(())
}

struct InotifyTxtStream {}

impl InotifyTxtStream {
    async fn txt_path((ev, dir_path): (Event<std::ffi::OsString>, PathBuf)) -> Option<PathBuf> {
        match ev.mask {
            EventMask::CREATE => {
                let path = dir_path.join(ev.name?);
                let is_txt = path.extension()? == "txt";
                is_txt.as_some(path)
            }
            _ => None,
        }
    }
    /// return an inotify instance streaming target path
    pub fn new(buffer: Vec<u8>, target_path: PathBuf) -> Result<impl Stream<Item = PathBuf>> {
        // make a inotify instance watching create event
        let mut inotify = Inotify::init()?;
        inotify
            .add_watch(&target_path, WatchMask::CREATE)
            .with_context(|| {
                format!(
                    "Failed to open target directory {}\n",
                    target_path.display()
                )
            })?;

        // event-stream: multi-event
        // take_while: take event only ready -> why?
        // map: create a pair (event, path)
        // filter_map: filtering only path has txt extension
        let inotify_stream = inotify
            .event_stream(buffer)?
            .take_while(|ev| future::ready(ev.is_ok()))
            .map(move |ev| (ev.unwrap(), target_path.clone()))
            .filter_map(InotifyTxtStream::txt_path);

        // return a stream that return PathBuf including txt
        Ok(inotify_stream)
    }
}

struct FsTxtStream {}

impl FsTxtStream {
    fn txt_path(dir_entry: std::fs::DirEntry) -> Option<PathBuf> {
        let file_type = dir_entry.file_type().ok()?;
        let is_txt =
            file_type.is_file() && !file_type.is_dir() && dir_entry.path().extension()? == "txt";
        is_txt.as_some(dir_entry.path())
    }

    pub fn new(target_path: PathBuf) -> Result<impl Stream<Item = PathBuf>> {
        // futures::stream::iter: convert iterator to stream
        // read_dir: iterator to read target path
        let fs_txt_stream = futures::stream::iter(
            std::fs::read_dir(target_path)?
                .flatten()
                .filter_map(FsTxtStream::txt_path),
        );
        // return a stream that return PathBuf including txt
        Ok(fs_txt_stream)
    }
}
```
