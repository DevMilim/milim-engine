use milim_engine::prelude::*;

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
    fn draw(&mut self, ctx: &mut impl EngineApi, _blending: f32) {}
}

#[derive(Scene)]
pub enum MainScene {
    Main(Main),
}
fn main() {
    let mut app = App::<MainScene>::new(Main::new().into());
}
