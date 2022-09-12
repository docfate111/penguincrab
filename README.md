# penguincrab

WIP: unsafe wrapper for linux kernel library in Rust

compile [LKL](https://github.com/lkl/linux.git) and put the .so in this directory
``` 
make LLVM=1 LLVM_IAS=1 CC=/path/to/AFLplusplus/afl-clang-fast ARCH=lkl -C tools/lkl
```
