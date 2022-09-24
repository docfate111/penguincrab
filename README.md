# penguincrab

unsafe wrapper for [Linux Kernel Library ](https://github.com/lkl/linux.git) in Rust


This command will take a while to compile since it also builds the kernel:
``` 
$ cargo build && cargo test
```


File naming


lkl/
    syscall_wrappers - wrappers for syscalls and tests
    lklh/

	lkl_specific.rs - defines general structs like lkl_host_ops and functions from the C library
	syscall_nos.rs - system call numbers. If you don't see the one you want compile the kernel with the settings you want and then use bindgen to get the value.
	consts.rs - constants like LKL_O_RDONLY and other flags
	rests.rs - all the other constants

