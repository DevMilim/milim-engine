use std::time::{Duration, Instant};

use engine_core::core::Runner;
use sdl2::event::Event;

mod draw_command;
mod plugin;
mod textures;

pub use draw_command::*;
pub use plugin::*;
pub use textures::*;

pub struct Sdl2Runner {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub vsync: bool,
    pub quit_on_escape: bool,
}

impl Runner for Sdl2Runner {
    fn run<S, P>(self, mut app: engine_core::core::App<S, P>) -> Result<(), ()>
    where
        S: engine_core::world::Scene + 'static,
        P: engine_core::object::GameObjectDispatch,
    {
        let (w, h) = (self.width, self.height);

        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window(&self.title, self.width, self.height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let mut builder = window.into_canvas();

        if self.vsync {
            builder = builder.present_vsync();
        }
        let mut canvas = builder.build().unwrap();

        let _ = canvas.set_logical_size(w, h);

        let creator = canvas.texture_creator();

        let mut pump = sdl.event_pump().unwrap();
        let frame_time = Duration::from_secs_f64(1.0 / 60.0);
        let mut last = Instant::now();
        let mut next_frame = last + frame_time;

        app.set_resource(Sdl2Context {
            canvas,
            texture_creator: creator,
        });

        'running: loop {
            for ev in pump.poll_iter() {
                match ev {
                    Event::Quit { .. } => break 'running,

                    _ => {}
                }
            }
            let now = Instant::now();
            let dt = (now - last).as_secs_f32();
            last = now;

            if !app.run_step(dt) {
                break 'running;
            }

            if !self.vsync {
                let now = Instant::now();
                if next_frame > now {
                    std::thread::sleep(next_frame - now);
                }
                next_frame += frame_time;
                if next_frame < Instant::now() {
                    next_frame = Instant::now() + frame_time;
                }
            }
        }
        app.shutdown();
        Ok(())
    }
}
