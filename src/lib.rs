pub mod lkl;
pub use lkl::syscall_wrappers::*;
use nix::unistd::close;
use std::fs::File;
use std::os::unix::io::{IntoRawFd, RawFd};

/// construct this with LklSetup::new()
pub struct LklSetup {
    pub disk: lkl_disk,
    partition: u32,
    disk_id: u32,
    pub mount_point: String,
    file: RawFd,
}

impl LklSetup {
    /// new parses the settings and initializes the kernel
    pub fn new(arg: LklSetupArgs) -> Result<LklSetup, &'static str> {
        let file = match File::options().read(true).write(true).open(arg.filename) {
            Err(e) => {
                panic!("Error opening {:}", e);
            }
            Ok(k) => k.into_raw_fd(),
        };

        let mut disk = lkl_disk {
            dev: 0,
            fd: file as i32,
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
        fn on_panic() {
            println!("an oopsie happened");
        }
        unsafe {
            lkl_host_ops.panic = match arg.on_panic {
                Some(k) => (k as *const fn()) as c_ulong,
                None => (on_panic as *const fn()) as c_ulong,
            };
            if arg.print.is_some() {
                lkl_host_ops.print = (arg.print.unwrap() as *const fn()) as c_ulong;
            }
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
            file: file,
        })
    }
}

/// Unmounts the disk for LKL then removes it and stops the kernel
impl Drop for LklSetup {
    fn drop(&mut self) {
        close(self.file).unwrap();
        unsafe {
            lkl_umount_dev(self.disk_id, self.partition, 0, 1000) as i32;
            lkl_disk_remove(self.disk);
            lkl_sys_halt();
        }
    }
}

/// Due to ownership automatically deallocating fds out of scope the file descriptor for
/// the disk image needs to be passed in.
/// Next is the boot settings which can be the amount of memory and log level
/// (i.e. mem=128M loglevel=8
/// on_panic is the function that runs when there is a panic and print replaces printk
pub struct LklSetupArgs {
    pub filename: String,
    pub boot_settings: Option<String>,
    pub partition_num: Option<u32>,
    pub filesystem_type: Option<String>,
    pub filesystem_options: Option<String>,
    pub on_panic: Option<fn()>,
    pub print: Option<fn()>,
}

fn setup_test() -> LklSetup {
    // pass in the file descriptor so the library can read and write
    // the changes to disk:
    LklSetup::new(LklSetupArgs {
        filename: String::from("ext4-00.image"),
        boot_settings: None,
        partition_num: None,
        filesystem_type: None,
        filesystem_options: None,
        on_panic: None,
        print: None,
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use more_asserts as ma;
    #[test]
    fn separate_later() {
        let server = setup_test();

        const BUF_LEN: usize = 26;
        const MSG: &str = "that's what i call riddim\0";
        let mut mpoint = server.mount_point.clone();
        mpoint.push_str("/test591\0");
        let filename = to_cstr(&mpoint).unwrap();

        // open a file in the mounted filesystem - make sure to use null bytes to terminate CStrings
        let mut r = lkl_sys_open(&filename, LKL_O_RDWR | LKL_O_CREAT, 0);
        ma::assert_ge!(r, 0);

        let fd = r as i32;
        let buf = MSG.as_bytes();
        r = lkl_sys_write(fd, buf, BUF_LEN);
        assert_eq!(r as usize, BUF_LEN);
        r = lkl_sys_close(fd);
        assert_eq!(r, 0);

        let mut read_buf = [0 as u8; BUF_LEN];
        let readfd = lkl_sys_open(&filename, LKL_O_RDONLY, 0) as i32;
        ma::assert_ge!(r, 0);
        // reading back our message from the file we wrote to:
        r = lkl_sys_read(readfd, &mut read_buf, BUF_LEN);
        assert_eq!(r as usize, BUF_LEN);
        assert_eq!(MSG, String::from_utf8(read_buf.to_vec()).unwrap());

        let mut stat = lkl_stat {
            ..Default::default()
        };
        let r = lkl_sys_fstat(readfd, &mut stat);
        const S_IFREG: u64 = 0o0100000;
        const S_IFMT: u64 = 0o0170000;
        const S_IFDIR: u64 = 0o0040000;
        // check if it is a regular file
        assert_eq!((stat.st_mode & S_IFMT), S_IFREG);
        // confirm it is not a directory
        assert_ne!((stat.st_mode & S_IFMT), S_IFDIR);
        ma::assert_ge!(r, 0);

        let mut ruid = 12;
        let mut euid = 12;
        let mut suid = 12;
        // get real, effective, and saved user IDs
        lkl_sys_getresuid(&mut ruid, &mut euid, &mut suid);
        assert_ne!(suid, 12);
        assert_eq!(stat.st_uid, ruid);

        let mut rgid = 12;
        let mut egid = 12;
        let mut sgid = 12;
        lkl_sys_getresgid(&mut rgid, &mut egid, &mut sgid);
        assert_ne!(sgid, 12);
        assert_eq!(stat.st_gid, rgid);
        assert_eq!(egid, 0);
        assert_eq!(euid, 0);

        const SEEK_SET: u32 = 0;
        let offset: u32 = 3;
        // seek should return offset if successful
        let r = lkl_sys_lseek(readfd, offset, SEEK_SET) as u32;
        assert_eq!(r, offset);

        /*const PAGE_SIZE: usize = 0x1000;
        const PROT_READ: i32 = 1;
        const MAP_SHARED: i32 = 4;
        let mut page = [0; PAGE_SIZE];
        // this works in C fine but no setup of mmap works for some reason
        // idk whether to use &mut [u8] or u64 for the address
        let ptr = lkl_sys_mmap(
                0x5b0000, 0x1000, 0x1|0x2, 0x10|0x20|0x02,-1, 0);
        print_error(&(ptr as i32));
        ma::assert_ge!(ptr, 0);*/
    }
}
