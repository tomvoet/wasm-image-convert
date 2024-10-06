use image::ImageFormat;

pub enum SourceType {
    Raster(ImageFormat),
    Svg,
}

impl SourceType {
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match ImageFormat::from_mime_type(mime_type) {
            Some(format) => Some(Self::Raster(format)),
            None => from_custom_mime_type(mime_type),
        }
    }
}

/// This function is used to handle types that don't usually have an associated mime type
fn from_custom_mime_type(mime_type: &str) -> Option<SourceType> {
    match mime_type {
        // SVG
        "image/svg+xml" => Some(SourceType::Svg),
        // Farbfeld (often application/octet-stream is not inferred by image-rs)
        "image/farbfeld" => Some(SourceType::Raster(ImageFormat::Farbfeld)),
        _ => None,
    }
}
