use std::io::Cursor;
use std::ops::Deref;

use image;
use image::{DynamicImage, ImageFormat};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;

struct Rgb4bppConvertTask(Buffer, String);

#[napi]
impl Task for Rgb4bppConvertTask {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = (&self.0).deref();
    let buf = Vec::<u8>::from(input);
    let img = match image::load_from_memory(&buf) {
      Ok(image) => image,
      Err(err) => return Err(Error::new(Status::Unknown, err.to_string())),
    };

    let format = match ImageFormat::from_extension(&self.1) {
      Some(format) => format,
      None => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unknown image format: {}", self.1),
        ))
      }
    };

    let rgb8_img = img.into_rgb8();

    let output_img = DynamicImage::from(image::DynamicImage::ImageRgb8(rgb8_img));
    let mut output_vec = Cursor::new(Vec::new());

    match output_img.write_to(&mut output_vec, format) {
      Ok(()) => Ok(output_vec.into_inner().into()),
      Err(err) => Err(Error::new(Status::Unknown, err.to_string())),
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_rgb_image(
  image: Buffer,
  format: Option<String>,
  signal: Option<AbortSignal>,
) -> AsyncTask<Rgb4bppConvertTask> {
  let actual_format = format.unwrap_or("png".to_string());
  match signal {
    Some(sig) => AsyncTask::with_signal(Rgb4bppConvertTask(image, actual_format), sig),
    None => AsyncTask::new(Rgb4bppConvertTask(image, actual_format)),
  }
}
