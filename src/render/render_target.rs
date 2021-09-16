use std::ops::DerefMut;

use crate::geom::Mat3;

pub trait RenderTarget {
    type RenderScene<T: RenderScene>: DerefMut<Target = T>;

    fn resize(&mut self, width: u32, height: u32);
    fn get_size(&self) -> (u32, u32);

    fn new_scene<R>(&mut self, scene: R) -> Self::RenderScene<R>
    where
        R: RenderScene;

    fn render_one<R>(&mut self, scene: &mut Self::RenderScene<R>, context: R::Context<'_>)
    where
        R: RenderScene;
}

pub trait RenderScene {
    type Context<'a>;

    fn triangles(&self, context: Self::Context<'_>) -> Box<dyn Iterator<Item = Mat3> + '_>;
}
