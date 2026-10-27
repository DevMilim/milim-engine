use engine_sdl2::{DrawQueue, Sdl2Context, Sdl2Plugin, Sdl2Runner, Texture, TextureManager};
use milim_engine::prelude::*;
pub struct Sprite2D {
    position: Vector2,
    path: &'static str,
    texture: Option<Handler<Texture>>,
}

impl Sprite2D {
    pub fn new(position: Vector2, path: &'static str) -> Self {
        Self {
            position,
            path: path,
            texture: None,
        }
    }
}

impl Component for Sprite2D {
    fn start(&mut self, ctx: &mut impl EngineApi, _base: &mut Base) {
        let Some((textures, sdl2_context)) =
            ctx.query_resources_mut::<(TextureManager, Sdl2Context)>()
        else {
            return;
        };
        let texture = textures.load_image(sdl2_context, self.path).unwrap();
        self.texture = Some(texture);
    }
    fn draw(&mut self, renderer: &mut impl EngineApi, _base: &Base, _blending: f32) {
        let Some(texture) = self.texture else {
            return;
        };
        let queue = renderer.require_resource_mut::<DrawQueue>();
        queue.draw_sprite(texture, self.position, 0);
    }
}
#[derive(GameObject)]
pub struct Main {
    #[base]
    base: Base,
}

impl Main {
    fn new() -> Self {
        Self {
            base: Base::default(),
        }
    }
}

impl GameObject for Main {
    fn draw(&mut self, ctx: &mut impl EngineApi, _blending: f32) {
        let render = ctx.require_resource_mut::<DrawQueue>();
        render.draw_rect(
            Vector2::new(300.0, 400.0),
            Vector2::new(64.0, 64.0),
            Color::BLUE,
            0,
        );
    }
}

#[derive(Scene)]
pub enum MainScene {
    Main(Main),
}
fn main() {
    let mut app = App::<MainScene>::new(Main::new().into());
    app.add_plugin(Sdl2Plugin);
    let _ = app.run(Sdl2Runner {
        title: "Hello".to_string(),
        width: 800,
        height: 600,
        scale: 1,
        vsync: true,
        quit_on_escape: true,
    });
}
