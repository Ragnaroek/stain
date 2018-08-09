extern crate stain;

use std::path::{Path, PathBuf};
use stain::coverage::go;

#[test]
pub fn test_collect_go_module_folders() {
    let dir = Path::new("tests/testdata/go/");
    let result = go::collect_go_module_folders(dir.to_path_buf()).unwrap();
    assert_eq!(result, vec![
        PathBuf::from("tests/testdata/go/top1"),
        PathBuf::from("tests/testdata/go/top2/top21"),
        PathBuf::from("tests/testdata/go/top2/top21/top211"),
        PathBuf::from("tests/testdata/go/top2/top22"),
        PathBuf::from("tests/testdata/go/top3_no_test"),
    ]);
}

#[test]
pub fn test_measure() {
    let dir = Path::new("tests/testdata/go/");
    let result = go::measure(dir.to_path_buf()).unwrap();
    println!("### measure result {:?}", result);
}
