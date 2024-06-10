#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub(crate) enum Settings {
    #[serde(rename = "svg")]
    Svg(SvgSettings),
}

#[derive(serde::Deserialize)]
pub(crate) struct SvgSettings {
    pub width: u32,
    pub height: u32,
}

impl Default for SvgSettings {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}
