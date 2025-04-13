
# Log

# Signal handling

```rust
use std::collections::HashMap;
//use std::env;

use tracing::info;
use tracing::span;
use tracing::trace;
use tracing::Instrument;
use tracing::Level;
use tracing_subscriber::EnvFilter;

//use std::io::Error;

use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct Context {
    variables: HashMap<String, String>,
    dry_run: bool,
}

impl Context {
    async fn from_path(path: &str) -> anyhow::Result<Self> {
        let mut variables = HashMap::new();
        tracing::error!("Creating context");
        info!("Creating context from path: {}", path);
        tracing::debug!("Creating context");
        trace!(?path);

        // test span again
        // If log level is info, process_recipe() prints log message like
        // 2025-02-23T15:10:29.315933Z  INFO cook{path="/path/to/file"}: rust_log: process_recipe::Ingredients: ["Pasta", "Eggs", "Bacon", "Parmesan"]
        // If log level is debug, process_recipe() prints log message like
        // 2025-02-23T15:09:40.495049Z  INFO my_span_main{path="/path/to/file"}:cook{path="/path/to/file"}: rust_log: process_recipe::Ingredients: ["Pasta", "Eggs", "Bacon", "Parmesan"]
        // because the span of "my_span_main" is added for the debug level.
        let span = span!(Level::INFO, "cook", ?path);
        process_recipe().instrument(span).await?;

        variables.insert("path".to_owned(), path.to_owned());
        Ok(Self {
            variables,
            dry_run: false,
        })
    }
}

async fn process_recipe() -> anyhow::Result<()> {
    let recipe = "Pasta Carbonara";
    let ingredients = vec!["Pasta", "Eggs", "Bacon", "Parmesan"];
    let steps = vec![
        "Cook the pasta",
        "Fry the bacon",
        "Mix the eggs and cheese",
        "Combine everything",
    ];

    tracing::error!("process_recipe::Recipe: {}", recipe);
    tracing::info!("process_recipe::Ingredients: {:?}", ingredients);
    tracing::debug!("Steps: {:?}", steps);

    Ok(())
}

async fn handle_signals(mut signals: Signals, term: Arc<AtomicBool>) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration
                // Reopen the log file
                println!("SIGHUP received");
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                println!("SIGTERM, SIGINT, or SIGQUIT received");
                term.store(true, Ordering::Relaxed);
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    // error,rust_log=info: rust_log crate can print info-level log messages, others can print only error level
    // But env RUST_LOG can override this setting, for example) RUST_LOG=debug cargo run
    let envfilter = EnvFilter::builder()
        .try_from_env()
        .unwrap_or_else(|_| EnvFilter::new("error,rust_log=info")); // try with rust_log=debug and rust_log=info to test two span! calls
    tracing_subscriber::fmt().with_env_filter(envfilter).init();

    tracing::error!("This is an error message");
    tracing::info!("This is an info message"); // This is not printed because the filter is set to info.
    tracing::debug!("This is a debug message"); // This is not printed because the filter is set to info.

    let path = "/path/to/file".to_owned();

    // ADD "my_span_main{path="/path/to/file"}" to the log message if the log level is DEBUG
    let span = span!(Level::DEBUG, "my_span_main", ?path);

    let context = Context::from_path(&path).instrument(span.clone()).await?;
    tracing::info!("info level message context={:?}", context);
    tracing::debug!("debug message");

    let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();

    let term = Arc::new(AtomicBool::new(false));
    let signals_task = tokio::spawn(handle_signals(signals, term.clone()));

    // Execute your main program logic

    while !term.load(Ordering::Relaxed) {
        // Do some time-limited stuff here
        // (if this could block forever, then there's no guarantee the signal will have any
        // effect).
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("sleeping");
    }

    // Terminate the signal stream.
    handle.close();
    signals_task.await?;

    Ok(())
}
```



# Raw memory handling

