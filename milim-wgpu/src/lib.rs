use std::{borrow::Cow, sync::Arc};

use engine_core::{core::CoreApi, plugin::Plugin};
use milim_winit::runner::{OwnedDisplayHandle, PhysicalSize, Window, WindowContext};
use wgpu::{
    CurrentSurfaceTexture, DeviceDescriptor, Instance, PresentMode::AutoNoVsync, RenderPipeline,
    RequestAdapterOptions, TextureUsages,
};

pub struct WgpuState {
    pub instance: wgpu::Instance,
    pub window: Arc<Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub render_pipeline: RenderPipeline,
    pub surface_format: wgpu::TextureFormat,
    pub size: PhysicalSize<u32>,
}

impl WgpuState {
    pub async fn new(display: Arc<OwnedDisplayHandle>, window: Arc<Window>) -> Self {
        let instance = Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(Box::new(
            display,
        )));
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .expect("Erro ao criar Adapter");
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .expect("Erro ao criar device e queue");
        let size = window.inner_size();

        let surface = instance
            .create_surface(window.clone())
            .expect("Erro ao criar surface");
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let mut state = WgpuState {
            instance,
            window,
            device,
            queue,
            surface,
            surface_format,
            render_pipeline,
            size,
        };
        state.configure_surface();
        state
    }
    fn configure_surface(&mut self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: AutoNoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }
}

pub fn spawn<T>(f: impl Future<Output = T> + 'static) -> T {
    pollster::block_on(f)
}
pub struct WgpuPlugin {
    state: Option<WgpuState>,
}

impl Default for WgpuPlugin {
    fn default() -> Self {
        Self { state: None }
    }
}

impl Plugin for WgpuPlugin {
    fn ready(&mut self, ctx: &mut engine_core::core::Context) {
        if self.state.is_none() {
            let window_context = ctx.require_resource_mut::<WindowContext>();
            let window = window_context.window.clone();
            let handler = window_context.handler.clone();
            self.state = Some(spawn(async { WgpuState::new(handler, window).await }));
        }
    }
    fn end_draw(&mut self, _ctx: &mut engine_core::core::Context, _blending: f32) {
        let wgpu_state = self.state.as_ref().unwrap();
        let CurrentSurfaceTexture::Success(_frame) = wgpu_state.surface.get_current_texture()
        else {
            return;
        };
    }
}
