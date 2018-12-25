#![feature(abi_ptx)]
#![feature(core_intrinsics)]
#![feature(stdsimd)]
#![no_std]

use core::arch::nvptx::*;

#[no_mangle]
pub unsafe extern "ptx-kernel" fn hello_kernel(x: *mut u32, n: i32) {
  let idx = _thread_idx_x() as isize;
  match idx {
    0 => {
      *x.offset(idx) = 0x_6c_6c_65_48;
    }
    1 => {
      *x.offset(idx) = 0x_77_20_2c_6f;
    }
    2 => {
      *x.offset(idx) = 0x_64_6c_72_6f;
    }
    3 => {
      *x.offset(idx) = 0x_00_00_0a_21;
    }
    _ => {
      *x.offset(idx) = 0;
    }
  }
}
