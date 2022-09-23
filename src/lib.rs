use core::str::Utf8Error;
pub use std::ffi::{CStr, CString};
pub use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};
pub mod lklh;
pub use lklh::syscall_nos::*;
pub use lklh::rest::*;
pub use lklh::consts::*;
pub use lklh:: lkl_specific::*;
pub use std::ptr;

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

pub fn lkl_sys_open(file: &str, flags: u32, mode: u32) -> c_long {
    return lkl_sys_openat(LKL_AT_FDCWD, file, flags, mode);
}

pub fn lkl_sys_openat(dfd: i32, file: &str, flags: u32, mode: u32) -> c_long {
    let mut filename = String::from(file);
    if file.chars().last().unwrap() != '\0' {
        filename.push_str("\0");
    }
    let mut params = [0 as c_long; 6];
    params[0] = dfd as c_long;
    params[1] = to_cstr(&filename)
        .expect("lkl_sys_open failed to parse filename")
        .as_ptr() as c_long;
    params[2] = flags as c_long;
    params[3] = mode as c_long;
    unsafe {
        return lkl_syscall(
            __lkl__NR_openat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
}

pub fn lkl_sys_read(fd: i32, buf: &mut [u8], count: usize) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_mut_ptr() as c_long;
    params[2] = count as c_long;
    unsafe {
        return lkl_syscall(
            __lkl__NR_read as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
}

pub fn lkl_sys_write(fd: i32, buf: &[u8], count: usize) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_ptr() as c_long;
    params[2] = count as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_write as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_close(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_close as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fstat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe { return lkl_syscall(__lkl__NR_lstat as c_long,
    ptr::addr_of_mut!(params).cast::<c_long>()); }
    
}*/

pub fn lkl_sys_lseek(fd: i32, offset: u32, origin: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = offset as c_long;
    params[2] = origin as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_lseek as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_mmap as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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

pub fn lkl_sys_getdents64(fd: i32, dirent: &mut lkl_linux_dirent64, count: usize) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = (dirent as *mut _) as c_long;
    params[2] = count as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_getdents64 as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_pread64(fd: i32, buf: &mut [u8], count: usize, off: u64) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_mut_ptr() as c_long;
    params[2] = count as c_long;
    params[3] = off as c_long;
    let mut params = [0 as c_long; 6];
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_pread64 as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_pwrite64(fd: i32, buf: &[u8], count: usize, off: u64) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = buf.as_ptr() as c_long;
    params[2] = count as c_long;
    params[3] = off as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_pwrite64 as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_renameat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_fsync(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fsync as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_fdatasync(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fdatasync as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_syncfs(fd: i32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_syncfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_sendfile(out_fd: i32, in_fd: i32, offset: &mut [u8], count: usize) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = out_fd as c_long;
    params[1] = in_fd as c_long;
    params[2] = offset.as_mut_ptr() as c_long;
    params[3] = count as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_sendfile as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_faccessat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_ftruncate(fd: i32, length: c_ulong) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = length as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_ftruncate as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_truncate as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}


/*
redefine lkl_statfs without type aliases
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_statfs as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_fstatfs(fd: i32, buf: &mut lkl_statfs) -> c_long {
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_mkdirat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_linkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_unlinkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_symlinkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_readlink(pathname: &str, buf: &mut [u8], bufsize: i32) -> c_long {
    return lkl_sys_readlinkat(LKL_AT_FDCWD, pathname, buf, bufsize);
}

pub fn lkl_sys_readlinkat(dfd: i32, pathname: &str, buf: &mut [u8], bufsize: i32) -> c_long {
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_readlinkat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fchmodat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_fchmod(fd: i32, mode: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = mode as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fchmod as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fchownat as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_fchown(fd: i32, user: u32, group: u32) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = user as c_long;
    params[2] = group as c_long;
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_fchown as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_setxattr(
    pathname: &str,
    strname: &str,
    value: &[u8],
    size: usize,
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_setxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_listxattr(pathname: &str, list: &mut [u8], size: usize) -> c_long {
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_listxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_llistxattr(pathname: &str, list: &mut [u8], size: usize) -> c_long {
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_llistxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_removexattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}
// copy and paste for __lkl__NR_lremovexattr and __lkl_NR_fremovexattr

pub fn lkl_sys_getxattr(pathname: &str, pairname: &str, value: &mut [u8], size: usize) -> c_long {
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
    
    unsafe {
        return lkl_syscall(
            __lkl__NR_getxattr as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

pub fn lkl_sys_fallocate(fd: i64, mode: i64, offset: i64, len: i64) -> c_long {
    let mut params = [0 as c_long; 6];
    params[0] = fd as c_long;
    params[1] = mode as c_long;
    params[2] = offset as c_long;
    params[3] = len as c_long;
    unsafe {
        return lkl_syscall(
            __lkl__NR_fallocate as c_long,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }
    
}

