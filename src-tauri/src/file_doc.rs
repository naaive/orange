use std::path::PathBuf;
use jwalk::DirEntry;
use crate::utils;

#[derive(Debug)]
pub struct FileDoc {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub ext: String,
    pub parent_dirs: String,
}


impl From<DirEntry<((), ())>> for FileDoc {
    fn from(en: DirEntry<((), ())>) -> Self {
        let buf = en.path();
        let parent_dirs = utils::parent_dirs(buf.clone());
        let file_type = en.file_type();
        let is_dir = file_type.is_dir();
        let path = buf.to_str().unwrap();
        let name = en.file_name().to_str().unwrap();
        let ext = utils::file_ext(name);
        FileDoc {
            name: name.to_string(),
            path: path.to_string(),
            is_dir,
            ext: ext.to_string(),
            parent_dirs: parent_dirs.join(" "),
        }
    }
}


