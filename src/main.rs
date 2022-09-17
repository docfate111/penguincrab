use penguincrab::*;
use std::env::args;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn main() {
    let filename = match args().nth(1) {
        None => {
            eprintln!("Usage: pass in filesystem image as an argument");
            exit(1);
        }
        Some(k) => k,
    };
    let file = match File::open(filename) {
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
    if disk_id < 0 {
        eprintln!("Error adding disk:");
        match ustrerror(&disk_id) {
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
    let fs_type = CStr::from_bytes_with_nul(b"ext4\0").unwrap();
    let mount_options = CStr::from_bytes_with_nul(b"errors=remount-ro\0").unwrap();
    let msize = 100;
    let cv: Vec<u8> = CString::new("").unwrap().into_bytes_with_nul();
    let mut tmp: Vec<c_char> = cv.into_iter().map(|c| c as c_char).collect::<_>();
    let mpoint: *mut c_char = tmp.as_mut_ptr();
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
        match strerror(&ret) {
            Ok(k) => {
                eprintln!("{:}", k);
            }
            Err(_) => {
                eprintln!("unparseable string");
            }
        }
        unsafe {
            lkl_sys_halt();
        }
        exit(1);
    }
    //println!("mounted at {:}", *mpoint);
    unsafe {
        lkl_sys_halt();
        lkl_disk_remove(disk);
        lkl_umount_dev(disk_id, partition, 0, 100);
    }
    exit(0);
}
