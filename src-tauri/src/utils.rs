use std::path::Path;
use std::process::Command;

pub fn open_file_path(path: &str) {
    //mac os
    Command::new("open")
        .args([Path::new(path).parent().unwrap().to_str().unwrap()])
        .output()
        .expect("failed to execute process");
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn t1() {
        open_file_path(".")
    }
}