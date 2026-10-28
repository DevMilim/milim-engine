use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use engine_core::{
    core::{App, Runner},
    object::{EmptyGlobals, GameObjectDispatch},
    world::Scene,
};
pub use winit::dpi::PhysicalSize;
pub use winit::event_loop::OwnedDisplayHandle;
pub use winit::window::Window;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

use crate::input::{Input, InputType};

pub struct WindowContext {
    pub window: Arc<Window>,
    pub handler: Arc<OwnedDisplayHandle>,
    pub size: PhysicalSize<u32>,
}

pub struct WinitApp<S: Scene + 'static, P: GameObjectDispatch = EmptyGlobals> {
    window: Option<Arc<Window>>,
    cfg: WinitRunner,
    app: Option<App<S, P>>,
    last: Instant,
    next_frame: Instant,
}

impl<S: Scene + 'static, P: GameObjectDispatch> ApplicationHandler for WinitApp<S, P> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes().with_inner_size(LogicalSize::new(800, 600));

        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        if let Some(app) = &mut self.app {
            app.set_resource::<WindowContext>(WindowContext {
                window: window.clone(),
                handler: Arc::new(event_loop.owned_display_handle()),
                size: window.inner_size(),
            });
            app.start_plugins();
        }

        self.last = Instant::now();
        self.window = Some(window)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last).as_secs_f32();
                self.last = now;
                if let Some(app) = &mut self.app {
                    if !app.run_step(dt) {
                        event_loop.exit();
                    }
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(app) = &mut self.app {
                    if let PhysicalKey::Code(keycode) = event.physical_key {
                        if let Some(input) = app.get_resource_mut::<Input>() {
                            input.update_input_state(
                                InputType::Key(keycode),
                                event.state.is_pressed(),
                            )
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(app) = &mut self.app {
                    if let Some(input) = app.get_resource_mut::<Input>() {
                        input.update_input_state(InputType::Mouse(button), state.is_pressed())
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(app) = &mut self.app {
                    if let Some(input) = app.get_resource_mut::<Input>() {
                        input.set_mouse_position(position.x as f32, position.y as f32);
                    }
                }
            }
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                if size.width == 0 || size.height == 0 {
                    return;
                }
                if let Some(window) = &self.window {
                    if let Some(app) = &mut self.app {
                        let window_size = app.require_resource_mut::<WindowContext>();
                        window_size.size = size
                    }
                    window.request_redraw();
                }
            }
            WindowEvent::Focused(false) => {
                if let Some(app) = &mut self.app {
                    if let Some(input) = app.get_resource_mut::<Input>() {
                        input.pressed_input.clear();
                    }
                }
            }
            _ => (),
        }
    }
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(window) = &self.window else {
            return;
        };
        if self.cfg.frame_limit.is_none() {
            event_loop.set_control_flow(ControlFlow::Poll);
            window.request_redraw();
            return;
        }
        let now = Instant::now();
        let frame_time = Duration::from_secs_f64(1.0 / self.cfg.frame_limit.unwrap() as f64);
        if now >= self.next_frame {
            window.request_redraw();

            self.next_frame += frame_time;
            if self.next_frame < now {
                self.next_frame = now + frame_time;
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

pub struct WinitRunner {
    pub frame_limit: Option<u32>,
}
impl Runner for WinitRunner {
    type Err = ();
    fn run<S, P>(self, app: App<S, P>) -> Result<(), ()>
    where
        S: Scene + 'static,
        P: GameObjectDispatch,
    {
        let el = EventLoop::new().unwrap();
        el.set_control_flow(ControlFlow::Poll);

        let mut winit_app = WinitApp {
            app: Some(app),
            cfg: self,
            window: None,
            last: Instant::now(),
            next_frame: Instant::now(),
        };
        el.run_app(&mut winit_app).map_err(|_| ())?;
        winit_app.app.unwrap().shutdown();
        Ok(())
    }
}

impl Default for WinitRunner {
    fn default() -> Self {
        Self {
            frame_limit: Some(60),
        }
    }
}
