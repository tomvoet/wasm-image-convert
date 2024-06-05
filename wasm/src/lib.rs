use std::io::Cursor;

use image::{ImageError, ImageFormat};
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, throw_str};

#[derive(thiserror::Error, Debug)]
enum ConvertError {
    #[error("Unknown file type: {0}")]
    UnknownFileType(String),
    #[error("Image library error: {0}")]
    LibError(#[from] ImageError),
}

fn load_image(
    file: &[u8],
    file_type: Option<ImageFormat>,
) -> Result<image::DynamicImage, ConvertError> {
    let load = match file_type {
        Some(file_type) => image::load_from_memory_with_format(file, file_type)?,
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

#[wasm_bindgen]
/// Convert an image from one format to another.
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    cb: &js_sys::Function,
) -> Uint8Array {
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

    let img = match load_image(&file, ImageFormat::from_mime_type(src_type)) {
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
