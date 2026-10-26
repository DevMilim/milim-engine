use engine_core::core::{AssetCache, Handler};
use sdl2::render::Texture;

use crate::Sdl2Context;

pub struct TextureManager {
    assets: AssetCache<Texture>,
}
impl TextureManager {
    pub fn new() -> Self {
        Self {
            assets: AssetCache::new(),
        }
    }
    pub fn load_image(
        &mut self,
        sdl2_context: &mut Sdl2Context,
        path: &str,
    ) -> Result<Handler<Texture>, String> {
        use sdl2::image::LoadTexture;

        let texture = sdl2_context.texture_creator.load_texture(path)?;
        Ok(self.assets.insert(path, texture))
    }
    pub fn get_mut(&mut self, handler: Handler<Texture>) -> Option<&mut Texture> {
        self.assets.get_mut(handler)
    }
    pub fn get(&mut self, handler: Handler<Texture>) -> Option<&Texture> {
        self.assets.get(handler)
    }
    pub fn remove(&mut self, handler: Handler<Texture>) -> Option<Texture> {
        self.assets.remove(handler)
    }
}
