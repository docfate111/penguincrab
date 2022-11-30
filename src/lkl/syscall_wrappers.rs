pub use crate::lkl::lklh::consts::*;
pub use crate::lkl::lklh::lkl_specific::*;
pub use crate::lkl::lklh::rest::*;
pub use crate::lkl::lklh::syscall_nos::*;
use core::str::Utf8Error;
pub use std::ffi::{CStr, CString};
pub use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};
pub use std::ptr;

// converts C char* to Rust String
pub fn from_cstr(some_str: *mut c_char) -> String {
    let cstr;
    unsafe {
        cstr = CStr::from_ptr(some_str);
    }
    String::from_utf8_lossy(cstr.to_bytes()).into_owned()
}

// converts Rust &str to Option<&CStr>
pub fn to_cstr<'a>(rust_str: &'a str) -> Option<&CStr> {
    if rust_str.matches("\0").count() > 1 {
        eprintln!(
            "String \"{}\" must not contain null bytes at the position not at the end",
            rust_str
        );
        return None;
    }
    let last = rust_str.chars().last();
    if last.is_none() {
        return None;
    }
    if last.unwrap() != '\0' {
        eprintln!("String \"{}\" must be null terminated (this function can't append null due to ownership rules)", rust_str);
        return None;
    }
    return Some(&CStr::from_bytes_with_nul(rust_str.as_bytes()).unwrap());
}

// converts lkl_strerror into a Rust string.
// returns a Result in case converting C String to Rust goes wrong
pub fn strerror<'a>(err: &i32) -> Result<&'a str, Utf8Error> {
    let char_ptr = unsafe { lkl_strerror(*err) };
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    c_str.to_str()
}

// prints out the error based on error from lkl_strerror
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

macro_rules! lkl_sys {
    (
        $id:expr => $vis:vis fn $func:ident(
            $( $arg:ident: $ty:ty $(=> $ex:expr)? ),+
        );
    ) => {
        $vis fn $func($($arg: $ty),+) -> c_long {
            let mut params = [0 as c_long; 6];
	    let used_params = [$(
                 $($ex)?($arg) as c_long
            ),+];
            params[..used_params.len()].copy_from_slice(&used_params);
            unsafe {
                lkl_syscall($id as i64, ptr::addr_of_mut!(params).cast::<c_long>())
            }
        }
    };
}

pub fn lkl_sys_open(file: &CStr, flags: u32, mode: u32) -> c_long {
    return lkl_sys_openat(LKL_AT_FDCWD, file, flags, mode);
}

lkl_sys! { __lkl__NR_openat =>
    pub fn lkl_sys_openat(dfd: i32,
    file: &CStr => |s: &CStr| s.as_ptr() ,
    flags: u32, mode: u32);
}

lkl_sys! { __lkl__NR_read =>
pub fn lkl_sys_read(fd: i32, buf: &mut [u8] => |s: &mut [u8] | s.as_mut_ptr(), count: usize); }

lkl_sys! {__lkl__NR_write =>
pub fn lkl_sys_write(fd: i32, buf: &[u8] => |s: &[u8]| s.as_ptr(), count: usize);}

lkl_sys! { __lkl__NR_close =>
    pub fn lkl_sys_close(fd: i32);
}

