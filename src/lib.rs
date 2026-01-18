#![allow(dead_code)]

pub mod diff;
pub mod grey_4bpp;
pub mod grey_4bpp_with_text;
pub mod qoi_convert;
pub mod monochrome;
pub mod rgb_convert;
pub mod png_convert;

use napi_derive::napi;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
