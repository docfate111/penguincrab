use core::str::Utf8Error;
pub use std::ffi::{CStr, CString};
pub use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};
pub mod lklh;
pub use lklh::lklh::*;
pub use lklh::*;
pub use std::ptr;
/**lkl_host_operations - host operations used by the Linux kernel
 *
 * These operations must be provided by a host library or by the application
 * itself.
 *
 * @virtio_devices - string containg the list of virtio devices in virtio mmio
 * command line format. This string is appended to the kernel command line and
 * is provided here for convenience to be implemented by the host library.
 *
 * @print - optional operation that receives console messages
 *
 * @panic - called during a kernel panic
 *
 * @sem_alloc - allocate a host semaphore an initialize it to count
 * @sem_free - free a host semaphore
 * @sem_up - perform an up operation on the semaphore
 * @sem_down - perform a down operation on the semaphore
 *
 * @mutex_alloc - allocate and initialize a host mutex; the recursive parameter
 * determines if the mutex is recursive or not
 * @mutex_free - free a host mutex
 * @mutex_lock - acquire the mutex
 * @mutex_unlock - release the mutex
 *
 * @thread_create - create a new thread and run f(arg) in its context; returns a
 * thread handle or 0 if the thread could not be created
 * @thread_detach - on POSIX systems, free up resources held by
 * pthreads. Noop on Win32.
 * @thread_exit - terminates the current thread
 * @thread_join - wait for the given thread to terminate. Returns 0
 * for success, -1 otherwise
 *
 * @tls_alloc - allocate a thread local storage key; returns 0 if successful; if
 * destructor is not NULL it will be called when a thread terminates with its
 * argument set to the current thread local storage value
 * @tls_free - frees a thread local storage key; returns 0 if succesful
 * @tls_set - associate data to the thread local storage key; returns 0 if
 * successful
 * @tls_get - return data associated with the thread local storage key or NULL
 * on error
 *
 * @mem_alloc - allocate memory
 * @mem_free - free memory
 * @page_alloc - allocate page aligned memory
 * @page_free - free memory allocated by page_alloc
 *
 * @timer_create - allocate a host timer that runs fn(arg) when the timer
* fires.
 * @timer_free - disarms and free the timer
 * @timer_set_oneshot - arm the timer to fire once, after delta ns.
* @timer_set_periodic - arm the timer to fire periodically, with a period of
 * delta ns.
 *
 * @ioremap - searches for an I/O memory region identified by addr and size and
 * returns a pointer to the start of the address range that can be used by
 * iomem_access
 * @iomem_acess - reads or writes to and I/O memory region; addr must be in the
 * range returned by ioremap
 *
 * @gettid - returns the host thread id of the caller, which need not
 * be the same as the handle returned by thread_create
 *
 * @jmp_buf_set - runs the give function and setups a jump back point by saving
 * the context in the jump buffer; jmp_buf_longjmp can be called from the give
 * function or any callee in that function to return back to the jump back
 * point
 *
 * NOTE: we can't return from jmp_buf_set before calling jmp_buf_longjmp or
 * otherwise the saved context (stack) is not going to be valid, so we must pass
 * the function that will eventually call longjmp here
 *
 * @jmp_buf_longjmp - perform a jump back to the saved jump buffer
 *
 * @memcpy - copy memory
 * @pci_ops - pointer to PCI host operations
struct lkl_host_operations {
        const char *virtio_devices;

        void (*print)(const char *str, int len);
        void (*panic)(void);

        struct lkl_sem* (*sem_alloc)(int count);
        void (*sem_free)(struct lkl_sem *sem);
        void (*sem_up)(struct lkl_sem *sem);
        void (*sem_down)(struct lkl_sem *sem);

        struct lkl_mutex *(*mutex_alloc)(int recursive);
        void (*mutex_free)(struct lkl_mutex *mutex);
        void (*mutex_lock)(struct lkl_mutex *mutex);
        void (*mutex_unlock)(struct lkl_mutex *mutex);

        lkl_thread_t (*thread_create)(void (*f)(void *), void *arg);
        void (*thread_detach)(void);
        void (*thread_exit)(void);
        int (*thread_join)(lkl_thread_t tid);
        lkl_thread_t (*thread_self)(void);
        int (*thread_equal)(lkl_thread_t a, lkl_thread_t b);

        struct lkl_tls_key *(*tls_alloc)(void (*destructor)(void *));
        void (*tls_free)(struct lkl_tls_key *key);
        int (*tls_set)(struct lkl_tls_key *key, void *data);
        void *(*tls_get)(struct lkl_tls_key *key);
           void* (*mem_alloc)(unsigned long);
        void (*mem_free)(void *);
        void* (*page_alloc)(unsigned long size);
        void (*page_free)(void *addr, unsigned long size);

        unsigned long long (*time)(void);

        void* (*timer_alloc)(void (*fn)(void *), void *arg);
        int (*timer_set_oneshot)(void *timer, unsigned long delta);
        void (*timer_free)(void *timer);

        void* (*ioremap)(long addr, int size);
        int (*iomem_access)(const __volatile__ void *addr, void *val, int size,
                            int write);

        long (*gettid)(void);

        void (*jmp_buf_set)(struct lkl_jmp_buf *jmpb, void (*f)(void));
        void (*jmp_buf_longjmp)(struct lkl_jmp_buf *jmpb, int val);

        void* (*memcpy)(void *dest, const void *src, unsigned long count);
        struct lkl_dev_pci_ops *pci_ops;
};*/

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_host_operations {
    pub virtio_devices: c_ulong,
    pub print: ::std::option::Option<unsafe extern "C" fn(str_: *const c_char, len: c_int)>,
    pub panic: ::std::option::Option<unsafe extern "C" fn()>,
    pub func_ptrs: [c_ulong; 32usize],
}

