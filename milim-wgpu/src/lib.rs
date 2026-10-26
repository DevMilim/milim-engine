use std::sync::Arc;

use engine_core::plugin::Plugin;
use milim_winit::runner::Window;

enum WgpuState {
    Uninitialized,
    Loading,
    Running(WgpuData),
}

pub struct WgpuData {
    instance: wgpu::Instance,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
}
fn spawn(f: impl Future<Output = ()> + 'static) {
    pollster::block_on(f);
}
pub struct WgpuPlugin;

impl Plugin for WgpuPlugin {}
