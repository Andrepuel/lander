use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::render::render_target::{RenderScene, RenderTarget};

pub struct WebglRenderTarget {
    canvas: HtmlCanvasElement,
    context: WebGlRenderingContext,
}
impl WebglRenderTarget {
    pub fn new(canvas: HtmlCanvasElement) -> WebglRenderTarget {
        let context = canvas.get_context("webgl").unwrap();

        let context =
            context.unwrap_or_else(|| canvas.get_context("experimental-webgl").unwrap().unwrap());

        let context = context.dyn_into::<WebGlRenderingContext>().unwrap();

        WebglRenderTarget { canvas, context }
    }

    pub fn get_context(&self) -> &WebGlRenderingContext {
        &self.context
    }
}
impl<'a> RenderTarget for WebglRenderTarget {
    type RenderContext = WebGlRenderingContext;

    fn render_one<R: RenderScene<Self>>(&mut self, scene: &mut R, scene_context: &R::Context) {
        self.context.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        scene.render_one(scene_context, self, &self.context);
    }

    fn new_scene<R: RenderScene<Self>>(&mut self) -> R {
        R::new_scene(self)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    fn get_size(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }
}
