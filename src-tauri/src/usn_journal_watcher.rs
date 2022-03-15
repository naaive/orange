use std::mem::size_of;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use windows::Win32::Foundation;
use windows::Win32::Storage::FileSystem;
use windows::Win32::System::Ioctl;
use windows::Win32::System::IO;

fn get_usn_record_time(record: &Ioctl::USN_RECORD_V3) -> std::time::Duration {
  std::time::Duration::from_nanos((record.TimeStamp as u64) * 100u64)
}

fn get_usn_record_name(record: &Ioctl::USN_RECORD_V3) -> String {
  let size = (record.FileNameLength / 2) as usize;

  if size > 0 {
    unsafe {
      let name_u16 = std::slice::from_raw_parts(record.FileName.as_ptr() as *const u16, size);
      let name = std::ffi::OsString::from_wide(name_u16)
        .to_string_lossy()
        .into_owned();
      return name;
    }
  }

  return String::new();
}

fn get_file_path(
  volume_handle: &Foundation::HANDLE,
  file_id: &FileSystem::FILE_ID_128,
) -> Option<PathBuf> {
  let file_id_desc = FileSystem::FILE_ID_DESCRIPTOR {
    Type: FileSystem::ExtendedFileIdType,
    dwSize: size_of::<FileSystem::FILE_ID_DESCRIPTOR>() as u32,
    Anonymous: FileSystem::FILE_ID_DESCRIPTOR_0 {
      ExtendedFileId: *file_id,
    },
  };

  unsafe {
    let file_handle = FileSystem::OpenFileById(
      volume_handle,
      &file_id_desc,
      FileSystem::FILE_GENERIC_READ,
      FileSystem::FILE_SHARE_READ | FileSystem::FILE_SHARE_WRITE | FileSystem::FILE_SHARE_DELETE,
      std::ptr::null_mut(),
      0,
    );

    if file_handle.is_invalid() {
      return None;
    }

    let info_buffer_size =
      size_of::<FileSystem::FILE_NAME_INFO>() + (Foundation::MAX_PATH as usize) * size_of::<u16>();
    let mut info_buffer = vec![0u8; info_buffer_size];
    let info_result = FileSystem::GetFileInformationByHandleEx(
      file_handle,
      FileSystem::FileNameInfo,
      &mut *info_buffer as *mut _ as *mut c_void,
      info_buffer_size as u32,
    );

    Foundation::CloseHandle(file_handle);

    if info_result.as_bool() {
      let (_, body, _) = info_buffer.align_to::<FileSystem::FILE_NAME_INFO>();
      let info = &body[0];
      let name_len = info.FileNameLength as usize / size_of::<u16>();
      let name_u16 = std::slice::from_raw_parts(info.FileName.as_ptr() as *const u16, name_len);
      let path = PathBuf::from(std::ffi::OsString::from_wide(name_u16));
      return Some(path);
    } else {
      return None;
    }
  }
}

fn get_usn_record_path(
  volume: &Foundation::HANDLE,
  record: &Ioctl::USN_RECORD_V3,
) -> (PathBuf, String) {
  let parent_path = get_file_path(&volume, &record.ParentFileReferenceNumber);
  let file_name = get_usn_record_name(&record);
  match parent_path {
    Some(path) => (path.join(&file_name), file_name),
    None => (PathBuf::from(&file_name), file_name),
  }
}

//fn get_usn_record_current_path(
//    volume: &Foundation::HANDLE,
//    record: &Ioctl::USN_RECORD_V3,
//) -> PathBuf {
//    return get_file_path(&volume, &record.FileReferenceNumber).unwrap_or_default();
//}

#[derive(Debug, Clone)]
pub struct UsnRecord {
  pub usn: i64,
  pub timestamp: std::time::Duration,
  pub file_id: u128,
  pub parent_id: u128,
  pub reason: u32,
  pub path: PathBuf,
  pub file_name: String,
}

