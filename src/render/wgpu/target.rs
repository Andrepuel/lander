use raw_window_handle::HasRawWindowHandle;
use wgpu::TextureViewDescriptor;

use crate::render::render_target::{self, RenderScene};

pub struct WgpuRenderTarget {
    device: wgpu::Device,
    sc_desc: wgpu::SurfaceConfiguration,
    swapchain_format: wgpu::TextureFormat,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}
impl WgpuRenderTarget {
    pub fn new<T: HasRawWindowHandle>(window: &T) -> WgpuRenderTarget {
        pollster::block_on(Self::new_async(window))
    }

    pub async fn new_async<T: HasRawWindowHandle>(window: &T) -> WgpuRenderTarget {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let size = (1, 1);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let sc_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &sc_desc);

        WgpuRenderTarget {
            device,
            sc_desc,
            swapchain_format,
            queue,
            surface,
        }
    }

    pub fn get_init(&self) -> (&wgpu::Device, &wgpu::Queue, wgpu::TextureFormat) {
        (&self.device, &self.queue, self.swapchain_format)
    }
}
impl render_target::RenderTarget for WgpuRenderTarget {
    type RenderContext = wgpu::TextureView;

    fn resize(&mut self, width: u32, height: u32) {
        self.sc_desc.width = width;
        self.sc_desc.height = height;
        self.surface.configure(&self.device, &self.sc_desc);
    }

    fn get_size(&self) -> (u32, u32) {
        (self.sc_desc.width, self.sc_desc.height)
    }

    fn new_scene<R: RenderScene<Self>>(&mut self) -> R {
        R::new_scene(self)
    }

    fn render_one<R: RenderScene<Self>>(&mut self, scene: &mut R, context: &R::Context) {
        let frame = self
            .surface
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;

        scene.render_one(
            context,
            &self,
            &frame.texture.create_view(&TextureViewDescriptor::default()),
        );
    }
}
