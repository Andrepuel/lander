#[cfg(feature = "wgpu_render")]
pub type RenderTarget = super::wgpu::target::RenderTarget;
#[cfg(feature = "webgl")]
pub type RenderTarget = super::webgl::target::RenderTarget;
