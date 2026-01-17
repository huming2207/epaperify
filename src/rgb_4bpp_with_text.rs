use std::collections::HashMap;
use std::io::BufWriter;
use std::ops::Deref;

use image::Rgb;
use image::{self, imageops::*};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use png::Encoder;

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

struct Rgb4bppWithTextConvertTask(Buffer, HashMap<String, String>, bool, bool);

#[napi]
impl Task for Rgb4bppWithTextConvertTask {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = (&self.0).deref();
    let buf = Vec::<u8>::from(input);
    let img =
      image::load_from_memory(&buf).map_err(|err| Error::new(Status::Unknown, err.to_string()))?;

    let mut rgb8_img = img.into_rgb8();
    dither(&mut rgb8_img, &Rgb4bppLevel);

    let (width, height) = rgb8_img.dimensions();
    let mut output_buf = Vec::new();

    {
      let writer = BufWriter::new(&mut output_buf);
      let mut encoder = Encoder::new(writer, width, height);

      for (key, val) in self.1.clone() {
        if self.2 {
          match encoder.add_ztxt_chunk(key, val) {
            Ok(_) => (),
            Err(err) => return Err(Error::new(Status::Unknown, err.to_string())),
          };
        } else {
          match encoder.add_text_chunk(key, val) {
            Ok(_) => (),
            Err(err) => return Err(Error::new(Status::Unknown, err.to_string())),
          };
        }
      }

      // Set color type to RGB and bit depth
      encoder.set_color(png::ColorType::Rgb);
      encoder.set_depth(png::BitDepth::Eight);

      if self.3 {
        encoder.set_compression(png::Compression::High);
      } else {
        encoder.set_compression(png::Compression::Balanced);
      }

      // Create the PNG writer and write the image data
      let mut png_writer = encoder
        .write_header()
        .map_err(|err| Error::new(Status::Unknown, err.to_string()))?;

      png_writer
        .write_image_data(&rgb8_img)
        .map_err(|err| Error::new(Status::Unknown, err.to_string()))?;
    }

    Ok(output_buf.clone().into())
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_rgb_4bpp_with_text_metadata(
  image: Buffer,
  text_chunks: Option<HashMap<String, String>>,
  compressed_text: Option<bool>,
  best_compression: Option<bool>,
  signal: Option<AbortSignal>,
) -> AsyncTask<Rgb4bppWithTextConvertTask> {
  let use_zext = compressed_text.unwrap_or(false);
  let best_compress = best_compression.unwrap_or(false);
  match signal {
    Some(sig) => AsyncTask::with_signal(
      Rgb4bppWithTextConvertTask(
        image,
        text_chunks.unwrap_or(HashMap::new()),
        use_zext,
        best_compress,
      ),
      sig,
    ),
    None => AsyncTask::new(Rgb4bppWithTextConvertTask(
      image,
      text_chunks.unwrap_or(HashMap::new()),
      use_zext,
      best_compress,
    )),
  }
}
