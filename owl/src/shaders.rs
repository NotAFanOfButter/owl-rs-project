use std::ffi::CString;

use crate::{IntegralVertexFormat, FloatVertexFormat, AttributePointer, OwlError, VertexArray, DataTypeUnsized, DataTypeSize3, DataTypeSize4};
use crate::prelude::*;
use crate::ox;

/// Corresponds to a glsl type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    UVec2,
    UVec3,
    UVec4,
    Mat2,
    Mat3,
    Mat4
}
impl AttributeType {
    /// Number of attributes this type will take up
    pub(crate) const fn size(self) -> u8 {
        match self {
            Self::Int | Self::Bool | Self::Float | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::BVec2 | Self::BVec3 | Self::BVec4 | Self::IVec2 | Self::IVec3 | Self::IVec4 | Self::UVec2 | Self::UVec3 | Self::UVec4 => 1,
            Self::Mat2 => 2,
            Self::Mat3 => 3,
            Self::Mat4 => 4,
        }
    }
}
impl std::fmt::Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::Vec2 => "vec2",
            Self::Vec3 => "vec3",
            Self::Vec4 => "vec4",
            Self::BVec2 => "bvec2",
            Self::BVec3 => "bvec3",
            Self::BVec4 => "bvec4",
            Self::IVec2 => "ivec2",
            Self::IVec3 => "ivec3",
            Self::IVec4 => "ivec4",
            Self::UVec2 => "uvec2",
            Self::UVec3 => "uvec3",
            Self::UVec4 => "uvec4",
            Self::Mat2 => "mat2",
            Self::Mat3 => "mat3",
            Self::Mat4 => "mat4",
        };    
        write!(f, "{s}")
    }
}

/// Corresponds to a floating point (at least natively) glsl type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ThinFloatAttributeType {
    Bool,
    Float,
    Vec2,
    Vec3,
    Vec4,
    BVec2,
    BVec3,
    BVec4,
}
impl From<ThinFloatAttributeType> for AttributeType {
    fn from(value: ThinFloatAttributeType) -> Self {
        match value {
            ThinFloatAttributeType::Bool => Self::Bool,
            ThinFloatAttributeType::Float => Self::Float,
            ThinFloatAttributeType::Vec2 => Self::Vec2,
            ThinFloatAttributeType::Vec3 => Self::Vec3,
            ThinFloatAttributeType::Vec4 => Self::Vec4,
            ThinFloatAttributeType::BVec2 => Self::BVec2,
            ThinFloatAttributeType::BVec3 => Self::BVec3,
            ThinFloatAttributeType::BVec4 => Self::BVec4,
        }
    }
}

/// Corresponds to an integral glsl type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntegralAttributeType {
    Int,
    IVec2,
    IVec3,
    IVec4,
    UVec2,
    UVec3,
    UVec4,
}
impl From<IntegralAttributeType> for AttributeType {
    fn from(value: IntegralAttributeType) -> Self {
        match value {
            IntegralAttributeType::Int => Self::Int,
            IntegralAttributeType::IVec2 => Self::IVec2,
            IntegralAttributeType::IVec3 => Self::IVec3,
            IntegralAttributeType::IVec4 => Self::IVec4,
            IntegralAttributeType::UVec2 => Self::UVec2,
            IntegralAttributeType::UVec3 => Self::UVec3 ,
            IntegralAttributeType::UVec4 => Self::UVec4,
        }
    }
}

/// Generic attribute with a name and glsl type,
/// used as part of an input, uniform, or pipe
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub glsl_type: AttributeType,
}

