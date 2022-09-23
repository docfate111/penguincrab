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
    let file = match File::options().read(true).write(true).open(filename) {
        Err(e) => {
            eprintln!("Error opening {:}", e);
            exit(1);
        }
        Ok(k) => k,
    };
    let server = LklSetup::new(LklSetupArgs {
        filesystem_fd: file.as_raw_fd(),
        boot_settings: None,
        partition_num: None,
        filesystem_type: None,
        filesystem_options: None,
    })
    .unwrap();
    // remove null byte at the end
    let mut mpoint = server.mount_point.clone();
    mpoint.push_str("/test591");
    let mut r = lkl_sys_open(&mpoint, LKL_O_RDWR | LKL_O_CREAT, 0);
    if r < 0 {
        print_error(&(r as i32));
    }
    let fd = r as i32;
    let buf = "that's what i call riddim\0".as_bytes();
    r = lkl_sys_write(fd, buf, buf.len());
    lkl_sys_close(fd);
    println!("wrote {} bytes", r);
    if r < 0 {
        print_error(&(r as i32));
    }
    const BUF_LEN: usize = 26;
    let mut read_buf = [0 as u8; BUF_LEN];
    let readfd = lkl_sys_open(&mpoint, LKL_O_RDONLY, 0) as i32;
    r = lkl_sys_read(readfd, &mut read_buf, BUF_LEN);
    println!("{} {:?}", r, String::from_utf8(read_buf.to_vec()).unwrap());
    exit(0);
}
