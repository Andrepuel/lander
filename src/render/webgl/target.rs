use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::render::render_target::{RenderScene, RenderTarget};

use super::triangles::TriangleScene;

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
impl RenderTarget for WebglRenderTarget {
    type RenderScene<T: RenderScene> = TriangleScene<T>;

    fn resize(&mut self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    fn get_size(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }

    fn new_scene<R>(&mut self, scene: R) -> TriangleScene<R>
    where
        R: RenderScene,
    {
        TriangleScene::new(scene, &self.context)
    }

    fn render_one<R>(&mut self, scene: &mut TriangleScene<R>, scene_context: R::Context<'_>)
    where
        R: RenderScene,
    {
        self.context.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        scene.render_one(scene_context, &self.context);
    }
}
