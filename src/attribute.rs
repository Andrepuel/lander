pub trait Attribute {
    type Attribute;

    fn from_attr(attr: Self::Attribute) -> Self;
    fn attr(&self) -> &Self::Attribute;
}
