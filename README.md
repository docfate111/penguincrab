# penguincrab

WIP: unsafe wrapper for linux kernel library in Rust

a binary at the moment but later I will rewrite it as a library

compile [LKL](https://github.com/lkl/linux.git) and put the .so in this directory:
``` 
git clone https://github.com/lkl/linux.git
cd linux
make ARCH=lkl -C tools/lkl
cd ..
cp linux/tools/lkl/lib/liblkl.so .
cp linux/tools/lkl/liblkl.a .
ar rcs liblkl.a liblkl.o
cargo build && cargo run
```
later for fuzzing
```
make LLVM=1 LLVM_IAS=1 CC=/path/to/AFLplusplus/afl-clang-fast ARCH=lkl -C tools/lkl
```
