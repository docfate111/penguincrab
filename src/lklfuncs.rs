
extern "C" {
    pub fn lkl_sys_open(
        file: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_creat(
        file: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_access(
        file: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_chown(
        path: *const ::std::os::raw::c_char,
        uid: ::std::os::raw::c_uint,
        gid: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_chmod(
        path: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_link(
        existing: *const ::std::os::raw::c_char,
        new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_unlink(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_symlink(
        existing: *const ::std::os::raw::c_char,
        new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_readlink(
        path: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        bufsize: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_rename(
        old: *const ::std::os::raw::c_char,
        new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_mkdir(
        path: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_rmdir(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_mknod(
        path: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_uint,
        dev: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_pipe(fd: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_send(
        fd: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lkl_sys_recv(
        fd: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_uint,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
