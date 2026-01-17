use std::ops::Deref;

use lzzzz::lz4;
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;

struct DiffImageTask(Buffer, Buffer);

#[napi]
impl Task for DiffImageTask {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let new_input = (&self.0).deref();
    let old_input = (&self.1).deref();

    // Copy to Vec to ensure thread safety (snapshotting the data)
    let new_buf = Vec::<u8>::from(new_input);
    let old_buf = Vec::<u8>::from(old_input);

    let (new_header, new_pixels) = match qoi::decode_to_vec(&new_buf) {
      Ok(res) => res,
      Err(_) => {
        return Err(Error::new(
          Status::InvalidArg,
          "Failed to decode new QOI image".to_string(),
        ))
      }
    };

    let (old_header, old_pixels) = match qoi::decode_to_vec(&old_buf) {
      Ok(res) => res,
      Err(_) => {
        return Err(Error::new(
          Status::InvalidArg,
          "Failed to decode old QOI image".to_string(),
        ))
      }
    };

    if new_header.channels != qoi::Channels::Rgb {
      return Err(Error::new(
        Status::InvalidArg,
        "New image is not RGB".to_string(),
      ));
    }
    if old_header.channels != qoi::Channels::Rgb {
      return Err(Error::new(
        Status::InvalidArg,
        "Old image is not RGB".to_string(),
      ));
    }
    if new_header.colorspace != qoi::ColorSpace::Srgb {
      return Err(Error::new(
        Status::InvalidArg,
        "New image is not SRGB".to_string(),
      ));
    }
    if old_header.colorspace != qoi::ColorSpace::Srgb {
      return Err(Error::new(
        Status::InvalidArg,
        "Old image is not SRGB".to_string(),
      ));
    }

    if new_pixels.len() != old_pixels.len() {
      return Err(Error::new(
        Status::InvalidArg,
        "Image dimensions or sizes do not match".to_string(),
      ));
    }

    let len = new_pixels.len();
    let mut diff_buffer = vec![0u8; len];

    for i in 0..len {
      diff_buffer[i] = new_pixels[i] ^ old_pixels[i];
    }

    let mut compressed_buffer = Vec::new();
    match lz4::compress_to_vec(&diff_buffer, &mut compressed_buffer, lz4::ACC_LEVEL_DEFAULT) {
      Ok(_) => Ok(compressed_buffer.into()),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("Compression failed: {}", e),
      )),
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn diff_two_qoi_images(
  new_image: Buffer,
  old_image: Buffer,
  signal: Option<AbortSignal>,
) -> AsyncTask<DiffImageTask> {
  match signal {
    Some(sig) => AsyncTask::with_signal(DiffImageTask(new_image, old_image), sig),
    None => AsyncTask::new(DiffImageTask(new_image, old_image)),
  }
}
