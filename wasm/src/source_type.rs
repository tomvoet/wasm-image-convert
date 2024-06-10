use image::ImageFormat;

pub enum SourceType {
    Raster(ImageFormat),
    Svg,
}

impl SourceType {
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match ImageFormat::from_mime_type(mime_type) {
            Some(format) => Some(Self::Raster(format)),
            None if mime_type == "image/svg+xml" => Some(Self::Svg),
            _ => None,
        }
    }
}
