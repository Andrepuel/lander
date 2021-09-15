pub trait RenderTarget {
    type RenderContext;

    fn resize(&mut self, width: u32, height: u32);
    fn get_size(&self) -> (u32, u32);

    fn new_scene<R>(&mut self) -> R
    where
        R: RenderScene<Self>;

    fn render_one<R>(&mut self, scene: &mut R, context: &R::Context)
    where
        R: RenderScene<Self>;
}

pub trait RenderScene<T: RenderTarget + ?Sized>
where
    Self: Sized,
{
    type Context;

    fn new_scene(target_context: &mut T) -> Self;
    fn render_one(
        &mut self,
        scene_context: &Self::Context,
        target_context: &T,
        render_context: &T::RenderContext,
    );
}
