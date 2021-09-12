pub mod geom;
pub mod inertia;
pub mod render;
pub mod ship;
pub mod world;

pub use world::World;

#[cfg(feature = "webgl")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn load() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
}
