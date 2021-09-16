#![feature(generic_associated_types)]

pub mod geom;
pub mod inertia;
pub mod render;
pub mod ship;
pub mod world;

#[cfg(feature = "webgl")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "webgl")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn load() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
}

#[cfg(feature = "webgl")]
#[wasm_bindgen]
pub struct World(world::World<render::webgl::target::WebglRenderTarget>);
#[cfg(feature = "webgl")]
#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> World {
        let target = render::webgl::target::WebglRenderTarget::new(canvas);
        World(target.into())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.0.resize(width, height);
    }

    pub fn redraw(&mut self) {
        self.0.redraw()
    }

    pub fn control(&mut self, throttle: ship::Throttle, activate: bool) {
        self.0.control(throttle, activate);
    }
}
