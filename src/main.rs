use milim_engine::prelude::*;
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
    fn update(&mut self, _ctx: &mut impl EngineApi, _delta: f32) {
        println!("Running");
    }
    fn fixed_update(&mut self, ctx: &mut impl EngineApi, _delta: f32) {
        let input = ctx.require_resource_mut::<Input>();
        if input.is_key_pressed(KeyCode::KeyA) {
            println!("Key A")
        }
    }
}

#[derive(Scene)]
pub enum MainScene {
    Main(Main),
}
fn main() {
    let mut app = App::<MainScene>::new(Main::new().into());
    app.add_plugin(InputPlugin);
    let _ = app.run(WinitRunner::default());
}
