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
    build.file("libwebp-1.0.0/src/dec/alpha_dec.c");
    build.file("libwebp-1.0.0/src/dec/buffer_dec.c");
    build.file("libwebp-1.0.0/src/dec/frame_dec.c");
    build.file("libwebp-1.0.0/src/dec/idec_dec.c");
    build.file("libwebp-1.0.0/src/dec/io_dec.c");
    // build.file("libwebp-1.0.0/src/dec/quant_dec.c");
    build.file("libwebp-1.0.0/src/dec/tree_dec.c");
    build.file("libwebp-1.0.0/src/dec/vp8_dec.c");
    build.file("libwebp-1.0.0/src/dec/vp8l_dec.c");
    build.file("libwebp-1.0.0/src/dec/webp_dec.c");
    dir(&mut build, "src/enc");
    dir(&mut build, "src/dsp");
    build.file("libwebp-1.0.0/src/utils/bit_reader_utils.c");
    build.file("libwebp-1.0.0/src/utils/bit_writer_utils.c");
    build.file("libwebp-1.0.0/src/utils/color_cache_utils.c");
    build.file("libwebp-1.0.0/src/utils/filters_utils.c");
    build.file("libwebp-1.0.0/src/utils/huffman_encode_utils.c");
    build.file("libwebp-1.0.0/src/utils/huffman_utils.c");
    build.file("libwebp-1.0.0/src/utils/quant_levels_dec_utils.c");
    build.file("libwebp-1.0.0/src/utils/quant_levels_utils.c");
    // build.file("libwebp-1.0.0/src/utils/random_utils.c");
    build.file("libwebp-1.0.0/src/utils/rescaler_utils.c");
    build.file("libwebp-1.0.0/src/utils/thread_utils.c");
    build.file("libwebp-1.0.0/src/utils/utils.c");
    build.compile("webp");
}
