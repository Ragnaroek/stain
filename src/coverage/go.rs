use std::path::Path;
use std::fs;
use std::io;

pub fn measure(path: &Path) -> io::Result<Vec<&Path>> {
    let go_folders = collect_go_folders(path);
    return go_folders;
}

fn collect_go_folders(path: &Path) -> io::Result<Vec<&Path>> {

    for entry in fs::read_dir(path)? {
        println!("dir = {:?}", entry);
    }

    return Ok(Vec::new());
}
