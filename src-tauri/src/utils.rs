use directories::ProjectDirs;
use std::ffi::CString;
use std::time::SystemTime;

pub fn subs(str: &str) -> Vec<String> {
    if let Ok(paths) = std::fs::read_dir(str) {
        return paths
            .into_iter()
            .map(|x| x.unwrap().path().to_str().unwrap().to_string())
            .collect();
    }
    vec![]
}
pub fn open_file_path(path: &str) {
    if cfg!(target_os = "windows") {
        let x = std::path::Path::new(path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        println!("{}", x);
        std::process::Command::new("explorer")
            .args([win_norm4explorer(x)])
            .output()
            .expect("failed to execute process");
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("xdg-open")
            .args([std::path::Path::new(path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()])
            .output()
            .expect("failed to execute process");
    } else {
        //mac os
        std::process::Command::new("open")
            .args([std::path::Path::new(path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()])
            .output()
            .expect("failed to execute process");
    }
}

pub fn data_dir() -> String {
    let project_dir = ProjectDirs::from("com", "github", "Orange").unwrap();
    project_dir.data_dir().to_str().unwrap().to_string()
}

pub fn parse_ts(time: SystemTime) -> u64 {
    let created_at = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;
    created_at
}
pub fn path2name(x: &str) -> Option<&str> {
    x.split("/").into_iter().last()
}

pub fn norm(path: &str) -> String {
    str::replace(path, "\\", "/")
}

pub fn win_norm4explorer(path: &str) -> String {
    str::replace(path, "/", "\\")
}
#[cfg(windows)]
pub unsafe fn get_win32_ready_drives() -> Vec<String> {
    let mut logical_drives = Vec::with_capacity(5);
    let mut bitfield = kernel32::GetLogicalDrives();
    let mut drive = 'A';

    while bitfield != 0 {
        if bitfield & 1 == 1 {
            let strfulldl = drive.to_string() + ":/";
            let cstrfulldl = CString::new(strfulldl.clone()).unwrap();
            let x = kernel32::GetDriveTypeA(cstrfulldl.as_ptr());
            if x == 3 || x == 2 {
                logical_drives.push(strfulldl);
                // println!("drive {0} is {1}", strfdl, x);
            }
        }
        drive = std::char::from_u32((drive as u32) + 1).unwrap();
        bitfield >>= 1;
    }
    logical_drives.sort_by(|x1, x2| x2.cmp(x1));
    logical_drives
}

#[cfg(windows)]
pub unsafe fn build_volume_path(str: &str) -> String {
    str::replace("\\\\?\\$:", "$", str)
}

#[test]
fn t1() {
    let str = "c";
    let string = unsafe{build_volume_path(str)};
    println!("{}", string);
}
