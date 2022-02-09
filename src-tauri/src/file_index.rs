use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileIndex {
    pub abs_path: String,
    pub name: String,
}

#[test]
fn t1() {
    let file = FileIndex {
        abs_path: String::from("jack"),
        name: String::from("rose"),
    };
    let result = serde_json::to_string(&file);
    println!("{:?}", result);
}
