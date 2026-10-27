use engine_core::{
    core::Handler,
    math::{Color, Vector2},
};
use sdl2::render::Texture;

#[derive(Debug)]
pub enum DrawCommandType {
    Sprite,
    Rect,
}

pub struct DrawCommand {
    pub cmd_type: DrawCommandType,
    pub material: DrawData,
}

pub struct DrawData {
    pub pos: Vector2,
    pub size: Vector2,
    pub uv_min: Vector2,
    pub uv_max: Vector2,
    pub rotation: f32,
    pub color: Color,
    pub image: Handler<Texture>,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl Default for DrawData {
    fn default() -> Self {
        Self {
            pos: Vector2::ZERO,
            size: Vector2::new(32.0, 32.0),
            uv_min: Vector2::ZERO,
            uv_max: Vector2::ZERO,
            rotation: 0.0,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            image: Handler::new(0),
            flip_h: false,
            flip_v: false,
        }
    }
}
