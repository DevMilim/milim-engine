use milim_collision::CollisionPlugin;
use milim_engine::prelude::*;
use milim_pixels::{PixelsPlugin, RenderQueue};
use milim_winit::{input::InputPlugin, runner::WinitRunner};

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
        let render = ctx.require_resource_mut::<RenderQueue>();
        render.draw_rect(Rect::new(50, 50, 50, 50), Color::BLACK, Anchor::Center, 0);
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
        .add_plugin(CollisionPlugin);
    let _ = app.run(WinitRunner::default());
}
