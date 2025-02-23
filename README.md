# Project 1 3502 Operating Systems

This repo covers the following topics:
- Threads
- Resource Protection
- Deadlock Management
- Inter-Process Communication (IPC)

The software is written in [Rust](https://www.rust-lang.org/) (No not the game :grin:) and is built in multiple phases. Each phase is stored in the `src/bin` folder and can be run using **cargo** rust's packmanager. 
```
cargo run --bin <bin-file>
```

## Runnable Binaries

The bin files that can be run are as follows:
- mt_phase1
- mt_phase2
- mt_phase3
- mt_phase4
- ipc
- main (This is the integration program)

## Testing

The project tests can be run with **cargo**. This will run all project unit tests and integration tests.

```
cargo test
```