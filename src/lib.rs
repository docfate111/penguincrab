use core::str::Utf8Error;
pub use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};
pub mod lklh;

	
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
pub struct lkl_dir {
   pub fd: c_int,
   pub buf: [c_char; 1024usize],
   pub pos: *mut c_char,
   pub len: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_disk {
    pub dev: c_ulong,
    pub fd: c_int,
    pub ops: c_ulong,
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
         timeout_ms: c_long
    ) -> c_long;
    /**
    	* lkl_strerror - returns a string describing the given error code
    	*
    	* @err - error code
    	* @returns - string for the given error code
    	*const char *lkl_strerror(int err);
    	**/
    pub fn lkl_strerror(err: c_int) -> *const c_char;
    pub fn lkl_syscall(
	no: c_long,
	params: *mut c_long,
    ) -> c_long;

    
}

pub fn strerror<'a>(err: &i32) -> Result<&'a str, Utf8Error> {
    let char_ptr = unsafe { lkl_strerror(*err) };
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    c_str.to_str()
}
