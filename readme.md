
## Build your rust code (rustc)
### for linux or macbook
```
rustc main.rs -o output && ./output
```

### for windows
```
rustc main.rs -o output && .\output.exe
```


## Cargo
Cargo is Rust’s build system and package manager.

### create a new project
```
cargo new hello_cargo
```
### open the directory
```
cd hello_cargo
```

### Building and Running a Cargo Project
```
cargo build
```
This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory

You can run
```./target/debug/hello_cargo``` or ```.\target\debug\hello_cargo.exe``` on Windows

or you can also run your project using the comands
```
cargo run
```

Noted: Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

### add lib in Cargo
```
cargo add <lib_name>
```
