# penguincrab

unsafe wrapper for [Linux Kernel Library ](https://github.com/lkl/linux.git) in Rust


This command will take a while to compile since it also builds the kernel:
``` 
$ cargo build && cargo test -- --nocapture
```
if you have docker
```
$ sudo docker build --progress=plain . -f Dockerfile
```

File naming


lkl/
    syscall_wrappers - wrappers for syscalls and tests
    lklh/

	lkl_specific.rs - defines general structs like lkl_host_ops and functions from the C library
	syscall_nos.rs - system call numbers. If you don't see the one you want compile the kernel with the settings you want and then use bindgen to get the value.
	consts.rs - constants like LKL_O_RDONLY and other flags
	rests.rs - all the other constants

Examples of how to use it

[tests](https://github.com/docfate111/penguincrab/blob/main/src/lib.rs#L192)

To use penguincrab as a crate either copy the build.rs into the crate that uses it or compile liblkl.so and run your crate with LD_LIBARAY_PATH=(directory with liblkl.so)

Working on fixing Dockerfile no file error will put commands and then publish
