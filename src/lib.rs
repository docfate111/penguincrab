pub mod lkl;
pub use lkl::syscall_wrappers::*;

pub struct LklSetup {
    disk: lkl_disk,
    partition: u32,
    disk_id: u32,
    pub mount_point: String,
}

impl LklSetup {
    pub fn new(arg: LklSetupArgs) -> Result<LklSetup, &'static str> {
        let mut disk = lkl_disk {
            dev: 0,
            fd: arg.filesystem_fd,
            ops: 0,
        };
        let boot_arg = match arg.boot_settings {
            Some(k) => to_cstr(&k)
                .expect("boot_settings formats null bytes wrong")
                .as_ptr()
                .cast(),
            None => to_cstr("mem=128M loglevel=8\0").unwrap().as_ptr().cast(),
        };
        let disk_id;
        // fn a() {}
        // lkl_host_ops = a as *const fn() as c_ulong
        unsafe {
            disk_id = lkl_disk_add(&mut disk) as u32;
            lkl_start_kernel(&lkl_host_ops, boot_arg);
        }
        if (disk_id as i32) < 0 {
            eprintln!("Error adding disk:");
            let _reason = match strerror(&(disk_id as i32)) {
                Ok(k) => {
                    eprintln!("{:}", k);
                    k
                }
                Err(_) => {
                    eprintln!("Unparseable error string");
                    "unknown string"
                }
            };
            unsafe {
                lkl_sys_halt();
            }
            return Err("Couldn't add disk");
        }
        let partition = arg.partition_num.unwrap_or(0);
        let fs_type = &arg
            .filesystem_type
            .unwrap_or("ext4\0".to_string())
            .to_owned()[..];
        let default_options = match fs_type {
            "ext4" => "errors=remount-ro\0",
            "btrfs" => "thread_pool=1\0",
            "gfs2" => "acl\0",
            "reiserfs" => "acl,user_xattr\0",
            &_ => "\0",
        };
        let mount_options = arg
            .filesystem_options
            .unwrap_or(default_options.to_string());
        let msize: u32 = 100;
        let mut mpoint = vec![0u8; msize as usize];
        let ret;
        unsafe {
            ret = lkl_mount_dev(
                disk_id,
                partition,
                to_cstr(&fs_type)
                    .expect("filesystem has incorrect nulls")
                    .as_ptr()
                    .cast(),
                0,
                to_cstr(&mount_options)
                    .expect("mount options has incorrect nulls")
                    .as_ptr()
                    .cast(),
                mpoint.as_mut_ptr().cast(),
                msize,
            ) as i32;
        }
        if ret < 0 {
            eprintln!("Error lkl_mount_dev:");
            print_error(&ret);
            unsafe {
                lkl_sys_halt();
            }
            return Err("lkl_mount_dev failed");
        }
        let mount_point = String::from_utf8(mpoint).unwrap();
        println!("[*] Filesystem mounted at {:}", mount_point);
        let full = String::from(mount_point);
        // removing trailing null bytes except leave one
        let mount_point = &full[0..full.find("\0").unwrap_or(0) + 1];
        let mut params = [ptr::null::<c_ulong>(); 5];
        params[0] = to_cstr(&mount_point)
            .expect("mount point has null")
            .as_ptr()
            .cast::<c_ulong>();
        let r;
        unsafe {
            r = lkl_syscall(
                __lkl__NR_chdir as i64,
                ptr::addr_of_mut!(params).cast::<c_long>(),
            );
        }
        if r < 0 {
            return Err("Can't chdir to moint point corrupted filesystem");
        }
        // return string without null byte
        let mount_point = &mount_point[0..mount_point.len() - 1];
        Ok(LklSetup {
            disk: disk,
            partition: partition,
            disk_id: disk_id,
            mount_point: mount_point.to_owned(),
        })
    }
}

impl Drop for LklSetup {
    fn drop(&mut self) {
        unsafe {
            let r = lkl_umount_dev(self.disk_id, self.partition, 0, 1000) as i32;
            if r < 0 {
                //eprintln!("lkl_umount_dev: ");
                //print_error(&r);
            }
            lkl_disk_remove(self.disk);
            lkl_sys_halt();
        }
    }
}

pub struct LklSetupArgs {
    pub filesystem_fd: i32,
    pub boot_settings: Option<String>,
    pub partition_num: Option<u32>,
    pub filesystem_type: Option<String>,
    pub filesystem_options: Option<String>,
}

#[cfg(test)]
mod tests {
    use more_asserts as ma;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;
    use crate::*;
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
