#[cfg(windows)]
use std::ffi::CString;

use std::fs;

use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::api::dialog::message;
use tauri::{Manager, Window, Wry};

extern crate directories;
extern crate kernel32;
extern crate libc;
use directories::ProjectDirs;
//
pub fn msg(window: Window<Wry>, content: &str) {
  let parent_window = window.get_window("main").unwrap();
  message(Some(&parent_window), "Warm", content);
}

pub fn path2name(x: &str) -> Option<&str> {
  x.split("/").into_iter().last()
}

pub fn open_file_path(path: &str) {
  if cfg!(target_os = "windows") {
    let x = Path::new(path).parent().unwrap().to_str().unwrap();
    println!("{}", x);
    Command::new("explorer")
      .args([x])
      .output()
      .expect("failed to execute process");
  } else if cfg!(target_os = "linux") {
    Command::new("xdg-open")
      .args([Path::new(path).parent().unwrap().to_str().unwrap()])
      .output()
      .expect("failed to execute process");
  } else {
    //mac os
    Command::new("open")
      .args([Path::new(path).parent().unwrap().to_str().unwrap()])
      .output()
      .expect("failed to execute process");
  }
}

pub fn data_dir() -> String {
  let project_dir = ProjectDirs::from("com", "github", "Orange").unwrap();
  project_dir.data_dir().to_str().unwrap().to_string()
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
pub fn subs(str: &str) -> Vec<String> {
  if let Ok(paths) = fs::read_dir(str) {
    return paths
      .into_iter()
      .map(|x| x.unwrap().path().to_str().unwrap().to_string())
      .collect();
  }
  vec![]
}

#[cfg(unix)]
pub fn sub_root() -> Vec<String> {
  let paths = fs::read_dir("/").unwrap();
  let subs: Vec<String> = paths
    .into_iter()
    .map(|x| x.unwrap().path().to_str().unwrap().to_string())
    .collect();
  subs
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

pub fn parse_ts(time: SystemTime) -> u64 {
  let created_at = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
  created_at
}

#[cfg(windows)]
pub unsafe fn get_win32_ready_drives_no() -> Vec<String> {
  let vec = get_win32_ready_drives();
  let mut res = vec![];
  for x in vec {
    let s = str::replace(x.as_str(), ":/", "");
    res.push(s);
  }
  res.sort();
  res
}

#[cfg(windows)]
pub unsafe fn sub_root() -> Vec<String> {
  let drives = get_win32_ready_drives();
  let mut res = vec![];
  for driv in drives {
    let paths = fs::read_dir(driv).unwrap();
    let mut subs: Vec<String> = paths
      .into_iter()
      .map(|x| x.unwrap().path().to_str().unwrap().to_string())
      .collect();
    res.append(&mut subs);
  }
  res
}

#[cfg(windows)]
pub unsafe fn build_volume_path(str: &str) -> String {
  str::replace("\\\\?\\$:", "$", str)
}
// fn parse_ts(time: SystemTime) -> u64 {
//   let created_at = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
//   created_at
// }
// pub fn bfs_travel<T>(exclude_path: Vec<String>, func: T, roots: Vec<String>)
// where
//   T: Fn(FileView) -> (),
// {
//   for root in roots {
//     // let root = path;
//     let mut deque = VecDeque::new();
//
//     let path1 = Path::new(root.as_str());
//     let view = parse_file_view(path1);
//     deque.push_back(view);
//     loop {
//       Duration::from_millis(3);
//
//       if deque.is_empty() {
//         break;
//       }
//       let len = deque.len();
//       for _i in 0..len {
//         let p = deque.pop_front().unwrap();
//         func(p.clone());
//         // println!("{:?}", p);
//         let result = fs::read_dir(p.abs_path);
//         if result.is_ok() {
//           let paths = result.unwrap();
//           for path in paths {
//             if path.is_ok() {
//               let buf = path.ok().unwrap().path();
//               let x = buf.as_path();
//               let file_view = parse_file_view(x);
//               if exclude_path.iter().any(|y| file_view.abs_path.contains(y)) {
//                 continue;
//               }
//
//               deque.push_back(file_view);
//             }
//           }
//         }
//       }
//     }
//   }
// }

// fn parse_file_view(path1: &Path) -> FileView {
//   let result = path1.metadata();
//
//   let meta = result.unwrap();
//   #[cfg(windows)]
//   let size = meta.file_size();
//   #[cfg(unix)]
//   let size = meta.size();
//   let view = FileView {
//     abs_path: path1.to_str().unwrap().to_string(),
//     name: path1
//       .file_name()
//       .unwrap_or(&OsStr::new(""))
//       .to_str()
//       .unwrap()
//       .to_string(),
//     created_at: parse_ts(meta.created().unwrap()),
//     mod_at: parse_ts(meta.modified().unwrap()),
//     size: size,
//     is_dir: meta.is_dir(),
//   };
//   view
// }

#[cfg(test)]
mod tests {

  use std::collections::HashSet;

  use std::iter::FromIterator;
  use std::time::UNIX_EPOCH;

  #[cfg(windows)]
  use crate::utils::build_volume_path;
  #[cfg(windows)]
  use crate::utils::get_win32_ready_drives;
  #[cfg(windows)]
  use crate::utils::get_win32_ready_drives_no;

  use crate::utils::{data_dir, home_sub_dir, subs};

  #[cfg(windows)]
  #[test]
  fn t1() {
    unsafe {
      let vec = get_win32_ready_drives_no();
      for x in vec {
        let string = build_volume_path(x.as_str());
        println!("{}", string.as_str());
      }
    }
  }

  #[test]
  fn t2() {
    let vec1 = vec!["hi", "jack", "rose", "hi"];
    let set: HashSet<&str> = HashSet::from_iter(vec1);
    println!("{:?}", set);
  }

  #[test]
  fn t3() {
    let dir = home_sub_dir();

    println!("{:?}", dir);
  }

  #[cfg(unix)]
  #[test]
  fn t4() {
    let root = crate::utils::sub_root();
    println!("{:?}", root);
  }

  #[test]
  fn t5() {
    let subs1 = subs("/bin");
    println!("{:?}", subs1);
  }

  #[test]
  fn t6() {
    let result = std::fs::metadata(
      "D:\\Program Files (x86)\\Thunder Network\\Thunder\\Program\\resources\\bin\\TBC\\cef.pak",
    );
    let i = result
      .unwrap()
      .modified()
      .unwrap()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis();
    println!("{:?}", i as u64);
  }

  #[test]
  fn t7() {
    let dir = data_dir();
    println!("{}", dir);
  }
}