/*pub fn lkl_sys_stat(fd: i32, stat: &mut stat) -> {
for some reason there is no __lkl__NR_stat constant
}*/

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct stat {
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

lkl_sys! { __lkl__NR_fstat =>
pub fn lkl_sys_fstat(fd: i32, stat: &mut stat => |s| (s as *mut _)); }

// no NR_lstat syscall number either

lkl_sys! {
 __lkl__NR_lseek => pub fn lkl_sys_lseek(fd: i32, offset: u32, origin: u32);
}

lkl_sys! {
 __lkl__NR_mmap => pub fn lkl_sys_mmap(
    addr: u64, // &mut [u8] => |s: &mut [u8]| s.as_mut_ptr(),
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: u32);
}

lkl_sys! {
__lkl__NR_munmap =>
    pub fn lkl_sys_munmap(
    addr: &mut [u8] => |s: &mut [u8]| s.as_mut_ptr(),
    length: usize);
}

#[repr(C)]
#[derive(Debug)]
pub struct DirentName {
    pub name: [c_ulong; 40],
}

impl Default for DirentName {
    fn default() -> DirentName {
        DirentName {
            name: [0 as c_ulong; 40],
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct dirent64 {
    pub d_ino: c_ulong,
    pub d_off: c_ulong,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: DirentName,
}

lkl_sys! {  __lkl__NR_getdents64 =>
pub fn lkl_sys_getdents64(
fd: i32,
dirent: &mut dirent64 => |s| (s as *mut _),
count: usize); }

lkl_sys! {  __lkl__NR_pread64 =>
    pub fn lkl_sys_pread64(
        fd: i32,
        buf: &mut [u8] => |s: &mut [u8]| s.as_mut_ptr(),
        count: usize, off: u64);
}

lkl_sys! { __lkl__NR_pwrite64 =>
    pub fn lkl_sys_pwrite64(
        fd: i32,
        buf: &[u8] => |s: &[u8]| s.as_ptr(),
        count: usize, off: u64);
}

lkl_sys! { __lkl__NR_renameat =>
    pub fn lkl_sys_renameat(oldfd: i32,
    oldname: &CStr => |s: &CStr| s.as_ptr(),
    newfd: i32,
    newname: &CStr => |s: &CStr| s.as_ptr());
}

pub fn lkl_sys_rename(old: &CStr, new: &CStr) -> c_long {
    return lkl_sys_renameat(LKL_AT_FDCWD, old, LKL_AT_FDCWD, new);
}

lkl_sys! {
    __lkl__NR_fsync => pub fn lkl_sys_fsync(fd: i32);
}

lkl_sys! {
    __lkl__NR_fdatasync => pub fn lkl_sys_fdatasync(fd: i32);
}

lkl_sys! { __lkl__NR_syncfs => pub fn lkl_sys_syncfs(fd: i32); }

lkl_sys! {  __lkl__NR_sendfile =>
    pub fn lkl_sys_sendfile(out_fd: i32, in_fd: i32, offset: &mut [u8] => |s: &mut [u8]| s.as_mut_ptr(), count: usize);
}

pub fn lkl_sys_access(file: &CStr, mode: i32) -> c_long {
    return lkl_sys_faccessat(LKL_AT_FDCWD, file, mode);
}

lkl_sys! { __lkl__NR_faccessat =>
    pub fn lkl_sys_faccessat(dfd: i32,
    filename: &CStr => |s: &CStr| s.as_ptr(),
    mode: i32);
}

lkl_sys! {
     __lkl__NR_ftruncate => pub fn lkl_sys_ftruncate(fd: i32, length: c_ulong);
}

lkl_sys! {
 __lkl__NR_truncate => pub fn lkl_sys_truncate(
filename: &CStr => |s: &CStr| s.as_ptr(),
length: c_long); }
/*
redefine statfs without type aliases
pub fn lkl_sys_statfs(pathname: &str, buf: &mut statfs) -> c_long {
    let mut params = [0 as c_long; 6];
    let mut file = String::from(pathname);
    if pathname.chars().last().unwrap() != '\0' {
        file.push_str("\0");
    }
    params[0] = to_cstr(&file)
        .expect("lkl_sys_statfs got invalid pathname")
        .as_ptr() as c_long;
    params[1] = (buf as *mut _) as c_long;

    unsafe {
        return lkl_syscall(
            __lkl__NR_statfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }

}

pub fn lkl_sys_fstatfs(fd: i32, buf: &mut statfs) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = (buf as *mut _) as c_long;

    unsafe {
        return lkl_syscall(
            __lkl__NR_fstatfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }

}*/

/* syscall doesn't exist?
pub fn lkl_sys_utimes() -> c_long {
) -> c_long {
    let mut params = [0 as c_long; 6];

    unsafe {
        ret_val =
    lkl_syscall( __lkl__NR_utimes as c_long,
    ptr::addr_of_mut!(params).cast::<c_long>());
    }

}*/

lkl_sys! { __lkl__NR_mkdirat =>
    pub fn lkl_sys_mkdirat(dfd: i32,
        pathname: &CStr => |s: &CStr| s.as_ptr(),
        mode: u32);
}

pub fn lkl_sys_mkdir(path: &CStr, mode: u32) -> c_long {
    return lkl_sys_mkdirat(LKL_AT_FDCWD, path, mode);
}

pub fn lkl_sys_rmdir(path: &CStr) -> c_long {
    return lkl_sys_unlinkat(LKL_AT_FDCWD, path, LKL_AT_REMOVEDIR);
}

lkl_sys! { __lkl__NR_unlinkat =>
    pub fn lkl_sys_unlinkat(
            oldfd: i32,
            oldname: &CStr => |s: &CStr| s.as_ptr(),
            newfd: u32);
}

pub fn lkl_sys_link(existing: &CStr, new: &CStr) -> c_long {
    return lkl_sys_linkat(LKL_AT_FDCWD, existing, LKL_AT_FDCWD, new, 0);
}

lkl_sys! { __lkl__NR_linkat =>
    pub fn lkl_sys_linkat(
            oldfd: i32,
            oldname: &CStr => |s: &CStr| s.as_ptr(),
            newfd: i32,
            newname: &CStr => |s: &CStr| s.as_ptr(),
            flags: u32);
}

pub fn lkl_sys_unlink(existing: &CStr) -> c_long {
    return lkl_sys_unlinkat(LKL_AT_REMOVEDIR as i32, existing, 0);
}

pub fn lkl_sys_symlink(existing: &CStr, new: &CStr) -> c_long {
    return lkl_sys_symlinkat(existing, LKL_AT_FDCWD, new);
}

lkl_sys! { __lkl__NR_symlinkat  =>
    pub fn lkl_sys_symlinkat(
        oldname: &CStr => |s: &CStr| s.as_ptr(),
        newfd: i32,
        newname: &CStr => |s: &CStr| s.as_ptr());
}

pub fn lkl_sys_readlink(pathname: &CStr, buf: &mut [u8], bufsize: i32) -> c_long {
    return lkl_sys_readlinkat(LKL_AT_FDCWD, pathname, buf, bufsize);
}

lkl_sys! { __lkl__NR_readlinkat =>
   pub fn lkl_sys_readlinkat(dfd: i32,
   pathname: &CStr => |s: &CStr| s.as_ptr(),
   buf: &mut [u8] => |s: &mut [u8]| s.as_mut_ptr(),
   bufsize: i32);
}

pub fn lkl_sys_chmod(path: &CStr, mode: u32) -> c_long {
    return lkl_sys_fchmodat(LKL_AT_FDCWD, path, mode);
}

lkl_sys! { __lkl__NR_fchmodat =>
    pub fn lkl_sys_fchmodat(dirfd: i32,
    pathname: &CStr => |s: &CStr| s.as_ptr(),
     mode: u32);
}

lkl_sys! {  __lkl__NR_fchmod =>
pub fn lkl_sys_fchmod(fd: i32, mode: u32); }

pub fn lkl_sys_chown(path: &CStr, uid: u32, gid: u32) -> c_long {
    return lkl_sys_fchownat(LKL_AT_FDCWD, path, uid, gid, 0);
}

lkl_sys! {  __lkl__NR_fchownat =>
pub fn lkl_sys_fchownat(dfd: i32,
pathname: &CStr => |s: &CStr| s.as_ptr(),
uid: u32, gid: u32,
 flags: u32); }

lkl_sys! { __lkl__NR_fchown => pub fn lkl_sys_fchown(fd: i32, user: u32, group: u32); }

lkl_sys! { __lkl__NR_setxattr => pub fn lkl_sys_setxattr(
    pathname: &CStr => |s: &CStr| s.as_ptr(),
    strname: &CStr => |s: &CStr| s.as_ptr(),
    value: &CStr => |s: &CStr| s.as_ptr(),
    size: usize,
    flags: u32);
}

lkl_sys! { __lkl__NR_listxattr =>
    pub fn lkl_sys_listxattr(
    pathname: &CStr => |s: &CStr| s.as_ptr(),
    list: &mut [u8] => |s: &mut [u8]| s.as_ptr());
}

lkl_sys! {  __lkl__NR_llistxattr =>
    pub fn lkl_sys_llistxattr(
    pathname: &CStr => |s: &CStr| s.as_ptr(),
    list: &mut [u8] => |s: &mut [u8]| s.as_ptr(), size: usize);
}

lkl_sys! {
    __lkl__NR_removexattr =>
    pub fn lkl_sys_removexattr(
    pathname: &CStr => |s: &CStr| s.as_ptr(),
    removename: &CStr => |s: &CStr| s.as_ptr());
}

// copy and paste for __lkl__NR_lremovexattr and __lkl_NR_fremovexattr

lkl_sys! {
     __lkl__NR_getxattr =>
    pub fn lkl_sys_getxattr(
        pathname: &CStr => |s: &CStr| s.as_ptr(),
        pairname: &CStr => |s: &CStr| s.as_ptr(),
        value: &mut [u8] => |s: &mut [u8]| s.as_mut_ptr(),
    size: usize);
}

lkl_sys! {
     __lkl__NR_fallocate =>
pub fn lkl_sys_fallocate(fd: i64, mode: i64, offset: i64, len: i64);
}

lkl_sys! {
     __lkl__NR_getresuid =>
    pub fn lkl_sys_getresuid(
        ruid: &mut u32 => |s: &mut u32| s as *mut c_uint,
        euid: &mut u32 => |s: &mut u32| s as *mut c_uint,
        suid: &mut u32 => |s: &mut u32| s as *mut c_uint);
}

lkl_sys! {
     __lkl__NR_getresgid =>
    pub fn lkl_sys_getresgid(
        ruid: &mut u32 => |s: &mut u32| s as *mut c_uint,
        euid: &mut u32 => |s: &mut u32| s as *mut c_uint,
        suid: &mut u32 => |s: &mut u32| s as *mut c_uint);
}
