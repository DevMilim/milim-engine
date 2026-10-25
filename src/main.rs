use milim_engine::prelude::*;
use sdl2_plugin::{DrawQueue, Sdl2Plugin, Sdl2Runner};

#[derive(GameObject)]
pub struct Counter {
    #[base]
    base: Base,
    value: u32,
}

impl Counter {
    fn new() -> Self {
        Self {
            base: Base::default(),
            value: 0,
        }
    }
}

impl GameObject for Counter {
    fn draw(&mut self, renderer: &mut impl EngineApi, _blending: f32) {
        let Some(queue) = renderer.get_resource_mut::<DrawQueue>() else {
            return;
        };
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
    Main(Counter),
}
fn main() {
    let _ = App::<MainScene>::new(Counter::new().into())
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
