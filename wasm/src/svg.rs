use resvg::render;
use resvg::tiny_skia::{IntSize, Pixmap};
use resvg::usvg::{Options, Size, Transform, Tree};

use crate::error::ConvertError;
use crate::settings::SvgSettings;

pub(crate) fn svg_to_png(svg: &[u8], svg_settings: SvgSettings) -> Result<Vec<u8>, ConvertError> {
    let SvgSettings { width, height } = svg_settings;

    let options = Options {
        dpi: 96.0,
        shape_rendering: resvg::usvg::ShapeRendering::GeometricPrecision,
        text_rendering: resvg::usvg::TextRendering::OptimizeLegibility,
        image_rendering: resvg::usvg::ImageRendering::OptimizeQuality,
        default_size: Size::from_wh(width as f32, height as f32).unwrap(),
        ..Default::default()
    };

    let tree = Tree::from_data(svg, &options)?;

    let original_svg_size = tree.size().to_int_size();
    let original_svg_size_f32 = original_svg_size.to_size();
    let fit_to_size = IntSize::from_wh(width, height).map(|s| original_svg_size.scale_to(s));
    let scaled_svg_size = match fit_to_size {
        Some(v) => v.to_size(),
        None => original_svg_size.to_size(),
    };

    let transform = Transform::from_scale(
        scaled_svg_size.width() / original_svg_size_f32.width(),
        scaled_svg_size.height() / original_svg_size_f32.height(),
    );

    let mut pixmap = Pixmap::new(width, height)
        .ok_or(ConvertError::SvgError(resvg::usvg::Error::InvalidSize))?;

    render(&tree, transform, &mut pixmap.as_mut());

    pixmap
        .encode_png()
        .map_err(|e| ConvertError::EncodingError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, ImageFormat};

    #[test]
    fn test_rasterize_svg() {
        let svg = include_bytes!("../assets/test.svg");

        let result = svg_to_png(svg, SvgSettings::default()).unwrap();

        let img = image::load_from_memory_with_format(&result, ImageFormat::Png).unwrap();

        assert_eq!(img.dimensions(), (100, 100));
    }
}
