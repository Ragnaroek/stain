use std::path::PathBuf;
use std::fs;
use std::io;
use std::process::Command;

#[derive(Debug)]
pub struct Coverage {
    pub module: PathBuf
}

pub fn measure(path: PathBuf) -> io::Result<Vec<Coverage>> {
    let go_folders = collect_go_module_folders(path);
    return Ok(collect_coverage(go_folders?));
}

fn collect_coverage(paths: Vec<PathBuf>) -> Vec<Coverage>{
    println!("### collect coverage");
    let mut coverage_result = Vec::new();
    for path in paths {
        println!("### coverage for path {:?}", path);
        let result = Command::new("go").current_dir(&path).arg("test").arg("-cover").output();
        coverage_result.push(Coverage{
            module: path.to_path_buf()
        });

        println!("#### output {:?}", result);
    }
    return coverage_result;
}

pub fn collect_go_module_folders(path: PathBuf) -> io::Result<Vec<PathBuf>> {

    let mut result = Vec::new();
    let mut append_self = false;

    for entry_result in fs::read_dir(path.to_path_buf())? {
        let entry = entry_result?;
        if entry.file_type()?.is_dir() {
            let sub_dir_path = entry.path();
            let mut sub_dir_result = collect_go_module_folders(sub_dir_path)?;
            result.append(&mut sub_dir_result);
        } else if entry.file_name().to_string_lossy().ends_with(".go") {
            append_self = true;
        }
    }
    if append_self {
        result.push(path);
    }
    result.sort();
    return Ok(result);
}
