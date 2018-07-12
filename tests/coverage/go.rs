extern crate stain;

use std::path::Path;
use stain::coverage::go;

#[test]
pub fn test_measure() {
    let dir = Path::new("tests/testdata/go/");
    let result = go::measure(&dir);
    println!("result = {:?}", result);
}
