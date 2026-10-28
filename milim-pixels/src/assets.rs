use std::io;

use engine_core::core::AssetCache;

use crate::{FontAsset, GlyphCache, ImageAsset};

pub struct RenderAssets {
    pub fonts: AssetCache<FontAsset>,
    pub images: AssetCache<ImageAsset>,
    pub glyph_cache: GlyphCache,
}

impl RenderAssets {
    pub fn load_font(&mut self, path: &str) -> io::Result<()> {
        let font = FontAsset::new(path)?;
        self.fonts.insert(path, font);
        Ok(())
    }
    pub fn load_image(&mut self, path: &str) -> io::Result<()> {
        let image = ImageAsset::load_from_file(path)?;
        self.images.insert(path, image);
        Ok(())
    }
    pub fn load_and_resize_image(&mut self, path: &str, width: u32, height: u32) -> io::Result<()> {
        let image = ImageAsset::load_from_file_and_resize(path, width, height)?;
        self.images.insert(path, image);
        Ok(())
    }
    pub fn clear_assets(&mut self) {
        self.fonts.clear();
        self.images.clear();
    }
}

impl Default for RenderAssets {
    fn default() -> Self {
        Self {
            fonts: AssetCache::default(),
            images: AssetCache::default(),
            glyph_cache: GlyphCache::new(),
        }
    }
}
