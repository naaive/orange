extern crate notify;

use std::{env, thread};
use std::sync::mpsc::{channel};


use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for path in args {
        thread::spawn(move || {
            do_watch(&path);
        }).join().unwrap();
    }
    println!("{}", 123);
}

fn do_watch(path: &str) -> ! {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent { path: Some(path), op: Ok(op), cookie: _ }) => {
                eprintln!("{:?} {:?} ", op, path)
            }
            Ok(_event) => (),
            Err(_e) => (),
        }
    }
}
