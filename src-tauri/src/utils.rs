#[cfg(windows)]
use std::ffi::CString;


use std::fs;
use std::path::Path;
use std::process::Command;
extern crate kernel32;
extern crate libc;

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

pub fn home_sub_dir() -> Vec<String> {
  let dir = home_dir();
  let paths = fs::read_dir(dir).unwrap();
  let subs: Vec<String> = paths
    .into_iter()
    .map(|x| x.unwrap().path().to_str().unwrap().to_string())
    .collect();

  let mut res1 = Vec::new();
  let mut res2 = Vec::new();
  for sub in subs {
    let often = vec![
      "Documents",
      "Desktop",
      "Downloads",
      "Movies",
      "Music",
      "Pictures",
    ];
    if often.into_iter().any(|x3| sub.contains(x3)) {
      res2.push(sub);
    } else {
      res1.push(sub);
    };
  }
  [res2, res1].concat()
}

#[cfg(windows)]
pub unsafe fn get_win32_ready_drives() -> Vec<String> {
  let mut logical_drives = Vec::with_capacity(5);
  let mut bitfield = kernel32::GetLogicalDrives();
  let mut drive = 'A';

  while bitfield != 0 {
    if bitfield & 1 == 1 {
      let strfulldl = drive.to_string() + ":\\";
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
  logical_drives
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;
  use super::*;
  use std::fs;
  use std::iter::FromIterator;

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
    let vec1 = vec!["hi", "jack", "rose", "hi"];
    let set:HashSet<&str> = HashSet::from_iter(vec1);
    println!("{:?}", set);
  }
}
