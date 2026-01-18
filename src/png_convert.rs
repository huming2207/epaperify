use std::collections::HashMap;
use std::io::BufWriter;
use std::ops::Deref;

use image;
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use png::Encoder;

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

    let rgb8_img = img.into_rgb8();
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
fn to_png(
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