#[derive(Debug, Clone)]
pub enum InputAttribute<'a, T: ToByteVec> {
    Thin(ThinInputAttribute, AttributePointer<'a, T>),
    Mat(MatInputAttributePointer<'a, T>)
}

#[derive(Debug, Clone)] // not necessarily unique, could be on different buffers ==> not Eq, Hash, etc...
pub enum ThinInputAttribute {
    Integral { name: String, glsl_type: IntegralAttributeType, data_format: IntegralVertexFormat },
    Float { name: String, glsl_type: ThinFloatAttributeType, data_format: FloatVertexFormat },
}
impl From<ThinInputAttribute> for Attribute {
    fn from(value: ThinInputAttribute) -> Self {
        match value {
            ThinInputAttribute::Integral { name, glsl_type, .. } =>
                Self { name, glsl_type: glsl_type.into() },
            ThinInputAttribute::Float { name, glsl_type, .. } =>
                Self { name, glsl_type: glsl_type.into() },
        }
    }
}

#[derive(Debug, Clone)] // not necessarily unique, could be on different buffers ==> not Eq, Hash, etc...
pub enum MatInputAttributePointer<'a, T: ToByteVec> {
    Mat2 { name: String, normalise: bool, pointers: [AttributePointer<'a,T>;2], data_type: DataTypeUnsized },
    Mat3 { name: String, normalise: bool, pointers: [AttributePointer<'a,T>;3], data_type: DataTypeSize3 },
    Mat4 { name: String, normalise: bool, pointers: [AttributePointer<'a,T>;4], data_type: DataTypeSize4 }
}
impl<'a, T: ToByteVec> MatInputAttributePointer<'a, T> {
    pub(crate) const fn size(&self) -> u8 {
        match self {
            Self::Mat2 { .. } => 2,
            Self::Mat3 { .. } => 3,
            Self::Mat4 { .. } => 4,
        }
    }
}
impl<'a, T: ToByteVec> From<MatInputAttributePointer<'a, T>> for Attribute {
    fn from(value: MatInputAttributePointer<'a, T>) -> Self {
        match value {
            MatInputAttributePointer::Mat2 { name, .. } => Self { name, glsl_type: AttributeType::Mat2 },
            MatInputAttributePointer::Mat3 { name, .. } => Self { name, glsl_type: AttributeType::Mat3 },
            MatInputAttributePointer::Mat4 { name, .. } => Self { name, glsl_type: AttributeType::Mat4 },
        }
    }
}

/// An input to the shader pipeline, stored in a [`VertexArray`].
#[derive(Clone, Debug, Hash)]
pub struct Input {
    index: u8,
    attribute: Attribute,
}
impl Input {
    // pub(crate) fn new<T: ToByteVec>(index: u8, attribute: Attribute, AttributePointer { buffer, stride, offset, format }: AttributePointer<T>) -> Self {
    //     buffer.bind();
    //     // buffer bound & index checked: shouldn't ever fail
    //     ox::vertex_attrib_pointer(index, format, stride, offset)
    //         .expect("buffer should be bound, and index checked");
    //     Self { index, attribute }
    // }
    /// Create a new non-matrix input
    pub(crate) fn new_thin<T: ToByteVec>(index: u8, attribute: ThinInputAttribute,
        AttributePointer { buffer, stride, offset }: AttributePointer<T>) -> Self {
        buffer.bind();
        match attribute {
            ThinInputAttribute::Integral { name, glsl_type, data_format } => {
                ox::vertex_attrib_i_pointer(index, data_format, stride.into(), offset.into())
                    .expect("buffer should be bound, and index checked");
                Self {
                    index, attribute: Attribute { name, glsl_type: glsl_type.into() }
                }
            },
            ThinInputAttribute::Float { name, glsl_type, data_format } => {
                ox::vertex_attrib_pointer(index, data_format, stride.into(), offset.into())
                    .expect("buffer should be bound, and index checked");
                Self {
                    index, attribute: Attribute { name, glsl_type: glsl_type.into() }
                }
            },
        }
    }
    pub(crate) fn new_mat<T: ToByteVec>(index: u8, attribute: MatInputAttributePointer<T>) -> Self {
        match attribute {
            MatInputAttributePointer::Mat2 { name, normalise, pointers, data_type } => {
                for p in pointers {
                    p.buffer.bind();
                    ox::vertex_attrib_pointer(index, FloatVertexFormat::Size2 { normalise, data_type },
                        p.stride.into(), p.offset.into())
                    .expect("buffer should be bound, and index checked");
                }
                Self {
                    index, attribute: Attribute { name, glsl_type: AttributeType::Mat2 }
                }
            },
            MatInputAttributePointer::Mat3 { name, normalise, pointers, data_type } => {
                for p in pointers {
                    p.buffer.bind();
                    ox::vertex_attrib_pointer(index, FloatVertexFormat::Size3 { normalise, data_type },
                        p.stride.into(), p.offset.into())
                    .expect("buffer should be bound, and index checked");
                }
                Self {
                    index, attribute: Attribute { name, glsl_type: AttributeType::Mat3 }
                }
            },
            MatInputAttributePointer::Mat4 { name, normalise, pointers, data_type } => {
                for p in pointers {
                    p.buffer.bind();
                    ox::vertex_attrib_pointer(index, FloatVertexFormat::Size4 { normalise, data_type },
                        p.stride.into(), p.offset.into())
                    .expect("buffer should be bound, and index checked");
                }
                Self {
                    index, attribute: Attribute { name, glsl_type: AttributeType::Mat4 }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PipeTargets {
    VertexFragment,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipe {
    pub targets: PipeTargets,
    pub attribute: Attribute,
}

/// A vertex shader and its (nul-terminated) source
#[derive(Debug, Clone, Hash)]
struct VertexShader {
    shader: ox::Shader,
    source: CString,
}
/// A fragment shader and its (nul-terminated) source
#[derive(Debug, Clone, Hash)]
struct FragmentShader {
    shader: ox::Shader,
    source: CString,
    output: Attribute
}

/// Newtype allowing for deletion on drop
pub struct Program(ox::ShaderProgram);

// INVARIANTS: only deleted on drop
impl Program {
    pub(crate) fn use_self(&self) -> Result<(),OwlError> {
        ox::use_program(self.0)
            .map_err(|e| {
                match e {
                    // cannot be deleted yet, so only possible error (I hope)
                    ox::OxError::BaseError(crate::OriginalError::InvalidOperation) =>
                        e.with_message("transform feedback mode active"),
                    _ => e.with_message("no other errors should be produced")
                }.with_context("using program failed")
            })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        ox::delete_program(self.0)
            .expect("program only deleted on drop");
    }
}

/// A representation of the shader pipeline as a whole, intended to be used as a builder, with the final stage ending in 'compile'
#[must_use]
#[derive(Debug)]
pub struct ShaderPipeline {
    version: u32,
    vertex: VertexShader,
    fragment: FragmentShader,
    inputs: Vec<Input>,
    pipes: Vec<Pipe>,
}

impl ShaderPipeline {
    /// # Errors
    ///
    /// This function will error if:
    /// * incorrect `glsl_version`, supported versions include 430; or
    /// * creation of either shader fails.
    pub fn new(glsl_version: u32) -> Result<Self,OwlError> {
        if ![430].contains(&glsl_version) {
            return Err(OwlError::custom("incorrect glsl version, accepted versions: 430"));
        }
        let vertex = ox::create_shader(ox::ShaderType::Vertex)
            .with_context("creating pipeline (vertex shader)")?;
        let fragment = ox::create_shader(ox::ShaderType::Fragment)
            .with_context("creating pipeline (fragment shader)")?;
        Ok(Self {
            version: glsl_version,
            vertex: VertexShader { shader: vertex, source: CString::default() },
            fragment: FragmentShader { shader: fragment, source: CString::default(),
                output: Attribute { name: String::default(), glsl_type: AttributeType::Vec4 } },
            inputs: Vec::new(),
            pipes: Vec::new(),
        })
    }
    pub fn inputs_from_vertex_array<T: ToByteVec>(mut self, vertex_array: &VertexArray<T>) -> Self {
        self.inputs = vertex_array.inputs.container.clone();
        self
    }
    /// # Errors
    ///
    /// This function will return an error if `source` contains nul bytes.
    pub fn vertex_body(mut self, source: &str) -> Result<Self,std::ffi::NulError> {
        self.vertex.source = CString::new(source)?;
        Ok(self)
    }
    /// # Errors
    ///
    /// This function will return an error if `source` contains nul bytes.
    pub fn fragment_body(mut self, source: &str, output: Attribute) -> Result<Self,std::ffi::NulError> {
        self.fragment = FragmentShader {
            source: CString::new(source)?, output,
            ..self.fragment
        };
        Ok(self)
    }
    pub fn pipe(mut self, pipe: Pipe) -> Self {
        self.pipes.push(pipe);
        self
    }
    /// # Errors
    ///
    /// This function will return an error if:
    /// * any shaders fail to compile; or
    /// * a new shader program cannot be created.
    pub fn compile(self) -> Result<Program,OwlError> {
        // add inputs to vertex code
        let version_prelude = format!("#version {} core\n", self.version);
        let vertex_source = {
            let input_to_glsl = |i: &Input| {
                format!("layout (location = {}) in {} {};\n", i.index, i.attribute.glsl_type, i.attribute.name)
            };
            let ins_prelude: String = self.inputs.iter().map(input_to_glsl).collect();
            let body = self.vertex.source.into_string().expect("created from &str, so valid UTF-8");
            #[allow(clippy::unnecessary_filter_map)] // more variants later will require filtering
            let pipes_prelude: String = self.pipes.iter()
                .filter_map(|Pipe { targets, attribute }| {
                    match targets {
                        PipeTargets::VertexFragment => Some(
                            format!("out {} {};\n", attribute.glsl_type, attribute.name)
                        )
                    }
            }).collect();
            CString::new(version_prelude.clone() + &ins_prelude + &pipes_prelude + &body)
                .expect("created from a collection of valid UTF-8 strings, so must be valid")
        };
        // println!("{}", vertex_source.clone().into_string().unwrap());
        let fragment_source = {
            #[allow(clippy::unnecessary_filter_map)] // more variants later will require filtering
            let pipes_prelude: String = self.pipes.iter()
                .filter_map(|Pipe { targets, attribute }| {
                    match targets {
                        PipeTargets::VertexFragment => Some(
                            format!("in {} {};\n", attribute.glsl_type, attribute.name)
                        )
                    }
            }).collect();
            let out_prelude = format!("out {} {};\n", self.fragment.output.glsl_type, self.fragment.output.name);
            let body = self.fragment.source.into_string().expect("created from &str, so valid UTF-8");
            CString::new(version_prelude + &pipes_prelude + &out_prelude + &body)
                .expect("created from a collection of valid UTF-8 strings, so must be valid")
        };
        // println!("{}", fragment_source.clone().into_string().unwrap());
        // compile shaders
        ox::shader_source(self.vertex.shader, &[vertex_source]).and(
            ox::shader_source(self.fragment.shader, &[fragment_source]))
            .expect("shaders not yet deleted");
        // shaders not yet deleted, so only ShaderErrors
        ox::compile_shader(self.vertex.shader).with_context("compiling pipeline (vertex shader)")?;
        ox::compile_shader(self.fragment.shader).with_context("compiling pipeline (fragment shader)")?;
        // link program
        let program = ox::create_program().with_context("compiling pipeline (shader program)")?;
        ox::attach_shader(program, self.vertex.shader)
            .expect("shader is neither deleted, nor already attached");
        ox::attach_shader(program, self.fragment.shader)
            .expect("shader is neither deleted, nor already attached");
        ox::link_program(program)
            .expect("program has not been deleted, is not active, nor is in transform feedback mode");
        ox::delete_shader(self.vertex.shader)
            .expect("shader is not deleted");
        ox::delete_shader(self.fragment.shader)
            .expect("shader is not deleted");
        Ok(Program(program))
    }
}
