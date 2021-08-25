use raw_window_handle::HasRawWindowHandle;

pub struct RenderTarget {
    device: wgpu::Device,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    swapchain_format: wgpu::TextureFormat,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}
impl RenderTarget {
    pub fn new<T: HasRawWindowHandle>(window: &T) -> RenderTarget {
        pollster::block_on(Self::new_async(window))
    }

    pub async fn new_async<T: HasRawWindowHandle>(window: &T) -> RenderTarget {
        let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);
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
                    features: wgpu::Features::NON_FILL_POLYGON_MODE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        RenderTarget {
            device,
            sc_desc,
            swap_chain,
            swapchain_format,
            queue,
            surface,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.sc_desc.width = width;
        self.sc_desc.height = height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render_one<R: RenderScene>(&mut self, scene: &mut R) {
        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;

        scene.render_one(&self.device, &self.queue, &frame.view);
    }

    pub fn new_scene<R: RenderScene>(&mut self) -> R {
        R::new_scene(&self.device, &self.queue, self.swapchain_format)
    }
}

pub trait RenderScene {
    fn new_scene(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_format: wgpu::TextureFormat,
    ) -> Self;
    fn render_one(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: &wgpu::TextureView);
}
