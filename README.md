# penguincrab

WIP: unsafe wrapper for [Linux Kernel Library ](https://github.com/lkl/linux.git) in Rust

a binary at the moment but later I will rewrite it as a library

This command will take a while to compile:
``` 
$ cargo build && cargo run
```


File naming under lklh

lkl_specific.rs - defines general structs like lkl_host_ops and functions from the C library
syscall_nos.rs - system call numbers. If you don't see the one you want compile the kernel with the settings you want and then use bindgen to get the value.
consts.rs - constants like LKL_O_RDONLY and other flags
rests.rs - all the other constants


later for fuzzing
```
$ make LLVM=1 LLVM_IAS=1 CC=/path/to/AFLplusplus/afl-clang-fast ARCH=lkl -C tools/lkl
```
