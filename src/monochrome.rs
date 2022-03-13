use std::ops::Deref;

use image::DynamicImage;
use image::ImageFormat;
use image::imageops::dither;
use image::imageops::BiLevel;
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;

struct MonochromeConvertTask(Buffer);

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

    let mut luma8_img = img.grayscale().into_luma8();
    dither(&mut luma8_img, &BiLevel);
    let output_img = DynamicImage::from(image::DynamicImage::ImageLuma8(luma8_img));
    let mut output_vec: Vec<u8> = Vec::new();
    match output_img.write_to(&mut output_vec, ImageFormat::Png) {
      Ok(()) => return Ok(output_vec.into()),
      Err(err) => return Err(Error::new(Status::Unknown, err.to_string())),
    };
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_monochrome(image: Buffer) -> AsyncTask<MonochromeConvertTask> {
  AsyncTask::new(MonochromeConvertTask(image))
}

#[napi]
fn to_monochrome_abortable(image: Buffer, signal: AbortSignal) -> AsyncTask<MonochromeConvertTask> {
  AsyncTask::with_signal(MonochromeConvertTask(image), signal)
}