/**
 * lkl_disk - host disk handle
 *
 * @dev - a pointer to 'virtio_blk_dev' structure for this disk
 * @fd - a POSIX file descriptor that can be used by preadv/pwritev
 * @handle - an NT file handle that can be used by ReadFile/WriteFile
 *struct lkl_disk {
        void *dev;
        union {
                int fd;
                void *handle;
        };
        struct lkl_dev_blk_ops *ops;
};**/

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_disk {
    pub dev: c_ulong,
    pub fd: c_int,
    pub ops: c_ulong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_dir {
    pub fd: c_int,
    pub buf: [c_char; 1024usize],
    pub pos: *mut c_char,
    pub len: c_int,
}

#[test]
fn bindgen_test_layout_lkl_disk() {
    assert_eq!(
        ::std::mem::size_of::<lkl_disk>(),
        24usize,
        concat!("Size of: ", stringify!(lkl_disk))
    );
    assert_eq!(
        ::std::mem::align_of::<lkl_disk>(),
        8usize,
        concat!("Alignment of ", stringify!(lkl_disk))
    );
    fn test_field_dev() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_disk>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).dev) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_disk),
                "::",
                stringify!(dev)
            )
        );
    }
    test_field_dev();
    fn test_field_fd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_disk>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fd) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_disk),
                "::",
                stringify!(fd)
            )
        );
    }
    test_field_fd();
    fn test_field_ops() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_disk>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ops) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_disk),
                "::",
                stringify!(ops)
            )
        );
    }
    test_field_ops();
}

