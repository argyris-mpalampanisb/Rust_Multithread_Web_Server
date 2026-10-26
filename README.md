# Rust Multithread Web Server

A tiny HTTP server built from scratch in Rust. It spawns a thread pool and
serves static files — no frameworks, no dependencies, just the standard library.

## What it does

- Binds to `127.0.0.1:7878`
- Accepts incoming TCP connections
- Hands each connection off to a worker thread (pool size: 4)
- Serves `hello.xhtml` for `GET /`, `404.xhtml` for everything else
- Shuts down gracefully when the pool is dropped

## Project structure
 

multithread_web_server/
├── Cargo.toml
├── src/
│ ├── lib.rs # ThreadPool + Worker
│ └── main.rs # TCP listener + connection handler
├── hello.xhtml
└── 404.xhtml


## Running it

## bash
## cargo run
 

## Then in another terminal:

## curl http://127.0.0.1:7878/
 

You should get back the contents of hello.xhtml.
How the thread pool works

ThreadPool creates N worker threads that share a single mpsc::Receiver
wrapped in an Arc<Mutex<…>>. Each worker loops on recv(), executes the
job, and repeats. When the ThreadPool is dropped, the sender is taken
out and dropped, which closes the channel — workers see a recv() error,
print a goodbye message, and exit.
No external dependencies

Everything here is std. The whole point was to learn how the pieces fit
together without a framework hiding the details.
