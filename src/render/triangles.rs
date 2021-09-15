use crate::{attribute::Attribute, geom::Mat3};

pub trait TriangleScene: Attribute {
    type Context;

    fn triangles(&self, context: &Self::Context) -> Box<dyn Iterator<Item = Mat3> + '_>;
}
