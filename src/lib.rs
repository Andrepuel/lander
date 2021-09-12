pub mod geom;
pub mod inertia;
pub mod render;
pub mod ship;
pub mod world;

use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;
pub use world::World;

#[wasm_bindgen(start)]
pub fn load() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
}
