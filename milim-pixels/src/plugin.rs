use engine_core::{core::CoreApi, plugin::Plugin};
use milim_winit::runner::WindowContext;

use crate::{Render, RenderAssets, RenderQueue};

pub struct PixelsPlugin {
    pub render: Option<Render>,
}

impl Default for PixelsPlugin {
    fn default() -> Self {
        Self { render: None }
    }
}

impl Plugin for PixelsPlugin {
    fn build(&mut self, app: &mut engine_core::core::AppBuilder) {
        app.set_resource(RenderQueue::default());
        app.set_resource(RenderAssets::default());
    }
    fn ready(&mut self, ctx: &mut engine_core::core::Context) {
        let window_context = ctx.require_resource_mut::<WindowContext>();
        let window_size = window_context.size;
        let render = Render::new(
            &window_context.window,
            (window_size.width, window_size.height),
        );
        self.render = Some(render)
    }
    fn end_draw(&mut self, ctx: &mut engine_core::core::Context, _blending: f32) {
        if let Some(render) = &mut self.render {
            let (queue, resources) = ctx.require_query_mut::<(RenderQueue, RenderAssets)>();
            render.render_auto(queue, resources);
        }
    }
}
