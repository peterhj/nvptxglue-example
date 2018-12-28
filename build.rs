extern crate nvptxglue;

use nvptxglue::prelude::*;

fn main() {
  nvptxglue::Builder::default()
    .crate_dir("kernels")
    .gencode(Gencode::Ptx(Cc_3_5))
    //.gencode(Gencode::Gpu(Cc_3_5))
    //.gencode(Gencode::Gpu(Cc_5_2))
    //.gencode(Gencode::Gpu(Cc_6_0))
    //.gencode(Gencode::Gpu(Cc_6_1))
    .whitelist_kernel("hello_kernel")
    .compile(Phase::Fatbin)
    .expect("failed to compile kernels")
    .write_bindings_to_file(CudaFfiGlue::default(), "src/kernels_glue.rs")
    .expect("failed to generate glue bindings");
}
