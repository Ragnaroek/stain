use std::fs::{DirEntry};
use std::path::Path;
use std::env;


fn main() {

    let mut args = env::args();
    args.next();
    let arg1 = args.next();
    if arg1.is_some() {
        let p = &arg1.unwrap();
        let dir = Path::new(p);
        let go_folders = Vec::new();
        collect_go_folders(dir, go_folders);
        println!("dir={:?}", dir);
    }
}

//TODO separate lib for go-coverage
fn collect_go_folders(path: &Path, go_folders: Vec<&Path>) {

}
