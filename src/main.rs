use milim_engine::prelude::*;
use sdl2_plugin::{DrawQueue, Sdl2Plugin, Sdl2Runner};

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
    fn draw(&mut self, renderer: &mut impl EngineApi, _blending: f32) {
        let queue = renderer.require_resource_mut::<DrawQueue>();
        queue.draw_rect(
            Vector2::new(400.0, 300.0),
            Vector2::new(32.0, 32.0),
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
    let _ = App::<MainScene>::new(Main::new().into())
        .add_plugin(Sdl2Plugin)
        .run(Sdl2Runner {
            title: "Hello".to_string(),
            width: 800,
            height: 600,
            scale: 1,
            vsync: true,
            quit_on_escape: true,
        });
}
