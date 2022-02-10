use std::ffi::CString;
use std::path::Path;
use std::process::Command;
extern crate kernel32;
extern crate libc;
extern crate glob;

use self::glob::glob;

pub fn open_file_path(path: &str) {
    //mac os
    Command::new("open")
        .args([Path::new(path).parent().unwrap().to_str().unwrap()])
        .output()
        .expect("failed to execute process");
}

pub fn home_dir() -> String {
    let option = dirs::home_dir();
    option.unwrap().to_str().unwrap().to_string()
}

#[cfg(windows)]
pub unsafe fn get_win32_ready_drives() -> Vec<String>
{
    let mut logical_drives = Vec::with_capacity(5);
    let mut bitfield = kernel32::GetLogicalDrives();
    let mut drive = 'A';
    let mut rtstr = CString::new("");

    while bitfield != 0 {
        if bitfield & 1 == 1 {
            let strfulldl = drive.to_string() + ":\\";
            let cstrfulldl = CString::new(strfulldl.clone()).unwrap();
            let x = kernel32::GetDriveTypeA(cstrfulldl.as_ptr());
            if (x == 3 || x == 2)
            {
                logical_drives.push(strfulldl);
                // println!("drive {0} is {1}", strfdl, x);
            }
        }
        drive = std::char::from_u32((drive as u32) + 1).unwrap();
        bitfield >>= 1;
    }
    logical_drives
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate glob;

    use self::glob::glob;

    #[cfg(windows)]
    #[test]
    fn t1() {
        unsafe {
            let vec = get_win32_ready_drives();
            println!("{:?}", vec);
        }
    }

    #[test]
    fn t2() {
        let dir = home_dir();
        println!("{}", dir);
    }
}