```
use std::alloc;
use std::fmt;
use std::io;
use std::ptr::NonNull;

struct Deque {
    buffer: NonNull<i32>,
    capacity: isize, // buffer size
    front: isize,
    back: isize,
}

impl Deque {
    fn new() -> Self {
        Deque {
            buffer: NonNull::dangling(),
            capacity: 0,
            front: 0,
            back: 0,
        }
    }

    fn size(&self) -> isize {
        if self.front >= self.back {
            self.front - self.back
        } else {
            self.capacity - self.back + self.front
        }
    }

    fn capacity(&self) -> isize {
        self.capacity
    }

    fn empty(&self) -> bool {
        self.size() == 0
    }

    fn full(&self) -> bool {
        // -1: an entry pointed by self.back must be empty
        self.size() == self.capacity() - 1
    }

    //
    // refer: https://doc.rust-lang.org/nomicon/vec/vec-raw.html
    //
    fn grow(&mut self, new_cap: isize) {
        // https://doc.rust-lang.org/alloc/alloc/fn.alloc.html
        let new_layout = alloc::Layout::array::<i32>(new_cap as usize).unwrap();
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = alloc::Layout::array::<i32>(self.capacity as usize).unwrap();
            let old_ptr = self.buffer.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.buffer = match NonNull::new(new_ptr as *mut i32) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.capacity = new_cap;
    }

    fn front(&self) -> i32 {
        if self.empty() == true {
            return -1;
        }
        let v: i32 = unsafe { *self.buffer.as_ptr().offset(self.front) };
        v
    }

    fn back(&self) -> i32 {
        if self.empty() == true {
            return -1;
        }
        let v: i32 = unsafe { *self.buffer.as_ptr().offset((self.back + 1) % self.capacity) };
        v
    }

    fn push_front(&mut self, v: i32) {
        assert!(!self.full());

        // move pointer and insert data
        self.front = (self.front + 1) % self.capacity;
        unsafe {
            *self.buffer.as_ptr().offset(self.front) = v;
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.empty() == true {
            return -1;
        }
        let ret = self.front();

        // no need to delete data
        // just move pointer backward
        self.front = if self.front == 0 {
            self.capacity - 1
        } else {
            self.front - 1
        };
        ret
    }

    fn push_back(&mut self, v: i32) {
        assert!(!self.full());

        // insert data and move pointer
        unsafe {
            *self.buffer.as_ptr().offset(self.back) = v;
        }
        self.back = if self.back == 0 {
            self.capacity - 1
        } else {
            self.back - 1
        };
    }

    fn pop_back(&mut self) -> i32 {
        if self.empty() == true {
            return -1;
        }
        let ret = self.back();
        self.back = (self.back + 1) % self.capacity;
        ret
    }
}

impl fmt::Debug for Deque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v: i32;
        let mut output: String = format!(
            "==== Deque: capacity={} back={} front={}\n",
            self.capacity, self.back, self.front
        );
        for i in 0..self.capacity {
            v = unsafe { *self.buffer.as_ptr().offset(i) };
            if v != 0 {
                output.push_str(&format!("buffer[{}]={}\n", i, v));
            }
        }

        write!(f, "{}====", output)
    }
}

impl Drop for Deque {
    fn drop(&mut self) {
        if self.capacity != 0 {
            let layout = alloc::Layout::array::<i32>(self.capacity as usize).unwrap();
            unsafe { alloc::dealloc(self.buffer.as_ptr() as *mut u8, layout) }
        }
    }
}

fn _test_push() {
    let mut que = Deque::new();
    que.grow(8);
    assert_eq!(que.capacity, 8);
    assert_eq!(que.empty(), true);
    assert_eq!(que.size(), 0);
    assert_eq!(que.full(), false);

    que.push_front(1);
    que.push_front(2);
    que.push_front(3);
    assert_eq!(que.empty(), false);
    assert_eq!(que.size(), 3);
    assert_eq!(que.full(), false);
    assert_eq!(que.front(), 3);
    assert_eq!(que.back(), 1);

    que.push_front(4);
    assert_eq!(que.empty(), false);
    assert_eq!(que.size(), 4);
    assert_eq!(que.full(), false);
    assert_eq!(que.front(), 4);
    assert_eq!(que.back(), 1);

    que.push_back(5);
    que.push_back(6);
    que.push_back(7);
    assert_eq!(que.empty(), false);
    assert_eq!(que.size(), 7);
    assert_eq!(que.full(), true);
    assert_eq!(que.front(), 4);
    assert_eq!(que.back(), 7);
}

fn _test_pop() {
    let mut que = Deque::new();
    que.grow(8);
    assert_eq!(que.capacity, 8);
    assert_eq!(que.empty(), true);
    assert_eq!(que.size(), 0);
    assert_eq!(que.full(), false);

    que.push_front(1);
    que.push_front(2);
    que.push_front(3);
    assert_eq!(que.empty(), false);
    assert_eq!(que.size(), 3);
    assert_eq!(que.full(), false);
    assert_eq!(que.front(), 3);
    assert_eq!(que.back(), 1);

    que.push_front(4);
    assert_eq!(que.empty(), false);
    assert_eq!(que.size(), 4);
    assert_eq!(que.full(), false);
    assert_eq!(que.front(), 4);
    assert_eq!(que.back(), 1);

    que.push_back(5);
    que.push_back(6);
    que.push_back(7);
    println!("{:?}", que);

    assert_eq!(que.pop_front(), 4);
    assert_eq!(que.pop_front(), 3);
    assert_eq!(que.pop_front(), 2);
    assert_eq!(que.pop_front(), 1);

    assert_eq!(que.pop_back(), 7);
    assert_eq!(que.pop_back(), 6);
    assert_eq!(que.pop_back(), 5);

    assert_eq!(que.pop_back(), -1);
    assert_eq!(que.pop_front(), -1);
}
fn main() {
    //_test_push();
    //_test_pop();
    //return;
    let mut que = Deque::new();
    que.grow(100000);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();

    for _ in 0..num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let vs: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(str::to_string)
            .collect();
        match vs[0].as_str() {
            "push_back" => {
                let v: i32 = vs[1].parse().unwrap();
                que.push_back(v);
            }
            "push_front" => {
                let v: i32 = vs[1].parse().unwrap();
                que.push_front(v);
            }
            "front" => {
                println!("{}", que.front());
            }
            "back" => {
                println!("{}", que.back());
            }
            "empty" => {
                if que.empty() == true {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            "pop_front" => {
                println!("{}", que.pop_front());
            }
            "pop_back" => {
                println!("{}", que.pop_back());
            }
            "size" => {
                println!("{}", que.size());
            }
            _ => {
                panic!("Unknown cmd");
            }
        }
    }
}


```
