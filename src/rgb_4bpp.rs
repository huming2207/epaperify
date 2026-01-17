use std::io::Cursor;
use std::ops::Deref;

use image::{self, imageops::*};
use image::{DynamicImage, ImageFormat, Rgb};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;

#[derive(Clone, Copy)]
pub struct Rgb4bppLevel;

impl ColorMap for Rgb4bppLevel {
  type Color = Rgb<u8>;

  #[inline(always)]
  fn index_of(&self, color: &Rgb<u8>) -> usize {
    let rgb = color.0;
    let r = rgb[0] >> 4;
    let g = rgb[1] >> 4;
    let b = rgb[2] >> 4;
    ((r as usize) << 8) | ((g as usize) << 4) | (b as usize)
  }

  #[inline(always)]
  fn lookup(&self, idx: usize) -> Option<Self::Color> {
    if idx > 0xFFF {
      return None;
    }
    let r = ((idx >> 8) & 0xF) as u8 * 16;
    let g = ((idx >> 4) & 0xF) as u8 * 16;
    let b = (idx & 0xF) as u8 * 16;
    Some(Rgb([r, g, b]))
  }

  /// Indicate NeuQuant implements `lookup`.
  fn has_lookup(&self) -> bool {
    true
  }

  #[inline(always)]
  fn map_color(&self, color: &mut Rgb<u8>) {
    for c in &mut color.0 {
      *c = *c & 0xF0;
    }
  }
}

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

    let is_qoi = self.1.eq_ignore_ascii_case("qoi");
    let format = if !is_qoi {
      match ImageFormat::from_extension(&self.1) {
        Some(format_str) => Some(format_str),
        None => {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Unknown image format: {}", self.1),
          ))
        }
      }
    } else {
      None
    };

    let mut rgb8_img = img.into_rgb8();
    dither(&mut rgb8_img, &Rgb4bppLevel);

    if is_qoi {
      let width = rgb8_img.width();
      let height = rgb8_img.height();
      let data = rgb8_img.into_raw();
      match qoi::encode_to_vec(&data, width, height) {
        Ok(vec) => Ok(vec.into()),
        Err(e) => Err(Error::new(
          Status::GenericFailure,
          format!("QOI encoding failed: {}", e),
        )),
      }
    } else {
      let output_img = DynamicImage::from(image::DynamicImage::ImageRgb8(rgb8_img));
      let mut output_vec = Cursor::new(Vec::new());
      match output_img.write_to(&mut output_vec, format.unwrap()) {
        Ok(()) => Ok(output_vec.into_inner().into()),
        Err(err) => Err(Error::new(Status::Unknown, err.to_string())),
      }
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_rgb_4bpp(
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
