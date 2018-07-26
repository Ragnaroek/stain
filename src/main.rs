use std::fs::{DirEntry};
use std::path::Path;
use std::env;

extern crate stain;

fn main() {

    let mut args = env::args();
    args.next();
    let arg1 = args.next();
    if arg1.is_some() {
        let p = &arg1.unwrap();
        let dir = Path::new(p);
        stain::coverage::go::measure(dir.to_path_buf());
        println!("dir={:?}", dir);
    }
}
