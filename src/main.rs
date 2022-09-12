/* automatically generated by rust-bindgen 0.60.1 */
use std::os::raw::{c_ulong, c_char, c_int};
use std::ffi::CStr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_host_operations {
    pub virtio_devices: c_ulong,
    pub print: ::std::option::Option<
        unsafe extern "C" fn(str_: *const c_char, len: c_int),
    >,
    pub panic: ::std::option::Option<unsafe extern "C" fn()>,
    pub func_ptrs: [c_ulong; 32usize],
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

    static lkl_host_ops: lkl_host_operations;

    pub fn lkl_start_kernel(
        lkl_ops: &lkl_host_operations,
    	 cmd: *const i8
	) -> c_int;

    pub fn lkl_is_running() -> ::std::os::raw::c_int;

    pub fn lkl_sys_halt() -> ::std::os::raw::c_long;
}

fn main() {
	let boot_arg = CStr::from_bytes_with_nul(b"mem=128M loglevel=8\0").unwrap();
	unsafe { 
		lkl_start_kernel(&lkl_host_ops, boot_arg.as_ptr().cast()); 
		lkl_sys_halt();
	} 
	println!("hello world");
}
<<<<<<< HEAD
=======

>>>>>>> 91e7a99f609c1e56bf6503e3dc4aba06a62e1ed5
