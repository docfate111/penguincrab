use penguincrab::*;
use std::env::args;
use std::fs::File;
use std::process::exit;
//use std::ptr;
use std::os::unix::io::AsRawFd;

pub struct LklSetup {
    disk: lkl_disk,
    partition: u32,
    disk_id: u32,
}

impl LklSetup {
    pub fn new(
        filesystem_image: String,
        boot_settings: Option<String>,
        partition_num: Option<u32>,
        filesystem_type: Option<String>,
        filesystem_options: Option<String>,
    ) -> Result<LklSetup, &'static str> {
        let file = match File::options()
            .read(true)
            .write(true)
            .open(filesystem_image)
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
        let boot_arg = match boot_settings {
            Some(k) => to_cstr(&k),
            None => to_cstr("mem=128M loglevel=8"),
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
        let partition = partition_num.unwrap_or(0);
        let fs_type = &filesystem_type.unwrap_or("ext4".to_string()).to_owned()[..];
        let default_options = match fs_type {
            "ext4" => "errors=remount-ro",
            "btrfs" => "thread_pool=1",
            "gfs2" => "acl",
            "reiserfs" => "acl,user_xattr",
            &_ => "",
        };
        let mount_options = filesystem_options.unwrap_or(default_options.to_string());
        let msize: u32 = 100;
        let mut mpoint = vec![0u8; msize as usize];
        let ret;
        unsafe {
            ret = lkl_mount_dev(
                disk_id,
                partition,
                to_cstr(&fs_type),
                0,
                to_cstr(&mount_options),
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
                eprintln!("lkl_umount_dev: ");
                print_error(&r);
            }
            lkl_disk_remove(self.disk);
            lkl_sys_halt();
        }
    }
}

fn main() {
    let filename = match args().nth(1) {
        None => {
            eprintln!("Usage: pass in filesystem image as an argument");
            exit(1);
        }
        Some(k) => k,
    };
    {
    let _server = LklSetup::new(filename, None, None, None, None);
    };
	/*arams[0] = dir;
    let r;
    unsafe {
        r = lkl_syscall(__lkl__NR_chdir as i64, ptr::addr_of_mut!(params).cast::<c_long>());
    }
    println!("chdir {:}", r);*/
    //print_error(&(r as i32));
    exit(0);
}
