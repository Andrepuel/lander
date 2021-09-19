use std::ops::DerefMut;

use crate::geom::Mat3;

pub trait RenderTarget {
    type RenderScene<T: RenderScene>: DerefMut<Target = T>;

    fn resize(&mut self, width: u32, height: u32);
    fn get_size(&self) -> (u32, u32);

    fn new_scene<R>(&mut self, scene: R) -> Self::RenderScene<R>
    where
        R: RenderScene;

    fn render_one<'a, R>(
        &'a mut self,
        scene: &'a mut Self::RenderScene<R>,
        context: R::Context<'a>,
    ) where
        R: RenderScene;
}

pub trait RenderScene {
    type Context<'a>;

    fn triangles<'a>(&'a self, context: Self::Context<'a>) -> Box<dyn Iterator<Item = Mat3> + 'a>;
}
