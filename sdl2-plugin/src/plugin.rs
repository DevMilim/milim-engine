use std::collections::BTreeMap;

use engine_core::{
    core::{AppBuilder, AssetCache, Context, CoreApi, Handler},
    math::{Color, Rect, Vector2},
    plugin::Plugin,
};
use sdl2::{
    rect::Point,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub use sdl2::render::Texture;

use crate::{DrawCommand, DrawCommandType, DrawData};

pub struct DrawQueue {
    draw_queue: BTreeMap<i32, Vec<DrawCommand>>,
}

impl DrawQueue {
    pub fn new() -> Self {
        Self {
            draw_queue: BTreeMap::new(),
        }
    }
    pub fn draw(&mut self, command: DrawCommand, z_index: i32) {
        self.draw_queue.entry(z_index).or_default().push(command);
    }
    pub fn draw_sprite(&mut self, image: Handler<Texture>, pos: Vector2, z_index: i32) {
        let cmd = DrawCommand {
            cmd_type: DrawCommandType::Sprite,
            material: DrawData {
                pos,
                image,
                ..Default::default()
            },
        };
        self.draw(cmd, z_index);
    }
    pub fn draw_rect(
        &mut self,
        pos: Vector2,
        size: Vector2,
        color: engine_core::math::Color,
        z_index: i32,
    ) {
        let cmd = DrawCommand {
            cmd_type: DrawCommandType::Rect,
            material: DrawData {
                pos,
                size,
                color,
                ..Default::default()
            },
        };
        self.draw(cmd, z_index);
    }
}

pub struct Sdl2Plugin;

impl Plugin for Sdl2Plugin {
    fn build(&mut self, app: &mut AppBuilder) {
        app.set_resource(DrawQueue::new());
        app.set_resource(AssetCache::<Texture>::new());
    }
    fn begin_draw(&mut self, ctx: &mut Context, _blending: f32) {
        let Some(sdl2_context) = ctx.get_resource_mut::<Sdl2Context>() else {
            return;
        };
        sdl2_context
            .canvas
            .set_draw_color(sdl2::pixels::Color::BLACK);
        sdl2_context.canvas.clear();
    }
    fn end_draw(&mut self, ctx: &mut Context, _blending: f32) {
        let Some((sdl2_context, assets, draw_queue)) =
            ctx.get3_resources_mut::<Sdl2Context, AssetCache<Texture>, DrawQueue>()
        else {
            return;
        };
        let window_size = (800, 600);
        self.render(
            assets,
            draw_queue,
            window_size,
            Vector2::new(window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0),
            sdl2_context,
        );
        sdl2_context.canvas.present();
    }
}

pub struct Sdl2Context {
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Sdl2Plugin {
    pub fn render(
        &mut self,
        assets: &mut AssetCache<Texture>,
        draw_queue: &mut DrawQueue,
        window_size: (u32, u32),
        camera_pos: Vector2,
        sdl2_context: &mut Sdl2Context,
    ) {
        let window_size = Vector2::new(window_size.0 as f32, window_size.1 as f32);
        let half_screen = Vector2::new(window_size.x / 2.0, window_size.y / 2.0);

        for commands in draw_queue.draw_queue.values_mut() {
            for cmd in commands.drain(..) {
                let mat = &cmd.material;

                let screen_x =
                    (mat.pos.x - camera_pos.x + half_screen.x - (mat.size.x / 2.0)) as i32;
                let screen_y =
                    (mat.pos.y - camera_pos.y + half_screen.y - (mat.size.y / 2.0)) as i32;

                let dst_rect = Rect::new(screen_x, screen_y, mat.size.x as u32, mat.size.y as u32);
                match cmd.cmd_type {
                    DrawCommandType::Sprite => {
                        if let Some(texture) = assets.get_mut(mat.image) {
                            let query = texture.query();
                            let tex_w = query.width as f32;
                            let tex_h = query.height as f32;

                            let src_rect = if mat.uv_max.x > 0.0 && mat.uv_max.y > 0.0 {
                                let src_x = (mat.uv_min.x * tex_w) as i32;
                                let src_y = (mat.uv_min.y * tex_h) as i32;
                                let src_w = ((mat.uv_max.x - mat.uv_min.x) * tex_w) as u32;
                                let src_h = ((mat.uv_max.y - mat.uv_min.y) * tex_h) as u32;
                                Some(engine_rect_to_sdl2(Rect::new(src_x, src_y, src_w, src_h)))
                            } else {
                                None
                            };

                            texture.set_color_mod(mat.color.r, mat.color.g, mat.color.b);
                            texture.set_alpha_mod(mat.color.a);

                            let center = Vector2::new(mat.size.x / 2.0, mat.size.y / 2.0);

                            sdl2_context
                                .canvas
                                .copy_ex(
                                    texture,
                                    src_rect,
                                    Some(engine_rect_to_sdl2(dst_rect)),
                                    f64::from(mat.rotation),
                                    Some(Point::new(
                                        center.x.round() as i32,
                                        center.y.round() as i32,
                                    )),
                                    mat.flip_h,
                                    mat.flip_v,
                                )
                                .expect("Erro ao desenhar sprite");
                        }
                    }
                    DrawCommandType::Rect => {
                        sdl2_context
                            .canvas
                            .set_draw_color(engine_color_to_sdl2(mat.color));
                        sdl2_context
                            .canvas
                            .draw_rect(engine_rect_to_sdl2(dst_rect))
                            .unwrap();
                    }
                }
            }
        }
    }
}

pub fn engine_color_to_sdl2(color: Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a)
}

pub fn engine_rect_to_sdl2(rect: Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::from(rect.into_tuple())
}
