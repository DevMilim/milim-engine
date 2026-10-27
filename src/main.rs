use milim_engine::prelude::*;
use milim_wgpu::WgpuPlugin;
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
    fn draw(&mut self, _ctx: &mut impl EngineApi, _blending: f32) {}
}

#[derive(Scene)]
pub enum MainScene {
    Main(Main),
}
fn main() {
    let mut app = App::<MainScene>::new(Main::new().into());
    app.add_plugin(WgpuPlugin::default())
        .add_plugin(InputPlugin);
    let _ = app.run(WinitRunner::default());
}
