# penguincrab

WIP: unsafe wrapper for linux kernel library in Rust

<<<<<<< HEAD
compile [LKL](https://github.com/lkl/linux.git) and put the .so in this directory
=======
compile (LKL)[https://github.com/lkl/linux.git] and put the .so in this directory
>>>>>>> 91e7a99f609c1e56bf6503e3dc4aba06a62e1ed5
``` 
make LLVM=1 LLVM_IAS=1 CC=/path/to/AFLplusplus/afl-clang-fast ARCH=lkl -C tools/lkl
```
