use engine_core::core::AssetCache;

use crate::{FontAsset, GlyphCache, ImageAsset};

pub struct RenderAssets {
    pub fonts: AssetCache<FontAsset>,
    pub images: AssetCache<ImageAsset>,
    pub glyph_cache: GlyphCache,
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
