extern crate cc;

use std::path::PathBuf;

fn dir(build: &mut cc::Build, dir: &str) {
    let mut path = PathBuf::from("libwebp-1.0.0");
    path.push(dir);
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension() == Some("c".as_ref()) {
            build.file(path);
        }
    }
}

fn main() {
    let mut build = cc::Build::new();
    build.include("libwebp-1.0.0");
    dir(&mut build, "src/dec");
    dir(&mut build, "src/enc");
    dir(&mut build, "src/dsp");
    dir(&mut build, "src/utils");
    build.compile("webp");
}
