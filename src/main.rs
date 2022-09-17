use penguincrab::*;
use std::env::args;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::exit;
use std::ptr;

fn main() {
    let filename = match args().nth(1) {
        None => {
            eprintln!("Usage: pass in filesystem image as an argument");
            exit(1);
        }
        Some(k) => k,
    };
    let file = match File::options().read(true).write(true).open(filename) {
        Err(e) => {
            eprintln!("Error opening {:}", e);
            exit(1);
        }
        Ok(k) => k,
    };
    let mut disk = lkl_disk {
        dev: 0,
        fd: file.as_raw_fd(),
        ops: 0,
    };
    let boot_arg = CStr::from_bytes_with_nul(b"mem=128M loglevel=8\0").unwrap();
    let disk_id;
    unsafe {
        disk_id = lkl_disk_add(&mut disk) as u32;
        lkl_start_kernel(&lkl_host_ops, boot_arg.as_ptr().cast());
    }
    if (disk_id as i32) < 0 {
        eprintln!("Error adding disk:");
        match strerror(&(disk_id as i32)) {
            Ok(k) => {
                eprintln!("{:}", k);
            }
            Err(_) => {
                eprintln!("Unparseable error string");
            }
        }
        unsafe {
            lkl_sys_halt();
        }
        exit(1);
    }
    let partition = 0;
    let fs_type = to_cstr("ext4\0");
    let mount_options = to_cstr("errors=remount-ro\0");
    let msize = 100;
    let mut mpoint: *mut c_char = to_mut_cstr("");
    let ret;
    unsafe {
        ret = lkl_mount_dev(
            disk_id,
            partition,
            fs_type.as_ptr().cast(),
            0,
            mount_options.as_ptr().cast(),
            mpoint,
            msize,
        ) as i32;
    }
    if ret < 0 {
	 eprintln!("Error lkl_mount_dev:"); 
	print_error(&ret);
	unsafe { lkl_sys_halt(); }
	exit(1);
    }
    let mount_point = from_cstr(mpoint);
    println!("mounted at {:?}", mount_point);
    let mut params = [ptr::null::<c_ulong>(); 5];
    let dir = mount_point.as_ptr().cast::<c_ulong>();
    params[0] = dir;
    let r;
    unsafe {
        r = lkl_syscall(__lkl__NR_chdir as i64, ptr::addr_of_mut!(params).cast::<c_long>());
    }
    println!("chdir {:}", r);
    print_error(&(r as i32));
    unsafe {
        let r = lkl_umount_dev(disk_id, partition, 0, 1000) as i32;
        if r < 0 {
            print_error(&r);
        }
        lkl_disk_remove(disk);
        lkl_sys_halt();
    }
    exit(0);
}