pub struct Watcher {
  volume_handle: Foundation::HANDLE,
  journal: Ioctl::USN_JOURNAL_DATA_V2,
  next_usn: i64,
  reason_mask: u32, // Ioctl::USN_REASON_FILE_CREATE
  history: Vec<UsnRecord>,
}

impl Watcher {
  pub fn new<P: AsRef<Path>>(
    volume_path: P,
    reason_mask: Option<u32>,
    next_usn: Option<i64>,
  ) -> Result<Watcher, std::io::Error> {
    let volume_handle: Foundation::HANDLE;

    unsafe {
      volume_handle = FileSystem::CreateFileA(
        volume_path.as_ref().to_str().unwrap(),
        FileSystem::FILE_GENERIC_READ | FileSystem::FILE_GENERIC_WRITE,
        FileSystem::FILE_SHARE_READ | FileSystem::FILE_SHARE_WRITE | FileSystem::FILE_SHARE_DELETE,
        std::ptr::null_mut(),
        FileSystem::OPEN_EXISTING,
        0,
        None,
      );
      if volume_handle.is_invalid() {
        return Err(std::io::Error::last_os_error());
      }
    }

    let mut journal = Ioctl::USN_JOURNAL_DATA_V2::default();

    unsafe {
      let mut ioctl_bytes_returned = 0;
      let result = IO::DeviceIoControl(
        volume_handle,
        Ioctl::FSCTL_QUERY_USN_JOURNAL,
        std::ptr::null_mut(),
        0,
        &mut journal as *mut _ as *mut c_void,
        size_of::<Ioctl::USN_JOURNAL_DATA_V2>() as u32,
        &mut ioctl_bytes_returned,
        std::ptr::null_mut(),
      );
      if result.0 == 0 {
        return Err(std::io::Error::last_os_error());
      }
    }

    Ok(Watcher {
      volume_handle,
      journal,
      next_usn: next_usn.unwrap_or(journal.NextUsn),
      reason_mask: reason_mask.unwrap_or(0xFFFFFFFF),
      history: Vec::new(),
    })
  }

  pub fn read(&mut self) -> Result<Vec<UsnRecord>, std::io::Error> {
    let mut results = Vec::<UsnRecord>::new();

    let mut read = Ioctl::READ_USN_JOURNAL_DATA_V1 {
      StartUsn: self.next_usn,
      ReasonMask: self.reason_mask,
      ReturnOnlyOnClose: 0,
      Timeout: 0,
      BytesToWaitFor: 0,
      UsnJournalID: self.journal.UsnJournalID,
      MinMajorVersion: 3,
      MaxMajorVersion: 3,
    };

    let mut buffer = [0u8; 4096];
    let mut ioctl_bytes_returned = 0;

    unsafe {
      let ioctl_result = IO::DeviceIoControl(
        self.volume_handle,
        Ioctl::FSCTL_READ_USN_JOURNAL,
        &mut read as *mut _ as *mut c_void,
        size_of::<Ioctl::READ_USN_JOURNAL_DATA_V1>() as u32,
        &mut buffer as *mut _ as *mut c_void,
        4096,
        &mut ioctl_bytes_returned,
        std::ptr::null_mut(),
      );

      if ioctl_result.0 == 0 {
        return Err(std::io::Error::last_os_error());
      }
    }

    unsafe {
      let next_usn = *(buffer.as_ptr() as *const i64);
      if next_usn == 0 || next_usn < self.next_usn {
        return Ok(results);
      } else {
        self.next_usn = next_usn;
      }
    }

    let mut offset = 8; // sizeof(USN)
    while offset < ioctl_bytes_returned {
      let record;
      let record_length;

      unsafe {
        let record_raw = std::mem::transmute::<*const u8, *const Ioctl::USN_RECORD_UNION>(
          buffer[offset as usize..].as_ptr(),
        );
        let header = &(*record_raw).Header;

        if header.RecordLength == 0 || header.MajorVersion != 3 {
          break;
        }

        record_length = header.RecordLength;
        record = &(*record_raw).V3;
      }

      let (path, file_name) = get_usn_record_path(&self.volume_handle, &record);
      let record = UsnRecord {
        usn: record.Usn,
        timestamp: get_usn_record_time(&record),
        file_id: u128::from_le_bytes(record.FileReferenceNumber.Identifier),
        parent_id: u128::from_le_bytes(record.ParentFileReferenceNumber.Identifier),
        reason: record.Reason,
        path: path,
        file_name,
      };

      if record.reason
        & (Ioctl::USN_REASON_RENAME_OLD_NAME
          | Ioctl::USN_REASON_HARD_LINK_CHANGE
          | Ioctl::USN_REASON_REPARSE_POINT_CHANGE)
        != 0
      {
        self.history.push(record.clone());
      }

      results.push(record);
      offset += record_length;
    }

    Ok(results)
  }

