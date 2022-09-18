# penguincrab

WIP: unsafe wrapper for [Linux Kernel Library ](https://github.com/lkl/linux.git) in Rust

a binary at the moment but later I will rewrite it as a library

This command will take a while to compile:
``` 
$ cargo build && cargo run
```
later for fuzzing
```
$ make LLVM=1 LLVM_IAS=1 CC=/path/to/AFLplusplus/afl-clang-fast ARCH=lkl -C tools/lkl
```
