use std::io::Cursor;

use error::ConvertError;
use gloo_utils::format::JsValueSerdeExt;
use image::ImageFormat;
use js_sys::Uint8Array;
use settings::{Settings, SvgSettings};
use source_type::SourceType;
use svg::svg_to_png;
use wasm_bindgen::{prelude::*, throw_str};

mod error;
mod settings;
mod source_type;
mod svg;

fn load_image(
    file: &[u8],
    source_type: Option<SourceType>,
    config: Option<Settings>,
) -> Result<image::DynamicImage, ConvertError> {
    let load = match source_type {
        Some(SourceType::Raster(file_type)) => {
            image::load_from_memory_with_format(file, file_type)?
        }
        Some(SourceType::Svg) => {
            if let Some(Settings::Svg(svg_settings)) = config {
                let img = svg_to_png(file, svg_settings)?;
                image::load_from_memory_with_format(&img, ImageFormat::Png)?
            } else {
                let img = svg_to_png(file, SvgSettings::default())?;
                image::load_from_memory_with_format(&img, ImageFormat::Png)?
            }
        }
        None => image::load_from_memory(file)
            .map_err(|e| ConvertError::UnknownFileType(e.to_string()))?,
    };

    Ok(load)
}

fn write_image(
    img: &image::DynamicImage,
    file_type: Option<ImageFormat>,
) -> Result<Vec<u8>, ConvertError> {
    let mut output: Vec<u8> = Vec::new();

    let target_type = file_type.unwrap_or(ImageFormat::Png);

    img.write_to(&mut Cursor::new(&mut output), target_type)?;

    Ok(output)
}

fn process_image(
    img: image::DynamicImage,
    target_type: Option<ImageFormat>,
) -> image::DynamicImage {
    let target_type = target_type.unwrap_or(ImageFormat::Png);

    match target_type {
        ImageFormat::Jpeg => image::DynamicImage::ImageRgb8(img.to_rgb8()),
        ImageFormat::Ico => img.resize(256, 256, image::imageops::FilterType::Lanczos3),
        _ => img,
    }
}

#[wasm_bindgen(js_name = convertImage)]
/// Convert an image from one format to another.
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    cb: &js_sys::Function,
    convert_settings: &JsValue,
) -> Uint8Array {
    let convert_settings = if convert_settings.is_undefined() || convert_settings.is_null() {
        None
    } else {
        Some(convert_settings.into_serde::<Settings>().unwrap())
    };

    let this = JsValue::NULL;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(10.0),
        &JsValue::from_str("Starting conversion"),
    );
    let file = file.to_vec();

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(35.0),
        &JsValue::from_str("Loading image"),
    );

    let img = match load_image(
        &file,
        SourceType::from_mime_type(src_type),
        convert_settings,
    ) {
        Ok(img) => img,
        Err(e) => throw_str(e.to_string().as_str()),
    };

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(50.0),
        &JsValue::from_str("Processing image"),
    );

    let img = process_image(img, ImageFormat::from_mime_type(target_type));

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(70.0),
        &JsValue::from_str("Converting image"),
    );

    let output = match write_image(&img, ImageFormat::from_mime_type(target_type)) {
        Ok(output) => output,
        Err(e) => throw_str(e.to_string().as_str()),
    };

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(100.0),
        &JsValue::from_str("Conversion complete"),
    );

    Uint8Array::from(output.as_slice())
}