  //noinspection RsExternalLinter
  pub fn _match_rename(&self, record: &UsnRecord) -> Option<PathBuf> {
    if record.reason & Ioctl::USN_REASON_RENAME_NEW_NAME == 0 {
      return None;
    }

    match self
      .history
      .iter()
      .find(|r| r.file_id == record.file_id && r.usn < record.usn)
    {
      Some(r) => Some(r.path.clone()),
      None => None,
    }
  }

  pub fn _trim_history(&mut self, min_usn: Option<i64>) {
    match min_usn {
      Some(usn) => self.history.retain(|r| r.usn > usn),
      None => self.history.clear(),
    }
  }
}

impl Drop for Watcher {
  fn drop(&mut self) {
    unsafe {
      Foundation::CloseHandle(self.volume_handle);
    }
  }
}

#[cfg(test)]
mod test {

  use std::fs::File;
  use std::io::Write;
  use std::time::{SystemTime, UNIX_EPOCH};

  use super::*;

  #[test]
  fn file_create() -> Result<(), std::io::Error> {
    let option = Option::Some(0);
    // let option = Option::None;
    let volume_path = "C:";
    let mut watcher = Watcher::new(format!("{}{}", "\\\\?\\", volume_path), None, option)?;

    let _start = SystemTime::now();

    let mut cnt = 0;
    loop {
      let result = watcher.read();
      if result.is_err() {
        watcher = Watcher::new(format!("{}{}", "\\\\?\\", volume_path), None, Some(0)).unwrap();
        continue;
      }
      let results = result?;
      for _x in results.clone() {
        cnt += 1;
        // let path = x.path.to_str().unwrap();
        // match fs::metadata(format!("{}{}", volume_path, path)) {
        //     Ok(_meta) => {}
        //     Err(_) => {}
        // }
        // println!("{:?}", metadata);
        // println!("{}", path);
      }
      if results.is_empty() {
        break;
      };
    }
    println!("{}", cnt);

    let since_the_epoch = _start
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
    println!("使用{:?}", since_the_epoch);
    Ok(())
  }

  #[test]
  fn file_move() -> Result<(), std::io::Error> {
    let mut watcher = Watcher::new("\\\\?\\C:", None, None)?;

    let path_old = std::env::temp_dir().join("usn-journal-watcher-test-move.txt");
    let path_new = path_old.with_extension("moved");

    File::create(path_old.as_path())?.write_all(b"test")?;
    std::fs::rename(path_old.as_path(), path_new.as_path())?;

    let results = watcher.read()?;
    for result in results {
      if (result.reason & Ioctl::USN_REASON_RENAME_NEW_NAME != 0) && result.path == path_new {
        if let Some(path) = watcher._match_rename(&result) {
          assert_eq!(path, path_old);
        } else {
          panic!("No old path found for {}", result.path.to_str().unwrap());
        }
      }
    }

    Ok(())
  }
}
