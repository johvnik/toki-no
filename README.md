# toki-no

[![Crates.io](https://img.shields.io/crates/v/toki-no.svg?style=flat-square)](https://crates.io/crates/toki-no)
[![Docs.rs](https://img.shields.io/docsrs/toki-no?style=flat-square)](https://docs.rs/toki-no)
[![Tests](https://github.com/johvnik/toki-no/actions/workflows/rust.yml/badge.svg)](https://github.com/johvnik/toki-no/actions/workflows/rust.yml)
[![License](https://img.shields.io/crates/l/toki-no.svg)](https://github.com/johvnik/toki-no/blob/main/LICENSE-MIT)

*時の... of time...*

**`toki-no` is a minimal and fast `async` runtime for Rust.**

## Overview

`toki-no` is an exploration into the fundamentals of asynchronous execution, built from the ground up in pure Rust. It provides a lean executor and I/O reactor, focusing on simplicity and performance.

The core goal is to provide a runtime that is easy to understand, has a minimal API, and maintains low overhead.

## Features

* **Minimal & Lightweight:** A small API surface and a lean codebase.
* **Fast & Efficient:** Designed for low-overhead asynchronous execution.
* **I/O Driven:** Powered by a modern event-polling mechanism.
* **Pure Rust:** Built with safe, modern Rust.

## Usage

Add `toki-no` to your `Cargo.toml`:

```rust
use std::time::Duration;

#[toki_no::main]
async fn main() {
    println!("Hello from an async world!");

    // Spawn a concurrent task
    let handle = toki_no::spawn(async {
        println!("Task is running...");
        toki_no::sleep(Duration::from_secs(1)).await;
        "Task finished!"
    });

    println!("Main function continues...");

    let result = handle.await;
    println!("Result from spawned task: {}", result);
}
```
