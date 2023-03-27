use std::io::Cursor;

use image::{ImageFormat, ImageOutputFormat};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));

    log(&format!("Hello, {}!", name));
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
pub fn convert_image(file: &Uint8Array, src_type: &str, target_type: &str) -> Uint8Array {
    let file = file.to_vec();

    let src_type = FileType::from_mime_type(src_type)
        .unwrap()
        .to_image_format();

    let load = image::load_from_memory_with_format(&file, src_type);

    let img = load.unwrap();

    log(&format!("img: {:?}", img));

    let (width, height) = (img.width(), img.height());

    log(&format!("width: {}, height: {}", width, height));

    let mut output: Vec<u8> = Vec::new();

    let target_type = FileType::from_mime_type(target_type)
        .unwrap()
        .to_image_output_format();

    img.write_to(&mut Cursor::new(&mut output), target_type)
        .unwrap();

    Uint8Array::from(output.as_slice())
}
