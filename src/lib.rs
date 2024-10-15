#![allow(dead_code)]

pub mod gray_4bpp;
pub mod gray_4bpp_with_text;
pub mod monochrome;

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
