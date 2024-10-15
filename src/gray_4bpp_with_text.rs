use std::collections::HashMap;
use std::io::BufWriter;
use std::ops::Deref;

use image::{self, imageops::*};
use image::{DynamicImage, Luma};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use png::Encoder;

#[derive(Clone, Copy)]
pub struct Gray4bppLevel;

impl ColorMap for Gray4bppLevel {
  type Color = Luma<u8>;

  #[inline(always)]
  fn index_of(&self, color: &Luma<u8>) -> usize {
    let luma = color.0;
    match luma[0] {
      0..=15 => 0,
      16..=31 => 1,
      32..=47 => 2,
      48..=63 => 3,
      64..=79 => 4,
      80..=95 => 5,
      96..=111 => 6,
      112..=127 => 7,
      128..=143 => 8,
      144..=159 => 9,
      160..=175 => 10,
      176..=191 => 11,
      192..=207 => 12,
      208..=223 => 13,
      224..=239 => 14,
      240..=255 => 15,
    }
  }

  #[inline(always)]
  fn lookup(&self, idx: usize) -> Option<Self::Color> {
    match idx {
      0 => Some([0].into()),
      1 => Some([16].into()),
      2 => Some([32].into()),
      3 => Some([48].into()),
      4 => Some([64].into()),
      5 => Some([80].into()),
      6 => Some([96].into()),
      7 => Some([112].into()),
      8 => Some([128].into()),
      9 => Some([144].into()),
      10 => Some([160].into()),
      11 => Some([176].into()),
      12 => Some([192].into()),
      13 => Some([208].into()),
      14 => Some([224].into()),
      15 => Some([240].into()),
      _ => None,
    }
  }

  /// Indicate NeuQuant implements `lookup`.
  fn has_lookup(&self) -> bool {
    true
  }

  #[inline(always)]
  fn map_color(&self, color: &mut Luma<u8>) {
    let new_color = self.lookup(self.index_of(color)).unwrap_or([240].into()).0[0];
    let luma = &mut color.0;
    luma[0] = new_color;
  }
}

struct Gray4bppWithTextConvertTask(Buffer, HashMap<String, String>, bool);

#[napi]
impl Task for Gray4bppWithTextConvertTask {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = (&self.0).deref();
    let buf = Vec::<u8>::from(input);
    let img =
      image::load_from_memory(&buf).map_err(|err| Error::new(Status::Unknown, err.to_string()))?;

    let mut luma8_img = img.grayscale().into_luma8();
    dither(&mut luma8_img, &Gray4bppLevel);
    let output_img = DynamicImage::from(image::DynamicImage::ImageLuma8(luma8_img)).to_luma8();

    let mut output_buf = Vec::new();

    {
      let writer = BufWriter::new(&mut output_buf);
      let (width, height) = output_img.dimensions();
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

      // Set color type to Grayscale (Luma8) and bit depth
      encoder.set_color(png::ColorType::Grayscale);
      encoder.set_depth(png::BitDepth::Eight);
      encoder.set_compression(png::Compression::Best);

      // Create the PNG writer and write the image data
      let mut png_writer = encoder
        .write_header()
        .map_err(|err| Error::new(Status::Unknown, err.to_string()))?;
      png_writer
        .write_image_data(&output_img)
        .map_err(|err| Error::new(Status::Unknown, err.to_string()))?;
    }

    Ok(output_buf.clone().into())
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_4bpp_with_text_metadata(
  image: Buffer,
  text_chunks: Option<HashMap<String, String>>,
  compressed: Option<bool>,
  signal: Option<AbortSignal>,
) -> AsyncTask<Gray4bppWithTextConvertTask> {
  let use_zext = compressed.unwrap_or(false);
  match signal {
    Some(sig) => AsyncTask::with_signal(
      Gray4bppWithTextConvertTask(image, text_chunks.unwrap_or(HashMap::new()), use_zext),
      sig,
    ),
    None => AsyncTask::new(Gray4bppWithTextConvertTask(
      image,
      text_chunks.unwrap_or(HashMap::new()),
      use_zext,
    )),
  }
}
