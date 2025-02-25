# Project 1 3502 Operating Systems

This repo covers the following topics:
- Threads
- Resource Protection
- Deadlock Management
- Inter-Process Communication (IPC)

The software is written in [Rust](https://www.rust-lang.org/) (No not the game :grin:) and is built in multiple phases. Each phase is stored in the `src/bin` folder and can be run using **cargo** rust's packmanager. The `main` binary program will run and integration program. This program should spawn mutliple customer processes that will put in account transfer requests. A separate Teller process will be spawned to accept the customer requests and perform the transaction. Each transaction should be handle in its own thread using thread safety to access all accounts in the proper order with no conflicts.

## Installation

To run this program you will need to do the following:
1. Install Rust and Cargo if not already installed. [Rust Installer](https://rustup.rs/)
2. Build the application with cargo
    ```bash
    cargo build
    ```
3. Run the rust binaries
    ```bash
    cargo run --bin <bin-file>
    ```

    **NOTE** All required dependencies will be handled by cargo's build step.

## Runnable Binaries

The bin files that can be run are as follows:
- mt_phase1
- mt_phase2
- mt_phase3
- mt_phase4
- ipc (This expects one of the assest filenames to be passed in as an argument)
- main (This is the integration program)

## Testing

The project tests can be run with **cargo**. This will run all project unit tests and integration tests.

```
cargo test
```