use anyhow::Result;
use gpui_kit::{AssetSource, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Assets;

// TODO: make zopra do this later
impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(file) = Assets::get(path) {
            return Ok(Some(file.data));
        }

        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut files = Assets::iter()
            .filter(|file| file.starts_with(path))
            .map(SharedString::from)
            .collect::<Vec<_>>();

        files.extend(gpui_kit::assets::Assets.list(path)?);

        Ok(files)
    }
}
