use milim_engine::prelude::*;

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
    fn start(&mut self, _ctx: &mut impl EngineApi) {
        println!("Counter iniciado");
    }

    fn update(&mut self, _ctx: &mut impl EngineApi, delta: f32) {
        self.value += 1;

        println!("Atualização: value={}, delta={delta}", self.value);
        if self.value == 5 {
            self.queue_free();
        }
    }
    fn fixed_update(&mut self, _ctx: &mut impl EngineApi, delta: f32) {
        println!("Atualização Fixa: value={}, delta={delta}", self.value);
    }
    fn destroy(&mut self, ctx: &mut impl EngineApi) {
        println!("Destroy");
        ctx.change_scene(MainScene::Main(Counter::new()));
    }
}

#[derive(Scene)]
pub enum MainScene {
    Main(Counter),
}
pub struct HeadlessRunner {
    pub dt: f32,
    pub max_steps: Option<u64>,
}

impl HeadlessRunner {
    pub fn new() -> Self {
        Self {
            dt: 0.01,
            max_steps: Some(20),
        }
    }
}

impl Runner for HeadlessRunner {
    fn run<S: Scene + 'static, P: GameObjectDispatch>(self, mut app: App<S, P>) -> Result<(), ()> {
        let mut n = 0u64;
        while app.run_step(self.dt) {
            n += 1;
            if self.max_steps == Some(n) {
                break;
            }
        }
        app.shutdown();
        Ok(())
    }
}

pub struct Log;

impl Plugin for Log {
    fn pre_update(&mut self, _ctx: &mut Context, _delta: f32) {
        println!("Log");
    }
    fn shutdown(&mut self, _ctx: &mut Context) {
        println!("shutdown")
    }
}

fn main() {
    let _ = App::<MainScene>::new(Counter::new().into())
        .add_plugin(Log)
        .run(HeadlessRunner::new());
}
