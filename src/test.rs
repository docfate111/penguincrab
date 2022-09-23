#[cfg(test)]
mod tests {
    use more_asserts as ma;
    use penguincrab::*;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;
    #[test]
    fn read_write_close() {
        let filename = "./ext4-00.image";
        let file = match File::options().read(true).write(true).open(filename) {
            Err(e) => {
                panic!("Error opening {:}", e);
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

        const BUF_LEN: usize = 26;
        const MSG: &str = "that's what i call riddim\0";
        // remove null byte at the end
        let mut mpoint = server.mount_point.clone();
        mpoint.push_str("/test591");
        let mut r = lkl_sys_open(&mpoint, LKL_O_RDWR | LKL_O_CREAT, 0);
        ma::assert_ge!(r, 0);
        if r < 0 {
            print_error(&(r as i32));
        }
        let fd = r as i32;
        let buf = MSG.as_bytes();
        r = lkl_sys_write(fd, buf, BUF_LEN);
        assert_eq!(r as usize, BUF_LEN);
        r = lkl_sys_close(fd);
        assert_eq!(r, 0);
        if r < 0 {
            print_error(&(r as i32));
        }
        let mut read_buf = [0 as u8; BUF_LEN];
        let readfd = lkl_sys_open(&mpoint, LKL_O_RDONLY, 0) as i32;
        ma::assert_ge!(r, 0);
        r = lkl_sys_read(readfd, &mut read_buf, BUF_LEN);
        assert_eq!(r as usize, BUF_LEN);
        assert_eq!(MSG, String::from_utf8(read_buf.to_vec()).unwrap());
    }
}