#[test]
fn bindgen_test_layout_lkl_host_operations() {
    assert_eq!(
        ::std::mem::size_of::<lkl_host_operations>(),
        280usize,
        concat!("Size of: ", stringify!(lkl_host_operations))
    );
    assert_eq!(
        ::std::mem::align_of::<lkl_host_operations>(),
        8usize,
        concat!("Alignment of ", stringify!(lkl_host_operations))
    );
    fn test_field_virtio_devices() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_host_operations>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).virtio_devices) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_host_operations),
                "::",
                stringify!(virtio_devices)
            )
        );
    }
    test_field_virtio_devices();
    fn test_field_print() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_host_operations>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).print) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_host_operations),
                "::",
                stringify!(print)
            )
        );
    }
    test_field_print();
    fn test_field_panic() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_host_operations>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).panic) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_host_operations),
                "::",
                stringify!(panic)
            )
        );
    }
    test_field_panic();
    fn test_field_funcPtrs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lkl_host_operations>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).funcPtrs) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(lkl_host_operations),
                "::",
                stringify!(funcPtrs)
            )
        );
    }
    test_field_funcPtrs();
}
extern "C" {
    pub static lkl_host_ops: lkl_host_operations;
    /**
    * lkl_start_kernel - registers the host operations and starts the kernel
    *
    * The function returns only after the kernel is shutdown with lkl_sys_halt.
    *
    * @lkl_ops - pointer to host operations
    * @cmd_line - format for command line string that is going to be used to
    * generate the Linux kernel command line
    int lkl_start_kernel(struct lkl_host_operations *lkl_ops,
                const char *cmd_line, ...);
    */
    pub fn lkl_start_kernel(lkl_ops: &lkl_host_operations, cmd: *const i8) -> c_int;

    /**
    lkl_is_running - returns 1 if the kernel is currently running
    int lkl_is_running(void);**/
    pub fn lkl_is_running() -> c_int;

    // long lkl_sys_halt(void);
    pub fn lkl_sys_halt() -> c_long;

    /**
     * lkl_disk_add - add a new disk
     *
     * @disk - the host disk handle
     * @returns a disk id (0 is valid) or a strictly negative value in case of error
     *int lkl_disk_add(struct lkl_disk *disk);*/
    pub fn lkl_disk_add(disk: *mut lkl_disk) -> c_int;

    /**
     * lkl_disk_remove - remove a disk
     *
     * This function makes a cleanup of the @disk's virtio_dev structure
     * that was initialized by lkl_disk_add before.
     *
     * @disk - the host disk handle
     *int lkl_disk_remove(struct lkl_disk disk);
     */
    pub fn lkl_disk_remove(disk: lkl_disk) -> c_int;

    /**
     * lkl_mount_dev - mount a disk
     *
     * This functions creates a device file for the given disk, creates a mount
     * point and mounts the device over the mount point.
     *
     * @disk_id - the disk id identifying the disk to be mounted
     * @part - disk partition or zero for full disk
     * @fs_type - filesystem type
     * @flags - mount flags
     * @opts - additional filesystem specific mount options
     * @mnt_str - a string that will be filled by this function with the path where
     * the filesystem has been mounted
     * @mnt_str_len - size of mnt_str
     * @returns - 0 on success, a negative value on error
     */
    pub fn lkl_mount_dev(
        disk_id: c_uint,
        part: c_uint,
        fs_type: *const c_char,
        flags: c_int,
        opts: *const c_char,
        mnt_str: *mut c_char,
        mnt_str_len: c_uint,
    ) -> c_long;

    /**
     * lkl_umount_dev - umount a disk
     *
     * This functions umounts the given disks and removes the device file and the
     * mount point.
     * @disk_id - the disk id identifying the disk to be mounted
     * @part - disk partition or zero for full disk
     * @flags - umount flags
     * @timeout_ms - timeout to wait for the kernel to flush closed files so that
     * umount can succeed
     * @returns - 0 on success, a negative value on error
     */
    pub fn lkl_umount_dev(
        disk_id: c_uint,
        part: c_uint,
        flags: c_int,
        timeout_ms: c_long,
    ) -> c_long;
    /**
    	* lkl_strerror - returns a string describing the given error code
    	*
    	* @err - error code
    	* @returns - string for the given error code
    	*const char *lkl_strerror(int err);
    	**/
    pub fn lkl_strerror(err: c_int) -> *const c_char;
    pub fn lkl_syscall(no: c_long, params: *mut c_long) -> c_long;

}

