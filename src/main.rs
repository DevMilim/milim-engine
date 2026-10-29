use milim_collision::CollisionPlugin;
use milim_engine::prelude::*;
use milim_pixels::{PixelsPlugin, RenderQueue};
use milim_winit::{
    input::{Input, InputPlugin, KeyCode},
    runner::WinitRunner,
};

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
    fn fixed_update(&mut self, ctx: &mut impl EngineApi, delta: f32) {
        let input = ctx.require_resource_mut::<Input>();

        let speed = 100.0;

        let direction =
            input.get_key_vector(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD);
        self.base.transform.position += direction * speed * delta;

        println!("Position: {:#?}", self.position());
    }
    fn draw(&mut self, ctx: &mut impl EngineApi, _blending: f32) {
        let render = ctx.require_resource_mut::<RenderQueue>();
        let position = self.global_position();
        render.draw_rect(
            Rect::new(position.x as i32, position.y as i32, 50, 50),
            Color::BLACK,
            Anchor::Center,
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
    app.add_plugin(PixelsPlugin::default())
        .add_plugin(InputPlugin)
        .add_plugin(CollisionPlugin::default());
    let _ = app.run(WinitRunner::default());
}
