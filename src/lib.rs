use after_effects as ae;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum AnyWorkflowEffectParams {
  MixChannels,
}


ae::define_effect!(AnyWorkflowGlobal, (), AnyWorkflowEffectParams);

#[derive(Debug)]
pub struct AnyWorkflowGlobalInit {
  // todo: this is where the python object should live
}

pub enum AnyWorkflowGlobal {
  Init(AnyWorkflowGlobalInit),
  Uninit,
}

impl AnyWorkflowGlobal {
  pub fn as_init(&self) -> Option<&AnyWorkflowGlobalInit> {
    match self {
      Self::Init(a) => Some(a),
      _ => None,
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnyWorkflowInstanceInit {}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnyWorkflowInstance {
  // Post initialization only fields
  #[serde(skip_serializing, skip_deserializing)]
  pub local_init: Option<AnyWorkflowInstanceInit>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub src: Option<String>,
}

after_effects::define_cross_thread_type!(AnyWorkflowInstance);

impl Default for AnyWorkflowGlobal {
  fn default() -> Self {
    // todo: create all the python objects here
    AnyWorkflowGlobal::Init(AnyWorkflowGlobalInit {})
  }
}

impl Drop for AnyWorkflowGlobal {
  fn drop(&mut self) {
    // todo: deconstruct all the python objects here since now the thing is unloaded
    ()
  }
}

impl AdobePluginGlobal for AnyWorkflowGlobal {
  fn can_load(_host_name: &str, _host_version: &str) -> bool {
    true
  }

  fn params_setup(&self, params: &mut ae::Parameters<AnyWorkflowEffectParams>, _in_data: InData, _out_data: OutData) -> Result<(), Error> {
    params.add(AnyWorkflowEffectParams::MixChannels, "Mix channels", ae::FloatSliderDef::setup(|f| {
      f.set_valid_min(0.0);
      f.set_slider_min(0.0);
      f.set_valid_max(200.0);
      f.set_slider_max(200.0);
      f.set_value(10.0);
      f.set_default(10.0);
      f.set_precision(1);
      f.set_display_flags(ae::ValueDisplayFlag::PERCENT);
    }))
  }

  fn handle_command(&mut self, cmd: ae::Command, in_data: ae::InData, mut out_data: ae::OutData, params: &mut ae::Parameters<AnyWorkflowEffectParams>) -> Result<(), ae::Error> {
    match cmd {
      ae::Command::About => {
        out_data.set_return_msg("Portable, v3.3\rThis example shows how to detect and respond to different hosts.\rCopyright 2007-2023 Adobe Inc.");
      }
      ae::Command::SequenceSetup => {
        // this proves we are interacting with Python correctly
        let _ = Python::with_gil(|py| -> PyResult<()> {
          let sys = py.import_bound("sys")?;
          let version: String = sys.getattr("version")?.extract()?;

          let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);
          let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
          let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;

          let message = format!("Hello {}, I'm Python {}", user, version);
          out_data.set_return_msg(&message);
          Ok(())
        });
      }
      ae::Command::Render { in_layer, mut out_layer } => {
        let slider_value = params.get(AnyWorkflowEffectParams::MixChannels)?.as_float_slider()?.value() as f32;

        // If the slider is 0 just make a direct copy.
        if slider_value < 0.001 {
          out_layer.copy_from(&in_layer, None, None)?;
        } else {
          let extent_hint = in_data.extent_hint();
          let out_extent_hint = out_layer.extent_hint();
          // clear all pixels outside extent_hint.
          if extent_hint != out_extent_hint {
            out_layer.fill(None, Some(out_extent_hint))?;
          }

          // iterate over image data.
          #[rustfmt::skip]
          in_layer.iterate_with(&mut out_layer, 0, extent_hint.height(), Some(extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
            let pixel = pixel.as_f32();

            // Mix the values. The higher the slider, the more we blend the channel with the average of all channels
            let average = (pixel.red + pixel.green + pixel.blue) / 3.0;
            // let midway_calc = (slider_value * average) + (200.0 - slider_value) * pixel.red;

            let r = ((slider_value * average) + (100.0 - slider_value) * pixel.red) / 100.0;
            let g = ((slider_value * average) + (100.0 - slider_value) * pixel.green) / 100.0;
            let b = ((slider_value * average) + (100.0 - slider_value) * pixel.blue) / 100.0;

            match out_pixel {
              ae::GenericPixelMut::Pixel8(out_pixel) => {
                out_pixel.alpha = pixel.alpha as _;
                out_pixel.red = r.min(ae::MAX_CHANNEL8 as f32) as _;
                out_pixel.green = g.min(ae::MAX_CHANNEL8 as f32) as _;
                out_pixel.blue = b.min(ae::MAX_CHANNEL8 as f32) as _;
              }
              ae::GenericPixelMut::Pixel16(out_pixel) => {
                out_pixel.alpha = pixel.alpha as _;
                out_pixel.red = r.min(ae::MAX_CHANNEL16 as f32) as _;
                out_pixel.green = g.min(ae::MAX_CHANNEL16 as f32) as _;
                out_pixel.blue = b.min(ae::MAX_CHANNEL16 as f32) as _;
              }
              _ => return Err(Error::BadCallbackParameter)
            }
            Ok(())
          })?;
        }
      }
      _ => {}
    }
    Ok(())
  }
}