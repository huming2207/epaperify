use std::ops::Deref;

use image::{self};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;

#[napi]
#[derive(Clone, Copy)]
pub enum QoiChannels {
  Rgb = 3,
  Rgba = 4,
}

struct ImageToQoiTask {
  image: Buffer,
  channels: QoiChannels,
}

#[napi]
impl Task for ImageToQoiTask {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = (&self.image).deref();
    // Copy to Vec to avoid V8 memory issues/race conditions if any, matching other files
    let buf = Vec::<u8>::from(input);

    let img = match image::load_from_memory(&buf) {
      Ok(image) => image,
      Err(err) => return Err(Error::new(Status::InvalidArg, err.to_string())),
    };

    let (width, height, mut data) = match self.channels {
      QoiChannels::Rgb => {
        let rgb = img.into_rgb8();
        (rgb.width(), rgb.height(), rgb.into_raw())
      }
      QoiChannels::Rgba => {
        let rgba = img.into_rgba8();
        (rgba.width(), rgba.height(), rgba.into_raw())
      }
    };

    let encoder = match qoi::Encoder::new(&mut data, width, height) {
      Ok(encoder) => encoder,
      Err(err) => return Err(Error::new(Status::GenericFailure, err.to_string())),
    };

    match encoder.encode_to_vec() {
      Ok(buf) => Ok(buf.into()),
      Err(err) => Err(Error::new(Status::GenericFailure, err.to_string())),
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn to_qoi(
  image: Buffer,
  channels: Option<QoiChannels>,
  signal: Option<AbortSignal>,
) -> AsyncTask<ImageToQoiTask> {
  let channels = channels.unwrap_or(QoiChannels::Rgb);

  match signal {
    Some(sig) => AsyncTask::with_signal(ImageToQoiTask { image, channels }, sig),
    None => AsyncTask::new(ImageToQoiTask { image, channels }),
  }
}
