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
        disk_id = lkl_disk_add(&mut disk) as i32;
        lkl_start_kernel(&lkl_host_ops, boot_arg.as_ptr().cast());
    }
    if disk_id < 1 {
        eprintln!("Error adding disk:");
        match strerror(&disk_id) {
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

    unsafe {
        lkl_sys_halt();
        lkl_disk_remove(disk);
    }
    exit(0);
}
