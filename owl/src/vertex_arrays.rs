pub enum AttributeType {
    Bool,
    Int,
    Float,
    Vec2,
    Vec3,
    Vec4,
    BVec2,
    BVec3,
    BVec4,
    IVec2,
    IVec3,
    IVec4,
    Mat2,
    Mat3,
    Mat4
}
/// Generic attribute with a name and glsl type,
/// used as part of an input, uniform, or pipe
pub struct Attribute {
    pub name: String,
    pub glsl_type: AttributeType,
}
/// An input to the shader pipeline, stored in a [VertexArray].
pub struct Input {
    index: u32,
    attribute: Attribute,
    buffer: &'a ArrayBuffer
}

pub struct VertexArray {
    id: u32,
    inputs: Vec<Input>,
}
