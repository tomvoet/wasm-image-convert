use std::io::Cursor;

use error::ConvertError;
use gloo_utils::format::JsValueSerdeExt;
use image::ImageFormat;
use js_sys::Uint8Array;
use settings::{Settings, SvgSettings};
use source_type::SourceType;
use svg::svg_to_png;
use wasm_bindgen::prelude::*;

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
            let svg_settings = match config {
                Some(Settings::Svg(settings)) => settings,
                _ => SvgSettings::default(),
            };
            let img = svg_to_png(file, svg_settings)?;
            image::load_from_memory_with_format(&img, ImageFormat::Png)?
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

/// Image pre-processing, to ensure that the image can be converted to the target format.
fn process_image(
    img: image::DynamicImage,
    source_type: Option<ImageFormat>,
    target_type: Option<ImageFormat>,
) -> image::DynamicImage {
    let target_type = target_type.unwrap_or(ImageFormat::Png);

    let img = match source_type {
        Some(ImageFormat::Hdr) => image::DynamicImage::ImageRgba8(img.to_rgba8()),
        _ => img,
    };

    match target_type {
        ImageFormat::Jpeg
        | ImageFormat::Qoi
        | ImageFormat::Farbfeld
        | ImageFormat::Pnm
        | ImageFormat::Tga => image::DynamicImage::ImageRgb8(img.to_rgb8()),
        ImageFormat::Ico => img.resize(256, 256, image::imageops::FilterType::Lanczos3),
        ImageFormat::OpenExr => image::DynamicImage::ImageRgba32F(img.to_rgba32f()),
        _ => img,
    }
}

#[wasm_bindgen(js_name = convertImage)]
/// Convert an image from one format to another.
/// # Arguments
/// * `file` - The image file to convert.
/// * `src_type` - The MIME type of the source image.
/// * `target_type` - The MIME type of the target image.
/// * `cb` - A callback function to report progress.
/// * `convert_settings` - Settings for the conversion.
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    cb: &js_sys::Function,
    convert_settings: &JsValue,
) -> Result<Uint8Array, JsValue> {
    let convert_settings = convert_settings
        .into_serde::<Option<Settings>>()
        .map_err(|e| {
            let e = ConvertError::ParseError(e);
            JsValue::from_str(e.to_string().as_str())
        })?;

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

    let img = load_image(
        &file,
        SourceType::from_mime_type(src_type),
        convert_settings,
    )
    .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(50.0),
        &JsValue::from_str("Processing image"),
    );

    let img = process_image(
        img,
        ImageFormat::from_mime_type(src_type),
        ImageFormat::from_mime_type(target_type),
    );

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(70.0),
        &JsValue::from_str("Converting image"),
    );

    let output = write_image(&img, ImageFormat::from_mime_type(target_type))
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let _ = cb.call2(
        &this,
        &JsValue::from_f64(100.0),
        &JsValue::from_str("Conversion complete"),
    );

    Ok(Uint8Array::from(output.as_slice()))
}
