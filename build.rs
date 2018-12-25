extern crate nvptxglue;

use nvptxglue::prelude::*;

fn main() {
  nvptxglue::Builder::default()
    .crate_dir("kernels")
    .gencode(Gencode::Ptx(Cc_3_5))
    //.gencode(Gencode::Sass(Cc_3_5))
    //.gencode(Gencode::Sass(Cc_5_2))
    //.gencode(Gencode::Sass(Cc_6_0))
    //.gencode(Gencode::Sass(Cc_6_1))
    .whitelist_kernel("hello_kernel")
    .generate()
    .expect("failed to generate glue")
    .write_to_file("kernels_glue.rs")
    .expect("failed to write glue to file");
  //panic!();
}
