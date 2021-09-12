use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub struct RenderTarget {
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,
}
impl RenderTarget {
    pub fn new(canvas: HtmlCanvasElement) -> RenderTarget {
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        RenderTarget { canvas, context }
    }

    pub fn render_one<R: RenderScene>(&mut self, scene: &mut R) {
        self.context.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        scene.render_one(&self.context);
    }

    pub fn new_scene<R: RenderScene>(&mut self) -> R {
        R::new_scene(&self.context)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.canvas.width(), self.canvas.height())
    }
}

pub trait RenderScene {
    fn new_scene(context: &WebGl2RenderingContext) -> Self;
    fn render_one(&mut self, context: &WebGl2RenderingContext);
}