pub fn from_cstr(some_str: *mut c_char) -> String {
    let cstr;
    unsafe {
        cstr = CStr::from_ptr(some_str);
    }
    String::from_utf8_lossy(cstr.to_bytes()).into_owned()
}

pub fn to_cstr<'a>(rust_str: &'a str) -> Option<&CStr> {
    if rust_str.matches("\0").count() > 1 {
        eprintln!(
            "String \"{}\" must not contain null bytes at the position not at the end",
            rust_str
        );
        return None;
    }
    if rust_str.chars().last().unwrap() != '\0' {
        eprintln!("String \"{}\" must be null terminated (this function can't append null due to ownership rules)", rust_str);
        return None;
    }
    return Some(&CStr::from_bytes_with_nul(rust_str.as_bytes()).unwrap());
}

pub fn strerror<'a>(err: &i32) -> Result<&'a str, Utf8Error> {
    let char_ptr = unsafe { lkl_strerror(*err) };
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    c_str.to_str()
}

pub fn print_error<'a>(err: &i32) {
    match strerror(err) {
        Ok(k) => {
            eprintln!("{:}", k);
        }
        Err(_) => {
            eprintln!("unparseable string");
        }
    }
}

pub fn lkl_sys_open<'a>(file: &'a str, flags: u32, mode: u32) -> c_long {
    let mut filename = String::from(file);
    if file.chars().last().unwrap() != '\0' {
        filename.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = LKL_AT_FDCWD as c_long;
    params[1] = to_cstr(&filename)
        .expect("lkl_sys_open failed to parse filename")
        .as_ptr() as c_long;
    params[2] = flags as c_long;
    params[3] = mode as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_openat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_read(fd: i32, buf: &mut Vec<u8>, count: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_mut_ptr() as c_long;
    params[2] = count as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_read as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_write(fd: i32, buf: &Vec<u8>, count: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_ptr() as c_long;
    params[2] = count as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_write as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_close(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_close as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

/*pub fn lkl_sys_stat(fd: i32, stat: &mut lkl_stat) -> {
for some reason there is no __lkl__NR_stat constant
}*/

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_mode: c_ulong,
    pub st_nlink: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub st_rdev: c_ulong,
    pub __pad1: c_ulong,
    pub st_size: c_ulong,
    pub st_blksize: c_int,
    pub __pad2: c_int,
    pub st_blocks: c_long,
    pub lkl_st_atime: c_long,
    pub st_atime_nsec: c_long,
    pub lkl_st_mtime: c_long,
    pub st_mtime_nsec: c_long,
    pub lkl_st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    pub __unused4: c_uint,
    pub __unused5: c_uint,
}

pub fn lkl_sys_fstat(fd: i32, stat: &mut lkl_stat) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = (stat as *mut _) as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fstat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

/* no NR_lstat
pub fn lkl_sys_lstat(file: u32, stat: &mut lkl_stat) -> c_long {
    let mut filename = String::from(file);
        if file.chars().last().unwrap() != '\0' {
                filename.push_str("\0");
        }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(&filename)
                .expect("lkl_sys_lstat failed to parse filename")
                .as_ptr() as c_long;
    params[1] = (stat as *mut _) as c_long;
    let ret_val;
    unsafe { ret_val = lkl_syscall(__lkl__NR_lstat as c_long,
    ptr::addr_of_mut!(params).cast::<c_long>()); }
    return ret_val;
}*/

pub fn lkl_sys_lseek(fd: i32, offset: u32, origin: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = offset as c_long;
    params[2] = origin as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_lseek as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_mmap(
    addr: u64,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: u32,
) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = addr as c_long;
    params[1] = length as c_long;
    params[2] = prot as c_long;
    params[3] = flags as c_long;
    params[4] = fd as c_long;
    params[5] = offset as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_mmap as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

#[repr(C)]
#[derive(Debug)]
pub struct lkl_linux_dirent64 {
    pub d_ino: c_ulong,
    pub d_off: c_ulong,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_ulong; 100],
}

pub fn lkl_sys_getdents64(fd: i32, dirent: &mut lkl_linux_dirent64, count: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = (dirent as *mut _) as c_long;
    params[2] = count as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_getdents64 as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_pread64(fd: i32, buf: &mut Vec<u8>, count: u32, off: u64) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_mut_ptr() as c_long;
    params[2] = count as c_long;
    params[3] = off as c_long;
    let mut params = [0 as c_long; 6];
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_pread64 as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_pwrite64(fd: i32, buf: &Vec<u8>, count: u32, off: u64) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_ptr() as c_long;
    params[2] = count as c_long;
    params[3] = off as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_pwrite64 as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_rename(old: &str, new: &str) -> c_long {
    return lkl_sys_renameat(LKL_AT_FDCWD, old, LKL_AT_FDCWD, new);
}

pub fn lkl_sys_renameat(oldfd: i32, oldname: &str, newfd: i32, newname: &str) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = oldfd as c_long;
    params[1] = to_cstr(oldname)
        .expect("lkl_sys_renameat invalid oldname")
        .as_ptr() as c_long;
    params[2] = newfd as c_long;
    params[3] = to_cstr(newname)
        .expect("lkl_sys_renameat invalid newname")
        .as_ptr() as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_renameat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_fsync(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fsync as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_fdatasync(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fdatasync as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_syncfs(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_syncfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_sendfile(out_fd: i32, in_fd: i32, offset: &mut Vec<u8>, count: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = out_fd as c_long;
    params[1] = in_fd as c_long;
    params[2] = offset.as_mut_ptr() as c_long;
    params[3] = count as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_sendfile as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_access(file: &str, mode: i32) -> c_long {
    return lkl_sys_faccessat(LKL_AT_FDCWD, file, mode);
}

pub fn lkl_sys_faccessat(dfd: i32, filename: &str, mode: i32) -> c_long {
    let mut file = String::from(filename);
    if filename.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dfd as c_long;
    params[1] = to_cstr(&file)
        .expect("lkl_sys_faccessat received invalid filename")
        .as_ptr() as c_long;
    params[2] = mode as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_faccessat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_ftruncate(fd: i32, length: c_ulong) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = length as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_ftruncate as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_truncate(filename: &str, length: c_long) -> c_long {
    let mut params = [0 as c_long; 6];
    let mut file = String::from(filename);
    if filename.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    params[0] = to_cstr(&file)
        .expect("lkl_sys_truncate received an invalid filename")
        .as_ptr() as c_long;
    params[1] = length as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_truncate as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_statfs {
    pub f_type: __lkl__kernel_long_t,
    pub f_bsize: __lkl__kernel_long_t,
    pub f_blocks: __lkl__kernel_long_t,
    pub f_bfree: __lkl__kernel_long_t,
    pub f_bavail: __lkl__kernel_long_t,
    pub f_files: __lkl__kernel_long_t,
    pub f_ffree: __lkl__kernel_long_t,
    pub f_fsid: __lkl__kernel_fsid_t,
    pub f_namelen: __lkl__kernel_long_t,
    pub f_frsize: __lkl__kernel_long_t,
    pub f_flags: __lkl__kernel_long_t,
    pub f_spare: [__lkl__kernel_long_t; 4usize],
}

pub fn lkl_sys_statfs(pathname: &str, buf: &mut lkl_statfs) -> c_long {
    let mut params = [0 as c_long; 6];
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    params[0] = to_cstr(&file)
        .expect("lkl_sys_statfs got invalid pathname")
        .as_ptr() as c_long;
    params[1] = (buf as *mut _) as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_statfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_fstatfs(fd: i32, buf: &mut lkl_statfs) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = (buf as *mut _) as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fstatfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

/* syscall doesn't exist?
pub fn lkl_sys_utimes() -> c_long {
) -> c_long {
    let mut params = [0 as c_long; 6];
    let ret_val;
    unsafe {
        ret_val =
    lkl_syscall( __lkl__NR_utimes as c_long,
    ptr::addr_of_mut!(params).cast::<c_long>());
    }
    return ret_val;
}*/

pub fn lkl_sys_mkdirat(dfd: i32, pathname: &str, mode: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dfd as c_long;
    params[1] = to_cstr(&file)
        .expect("lkl_sys_mkdirat received invalid pathname")
        .as_ptr() as c_long;
    params[2] = mode as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_mkdirat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_mkdir(path: &str, mode: u32) -> c_long {
    return lkl_sys_mkdirat(LKL_AT_FDCWD, path, mode);
}

pub fn lkl_sys_rmdir(path: &str) -> c_long {
    return lkl_sys_unlinkat(LKL_AT_FDCWD, path, LKL_AT_REMOVEDIR);
}

pub fn lkl_sys_link(existing: &str, new: &str) -> c_long {
    return lkl_sys_linkat(LKL_AT_FDCWD, existing, LKL_AT_FDCWD, new, 0);
}

pub fn lkl_sys_linkat(oldfd: i32, oldname: &str, newfd: i32, newname: &str, flags: u32) -> c_long {
    let mut oldfile = String::from(oldname);
    if oldname.chars().last().unwrap() != '\0' {
        oldfile.push_str("\0");
    }
    let mut newfile = String::from(newname);
    if newname.chars().last().unwrap() != '\0' {
        newfile.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = oldfd as c_long;
    params[1] = to_cstr(&oldfile)
        .expect("lkl_sys_linkat received an invalid oldname")
        .as_ptr() as c_long;
    params[2] = newfd as c_long;
    params[3] = to_cstr(&newfile)
        .expect("lkl_sys_linkat received an invalid newname")
        .as_ptr() as c_long;
    params[4] = flags as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_linkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_unlink(existing: &str, new: &str) -> c_long {
    return lkl_sys_linkat(LKL_AT_FDCWD, existing, LKL_AT_FDCWD, new, 0);
}

pub fn lkl_sys_unlinkat(dfd: i32, pathname: &str, flag: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dfd as c_long;
    params[1] = to_cstr(&file)
        .expect("lkl_sys_unlinkat received invalid pathname")
        .as_ptr() as c_long;
    params[2] = flag as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_unlinkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_symlink(existing: &str, new: &str) -> c_long {
    return lkl_sys_symlinkat(existing, LKL_AT_FDCWD, new);
}

pub fn lkl_sys_symlinkat(oldname: &str, newfd: i32, newname: &str) -> c_long {
    let mut oldfile = String::from(oldname);
    if oldname.chars().last().unwrap() != '\0' {
        oldfile.push_str("\0");
    }
    let mut newfile = String::from(newname);
    if newname.chars().last().unwrap() != '\0' {
        newfile.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(&oldfile)
        .expect("lkl_sys_symlinkat received invalid oldfile")
        .as_ptr() as c_long;
    params[1] = newfd as c_long;
    params[2] = to_cstr(&newfile)
        .expect("lkl_sys_symlinkat received invalid newfile")
        .as_ptr() as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_symlinkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_readlink(pathname: &str, buf: &mut Vec<u8>, bufsize: i32) -> c_long {
    return lkl_sys_readlinkat(LKL_AT_FDCWD, pathname, buf, bufsize);
}

pub fn lkl_sys_readlinkat(dfd: i32, pathname: &str, buf: &mut Vec<u8>, bufsize: i32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dfd as c_long;
    params[1] = to_cstr(&file)
        .expect("lkl_sys_readlinkat received invalid pathname")
        .as_ptr() as c_long;
    params[2] = buf.as_mut_ptr() as c_long;
    params[3] = bufsize as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_readlinkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_chmod(path: &str, mode: u32) -> c_long {
    return lkl_sys_fchmodat(LKL_AT_FDCWD, path, mode);
}

pub fn lkl_sys_fchmodat(dirfd: i32, pathname: &str, mode: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dirfd as c_long;
    params[1] = to_cstr(&file)
        .expect("lkl_sys_fchmodat received invalid pathname")
        .as_ptr() as c_long;
    params[2] = mode as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fchmodat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_fchmod(fd: i32, mode: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = mode as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fchmod as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_chown(path: &str, uid: u32, gid: u32) -> c_long {
    return lkl_sys_fchownat(LKL_AT_FDCWD, path, uid, gid, 0);
}

pub fn lkl_sys_fchownat(dfd: i32, pathname: &str, uid: u32, gid: u32, flags: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dfd as c_long;
    params[1] = to_cstr(&file)
        .expect("lkl_sys_fchownat received invalid pathname")
        .as_ptr() as c_long;
    params[2] = uid as c_long;
    params[3] = gid as c_long;
    params[4] = flags as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fchownat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_fchown(fd: i32, user: u32, group: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = user as c_long;
    params[2] = group as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fchown as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_setxattr(
    pathname: &str,
    strname: &str,
    value: &Vec<u8>,
    size: u32,
    flags: u32,
) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut name = String::from(strname);
    if strname.chars().last().unwrap() != '\0' {
        name.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(&file)
        .expect("lkl_sys_setxattr received invalid pathname")
        .as_ptr() as c_long;
    params[1] = to_cstr(&name)
        .expect("lkl_sys_setxattr received invalid name")
        .as_ptr() as c_long;
    params[2] = value.as_ptr() as c_long;
    params[3] = size as c_long;
    params[4] = flags as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_setxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_listxattr(pathname: &str, list: &mut Vec<u8>, size: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(&file)
        .expect("lkl_sys_listxattr received invalid name")
        .as_ptr() as c_long;
    params[1] = list.as_mut_ptr() as c_long;
    params[2] = size as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_listxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_llistxattr(pathname: &str, list: &mut Vec<u8>, size: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(&file)
        .expect("lkl_sys_llistxattr received invalid name")
        .as_ptr() as c_long;
    params[1] = list.as_mut_ptr() as c_long;
    params[2] = size as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_llistxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_removexattr(pathname: &str, removename: &str) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut remove = String::from(removename);
    if removename.chars().last().unwrap() != '\0' {
        remove.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(pathname)
        .expect("lkl_sys_removexattr received invalid pathname")
        .as_ptr() as c_long;
    params[1] = to_cstr(removename)
        .expect("lkl_sys_removexattr received invalid removename")
        .as_ptr() as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_removexattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}
// copy and paste for __lkl__NR_lremovexattr and __lkl_NR_fremovexattr

pub fn lkl_sys_getxattr(pathname: &str, pairname: &str, value: &mut Vec<u8>, size: u32) -> c_long {
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    let mut name = String::from(pairname);
    if pairname.chars().last().unwrap() != '\0' {
        name.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = to_cstr(&file)
        .expect("lkl_sys_getxattr received invalid pathname")
        .as_ptr() as c_long;
    params[1] = to_cstr(&name)
        .expect("lkl_sys_getxattr received pair name")
        .as_ptr() as c_long;
    params[2] = value.as_mut_ptr() as c_long;
    params[3] = size as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_getxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}

pub fn lkl_sys_fallocate(fd: i64, mode: i64, offset: i64, len: i64) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = mode as c_long;
    params[2] = offset as c_long;
    params[3] = len as c_long;
    let ret_val;
    unsafe {
        ret_val = lkl_syscall(
            __lkl__NR_fallocate as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    return ret_val;
}
