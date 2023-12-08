# shellcode-runner-rust

Simple Shellcode Runner in Rust Language

Downloads a binary file from a specified URL, stores the data in a byte array, allocates memory for that data, copies the data to the allocated memory, and then creates a thread to execute the code at the location where the data has been copied.

Using D\Invoke Technique + Donut.exe in Mimikatz.exe and Python server http to download using wrequest

### Dependeces used:

winapi = { version = "0.3", features = ["memoryapi", "processthreadsapi", "synchapi", "winnt"] }
reqwest = "0.11"
tokio = { version = "1", features = ["full"] }

It provides a runtime and functions that allow the use of asynchronous I/O

### Commands to compile:

Access Folder > Execute command 

cargo build
cargo run 


Reference:
https://github.com/cdong1012/Crab-Runner
