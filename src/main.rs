use std::path::Path;
use std::env;

extern crate stain;
extern crate env_logger;

fn main() {
    env_logger::init();

    let mut args = env::args();
    args.next();
    let arg1 = args.next();
    if arg1.is_some() {
        let p = &arg1.unwrap();
        let dir = Path::new(p);
        let coverage_result = stain::coverage::go::measure(dir.to_path_buf());
        println!("result={:?}", coverage_result);
    }
}
