use std::{fmt::Display, io::Cursor};

use image::{ImageError, ImageFormat, ImageOutputFormat};
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug)]
enum ConvertError {
    UnknownFileType(String),
    LibError(ImageError),
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::UnknownFileType(s) => write!(f, "Unknown file type: {}", s),
            ConvertError::LibError(e) => write!(f, "Image library error: {}", e),
        }
    }
}

impl From<image::ImageError> for ConvertError {
    fn from(e: image::ImageError) -> Self {
        ConvertError::LibError(e)
    }
}

impl From<&str> for ConvertError {
    fn from(e: &str) -> Self {
        ConvertError::UnknownFileType(e.to_owned())
    }
}

#[derive(Default)]
enum FileType {
    #[default]
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

fn load_image(
    file: &[u8],
    file_type: Option<FileType>,
) -> Result<image::DynamicImage, ConvertError> {
    let load = match file_type {
        Some(file_type) => image::load_from_memory_with_format(file, file_type.to_image_format())?,
        None => image::load_from_memory(file)
            .map_err(|e| ConvertError::UnknownFileType(e.to_string()))?,
    };

    Ok(load)
}

fn write_image(
    img: &image::DynamicImage,
    file_type: Option<FileType>,
) -> Result<Vec<u8>, ConvertError> {
    let mut output: Vec<u8> = Vec::new();

    let target_type = file_type.unwrap_or_default().to_image_output_format();

    img.write_to(&mut Cursor::new(&mut output), target_type)?;

    Ok(output)
}

#[wasm_bindgen]
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    /*cb: &js_sys::Function, */
) -> Uint8Array {
    let file = file.to_vec();

    let img = match load_image(&file, FileType::from_mime_type(src_type)) {
        Ok(img) => img,
        Err(e) => throw_str(e.to_string().as_str()),
    };

    let output = match write_image(&img, FileType::from_mime_type(target_type)) {
        Ok(output) => output,
        Err(e) => throw_str(e.to_string().as_str()),
    };

    Uint8Array::from(output.as_slice())
}
