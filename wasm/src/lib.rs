use std::io::Cursor;

use image::{ImageFormat, ImageOutputFormat};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

enum FileType {
    PNG,
    JPEG,
    GIF,
    BMP,
    TIFF,
    WEBP,
    ICO,
    TGA,
}

impl FileType {
    fn from_mime_type(mime_type: &str) -> Option<Self> {
        match mime_type {
            "image/png" => Some(FileType::PNG),
            "image/jpeg" => Some(FileType::JPEG),
            "image/gif" => Some(FileType::GIF),
            "image/bmp" => Some(FileType::BMP),
            "image/tiff" => Some(FileType::TIFF),
            "image/webp" => Some(FileType::WEBP),
            "image/x-icon" => Some(FileType::ICO),
            "image/x-tga" => Some(FileType::TGA),
            _ => None,
        }
    }

    fn to_image_format(&self) -> ImageFormat {
        match self {
            FileType::PNG => ImageFormat::Png,
            FileType::JPEG => ImageFormat::Jpeg,
            FileType::GIF => ImageFormat::Gif,
            FileType::BMP => ImageFormat::Bmp,
            FileType::TIFF => ImageFormat::Tiff,
            FileType::WEBP => ImageFormat::WebP,
            FileType::ICO => ImageFormat::Ico,
            FileType::TGA => ImageFormat::Tga,
        }
    }

    fn to_image_output_format(&self) -> ImageOutputFormat {
        match self {
            FileType::PNG => ImageOutputFormat::Png,
            FileType::JPEG => ImageOutputFormat::Jpeg(100),
            FileType::GIF => ImageOutputFormat::Gif,
            FileType::BMP => ImageOutputFormat::Bmp,
            FileType::TIFF => ImageOutputFormat::Tiff,
            FileType::WEBP => ImageOutputFormat::Unsupported("WebP".to_owned()),
            FileType::ICO => ImageOutputFormat::Ico,
            FileType::TGA => ImageOutputFormat::Tga,
        }
    }
}

#[wasm_bindgen]
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    /*cb: &js_sys::Function, */
) -> Uint8Array {
    let file = file.to_vec();

    let load = match FileType::from_mime_type(src_type) {
        Some(file_type) => image::load_from_memory_with_format(&file, file_type.to_image_format()),
        None => {
            log("Unknown source type");
            image::load_from_memory(&file)
        }
    };

    let img = load.unwrap();

    log(&format!("img: {:?}", img));

    let (width, height) = (img.width(), img.height());

    log(&format!("width: {}, height: {}", width, height));

    let mut output: Vec<u8> = Vec::new();

    //let this = JsValue::null();
    ////cb.call1(&this, &JsValue::from_str(String::new("Hallooo")));
    //cb.call1(&this, &JsValue::from_str(&("testtaaaa".to_owned())))
    //    .unwrap();

    let target_type = match FileType::from_mime_type(target_type) {
        Some(file_type) => file_type.to_image_output_format(),
        None => FileType::PNG.to_image_output_format(),
    };

    img.write_to(&mut Cursor::new(&mut output), target_type)
        .unwrap();

    Uint8Array::from(output.as_slice())
}
