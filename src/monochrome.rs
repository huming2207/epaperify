use std::io::Cursor;
use std::ops::Deref;

use image::imageops::dither;
use image::imageops::BiLevel;
use image::DynamicImage;
use image::ImageFormat;
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;

struct MonochromeConvertTask(Buffer, String);

#[napi]
impl Task for MonochromeConvertTask {
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
      Some(format_str) => format_str,
      None => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unknown image format: {}", self.1),
        ))
      }
    };

    let mut luma8_img = img.grayscale().into_luma8();
    dither(&mut luma8_img, &BiLevel);
    let output_img = DynamicImage::from(image::DynamicImage::ImageLuma8(luma8_img));
    let mut output_vec = Cursor::new(Vec::new());
    match output_img.write_to(&mut output_vec, format) {
      Ok(()) => return Ok(output_vec.into_inner().into()),
      Err(err) => return Err(Error::new(Status::Unknown, err.to_string())),
    };
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_monochrome(
  image: Buffer,
  format: Option<String>,
  signal: Option<AbortSignal>,
) -> AsyncTask<MonochromeConvertTask> {
  let actual_format = format.unwrap_or("png".to_string());
  match signal {
    Some(sig) => AsyncTask::with_signal(MonochromeConvertTask(image, actual_format), sig),
    None => AsyncTask::new(MonochromeConvertTask(image, actual_format)),
  }
}
