# Connect Four AI with Network and Concurrency Support

This project was developed as part of the "Systèmes Embarqués" course (Embedded Systems) at Télécom Paris. It is part of the concurrent programming laboratory assignments. The goal is to implement a networked version of the game Connect Four, supported by concurrent and asynchronous computation mechanisms to evaluate game strategies.

## Overview

The project consists of a Connect Four game engine played over a network between a host and a client. The game integrates AI evaluation modules with multiple concurrency techniques (threads, thread pools, asynchronous tasks) to simulate realistic game behavior and efficient computation. It is organized in multiple steps, each targeting a specific concept of concurrent programming in Rust.

## How to Build

Make sure you have Rust installed. Then, in the root directory:

```bash
cargo build --release
```

## How to Play

The game supports two binaries: `robot` and `async_robot`. One acts as host, and the other as client.

### Host vs Client (synchronous robot with threads)

Open two terminals and run:

**Terminal 1 – Host:**

```bash
cargo run --release --bin robot -- host 127.0.0.1:4444 -r -t 7
```

**Terminal 2 – Client:**

```bash
cargo run --release --bin robot -- client 127.0.0.1:4444 -r -t 7
```

### Async Version (uses async tasks and blocking futures)

**Terminal 1 – Host:**

```bash
cargo run --release --bin async_robot -- host 127.0.0.1:4444 -r -a 7
```

**Terminal 2 – Client:**

```bash
cargo run --release --bin async_robot -- client 127.0.0.1:4444 -r -a 7
```

### Options

- `host` / `client` – specify the role in the game
- `127.0.0.1:4444` – IP and port for communication
- `-r` / `--render` – render the board after each move
- `-t <depth>` – use synchronous threaded evaluation with depth
- `-a <depth>` – use asynchronous evaluation with depth
- `--alive` – print heartbeat message every 2 seconds to show the program is responsive

---

## Project Structure and Implementations

### Part I – Threaded Game Evaluation

We implemented a `SyncEvaluator` trait and a `MinMaxPolicy` AI that evaluates board states using a depth-limited minimax algorithm. It uses blocking threads (`std::thread`) to evaluate all legal moves in parallel.

### Part II – Condition Variables

A custom bounded blocking queue was implemented using `Mutex` and `Condvar`, supporting multiple producers and consumers for safe concurrent access. This is used to simulate typical concurrency control patterns.

### Part III – Async Evaluation with Blocking Futures

We introduced asynchronous evaluation using Rust's `Future` system. To handle blocking computations, we implemented:

- A custom `BlockingFuture` that creates a background thread and wakes the future upon completion.
- An `AsyncEvaluator` trait to generalize evaluation in async contexts.
- A wrapper `BlockingTaskWrapper` that makes any `SyncEvaluator` compatible with async tasks using `spawn_blocking` or our `BlockingFuture`.

### Part III.2 – Thread-Safe Cache

We implemented a concurrent cache called `KnowledgeCacheMultiThread`, which stores previously evaluated board positions. It uses `parking_lot::RwLock` for concurrent read/write access to a shared BTreeMap.

### Part III.3 – Async Robot with Heartbeat

We created the binary `async_robot` to test and validate asynchronous evaluation with a heartbeat mechanism (`--alive`) using `tokio::spawn`.

### Part IV – ThreadPool (Optional)

As a final enhancement, we created a `ThreadPool` structure that maintains a fixed number of worker threads. It implements `ThreadPool::execute` to reuse threads instead of spawning new ones, reducing overhead and better matching real-world frameworks.

---

## Final Notes

This project showcases multiple patterns of concurrent programming in Rust including:

- Thread spawning and joining
- Condition variables
- Asynchronous evaluation and futures
- Thread-safe shared state
- Fixed-size thread pools

It simulates real-world concurrent evaluation strategies and offers a playable and testable Connect Four AI engine over TCP/IP.
