extern crate nvptxglue;

use nvptxglue::prelude::*;

//use std::env;

fn main() {
  //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  nvptxglue::Builder::default()
    .crate_dir("kernels")
    //.gencode(Gencode::VGpu(Cc_3_5))
    .gencode(Gencode::Gpu(Cc_3_5))
    //.gencode(Gencode::Gpu(Cc_5_2))
    //.gencode(Gencode::Gpu(Cc_6_0))
    //.gencode(Gencode::Gpu(Cc_6_1))
    .whitelist_kernel("hello_kernel")
    .generate()
    .expect("failed to generate glue")
    //.write_to_file(out_dir.join("kernels_glue.rs"))
    .write_to_file(CudaFfiGlue::default(), "src/kernels_glue.rs")
    .expect("failed to write glue to file");
}
