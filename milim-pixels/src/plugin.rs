use engine_core::{core::CoreApi, plugin::Plugin};
use milim_winit::runner::WindowContext;

use crate::{Render, RenderAssets, RenderQueue};

pub struct PixelsPlugin {
    pub render: Option<Render>,
    pub logical_resolution: (u32, u32),
}

impl Default for PixelsPlugin {
    fn default() -> Self {
        Self {
            render: None,
            logical_resolution: (320, 240),
        }
    }
}

impl Plugin for PixelsPlugin {
    fn build(&mut self, app: &mut engine_core::core::AppBuilder) {
        app.set_resource(RenderQueue::default());
        app.set_resource(RenderAssets::default());
    }
    fn ready(&mut self, ctx: &mut engine_core::core::Context) {
        let window_context = ctx.require_resource_mut::<WindowContext>();
        let render = Render::new(&window_context.window, self.logical_resolution);
        self.render = Some(render)
    }
    fn pre_update(&mut self, _ctx: &mut engine_core::core::Context, _delta: f32) {}
    fn end_draw(&mut self, ctx: &mut engine_core::core::Context, _blending: f32) {
        if let Some(render) = &mut self.render {
            let (queue, resources) = ctx.require_query_mut::<(RenderQueue, RenderAssets)>();
            render.render_auto(queue, resources);
        }
    }
}
