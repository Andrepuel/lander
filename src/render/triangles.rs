use crate::geom::Mat3;

pub trait TriangleScene<T = TriangleSceneAttr> {
    type Context;

    fn new_scene(attr_pipeline: T) -> Self;
    fn attr_pipeline(&self) -> &T;
    fn triangles(&self, context: &Self::Context) -> Box<dyn Iterator<Item = Mat3> + '_>;
}

#[cfg(feature = "wgpu_render")]
pub type TriangleSceneAttr = super::wgpu::triangles::TriangleScene;
#[cfg(feature = "webgl")]
pub type TriangleSceneAttr = super::webgl::triangles::TriangleScene;
