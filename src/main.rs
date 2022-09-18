use penguincrab::*;
use std::env::args;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::exit;
use std::ptr;

pub struct LklSetup {
    disk: lkl_disk,
    partition: u32,
    disk_id: u32,
}

impl LklSetup {
    pub fn new(arg: LklSetupArgs) -> Result<LklSetup, &'static str> {
        let file = match File::options()
            .read(true)
            .write(true)
            .open(arg.filesystem_image)
        {
            Err(e) => {
                eprintln!("Error opening {:}", e);
                return Err("Can't open filesystem image");
            }
            Ok(k) => k,
        };
        let mut disk = lkl_disk {
            dev: 0,
            fd: file.as_raw_fd(),
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
        Ok(LklSetup {
            disk: disk,
            partition: partition,
            disk_id: disk_id,
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
    filesystem_image: String,
    boot_settings: Option<String>,
    partition_num: Option<u32>,
    filesystem_type: Option<String>,
    filesystem_options: Option<String>,
}

fn main() {
    let filename = match args().nth(1) {
        None => {
            eprintln!("Usage: pass in filesystem image as an argument");
            exit(1);
        }
        Some(k) => k,
    };
    let _server = LklSetup::new(LklSetupArgs {
        filesystem_image: filename,
        boot_settings: None,
        partition_num: None,
        filesystem_type: None,
        filesystem_options: None,
    });
    /*let mut params = [ptr::null::<c_ulong>(); 5];
    params[0] = to_cstr("/\0").unwrap().as_ptr().cast::<c_ulong>();
    let r;
    unsafe {
        r = lkl_syscall(
            __lkl__NR_chdir as i64,
            ptr::addr_of_mut!(params).cast::<c_long>(),
        );
    }*/
    let r = lkl_sys_open("/test/f", LKL_O_RDWR, 0);
    println!("open fd {:}", r);
    print_error(&(r as i32));

    exit(0);
}
