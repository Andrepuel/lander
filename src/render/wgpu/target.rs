use raw_window_handle::HasRawWindowHandle;
use wgpu::TextureViewDescriptor;

pub struct RenderTarget {
    device: wgpu::Device,
    sc_desc: wgpu::SurfaceConfiguration,
    swapchain_format: wgpu::TextureFormat,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}
impl RenderTarget {
    pub fn new<T: HasRawWindowHandle>(window: &T) -> RenderTarget {
        pollster::block_on(Self::new_async(window))
    }

    pub async fn new_async<T: HasRawWindowHandle>(window: &T) -> RenderTarget {
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

        RenderTarget {
            device,
            sc_desc,
            swapchain_format,
            queue,
            surface,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.sc_desc.width = width;
        self.sc_desc.height = height;
        self.surface.configure(&self.device, &self.sc_desc);
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.sc_desc.width, self.sc_desc.height)
    }

    pub fn render_one<R: RenderScene>(&mut self, scene: &mut R, context: &R::Context) {
        let frame = self
            .surface
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;

        scene.render_one(
            context,
            &self.device,
            &self.queue,
            &frame.texture.create_view(&TextureViewDescriptor::default()),
        );
    }

    pub fn new_scene<R: RenderScene>(&mut self) -> R {
        R::new_scene(&self.device, &self.queue, self.swapchain_format)
    }
}

pub trait RenderScene {
    type Context;

    fn new_scene(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_format: wgpu::TextureFormat,
    ) -> Self;

    fn render_one(
        &mut self,
        context: &Self::Context,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
    );
}
