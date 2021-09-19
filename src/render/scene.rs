use crate::geom::{Mat3, Point};

use super::render_target::RenderScene;

pub struct Scene {
    camera: Mat3,
}
impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Mat3::identity(),
        }
    }

    pub fn set_camera(&mut self, position: Point, zoom: f32, window: (u32, u32)) {
        self.camera = Mat3::translate(-position.0, -position.1);
        self.camera = Mat3::scale(zoom, zoom) * self.camera;
        let aspect = (window.1 as f32) / (window.0 as f32);
        self.camera = Mat3::scale(aspect, 1.0) * self.camera;
    }
}
impl RenderScene for Scene {
    type Context<'a> = &'a mut [&'a mut dyn Drawable];
    type Triangles<'a> = impl Iterator<Item = Mat3> + 'a;

    fn triangles<'a>(&'a self, context: Self::Context<'a>) -> Self::Triangles<'a> {
        let r = context
            .into_iter()
            .map(move |drawable| {
                let position = drawable.position();
                drawable
                    .triangles()
                    .map(move |triangle| self.camera * position * triangle)
            })
            .flatten();

        Box::new(r)
    }
}

pub trait Drawable {
    fn position(&self) -> Mat3;
    fn triangles<'a>(&'a mut self) -> &'a mut (dyn Iterator<Item = Mat3> + 'a);
}
impl<T: Iterator<Item = Mat3>> Drawable for T {
    fn position(&self) -> Mat3 {
        Mat3::identity()
    }

    fn triangles<'a>(&'a mut self) -> &'a mut (dyn Iterator<Item = Mat3> + 'a) {
        self
    }
}
