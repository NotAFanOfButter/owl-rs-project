use winit::{
    dpi::LogicalSize,
    event::{Event,WindowEvent},
    event_loop::{EventLoop,ControlFlow},
    window::{WindowButtons,WindowBuilder},
};
use raw_gl_context::{GlConfig,GlContext};

use owl::prelude::*;

fn main() {
    env_logger::init();
    if let Err(e) = fallible_main() {
        log::error!("{e}");
    }
}

fn fallible_main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // Setup
    //

    let event_loop = EventLoop::new().expect("unable to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(960,540))
        .with_title("Opengl Shader Triangle")
        .with_enabled_buttons(WindowButtons::CLOSE)
        .build(&event_loop).expect("failed to create a window");
    // SAFETY: idk really, but calls into native windowing c libs
    //          & author appears to have done 0 safety checks
    let context = unsafe {
        GlContext::create(&window,
            GlConfig { version: (4,3), ..Default::default() })
        .expect("failed to create an OpenGL context")
    };
    // SAFETY: idk really, but calls into native windowing c libs
    //          & author appears to have done 0 safety checks
    unsafe { context.make_current() };
    owl::load_proc(&context);

    //
    // Vertices
    //

    #[allow(clippy::items_after_statements)]
    #[derive(ToByteVec, Clone)]
    struct Vertex {
        pos: [f32;2],
        colour: [u8;3],
    }
    let test_vertex = Vertex {pos: [0.0;2], colour: [0;3]};
    let vertices = vec![
        Vertex { pos: [0.5, -0.5], colour: [0,0,200] },
        Vertex { pos: [-0.5, -0.5], colour: [0,200,0] },
        Vertex { pos: [0.0, 0.5], colour: [200,0,0] },
    ];
    let vertex_buffer = owl::ArrayBuffer::new(vertices, owl::BufferUsage::StaticDraw)?;
    let index_buffer = owl::ElementBuffer::new(
        vec![0,1,2], owl::BufferUsage::StaticDraw, owl::IndexType::UnsignedInt)?;
    let vertex_array_object = owl::VertexArray::new()
        .with_indices(index_buffer)
        .with_input(
            owl::ThinInputAttribute::Float { name: "pos".to_owned(), glsl_type: owl::ThinFloatAttributeType::Vec2,
                data_format: owl::FloatVertexFormat::Size2 { normalise: false, data_type: owl::DataTypeUnsized::Float } },
            owl::AttributePointer { buffer: &vertex_buffer,
                stride: test_vertex.stride(),
                offset: test_vertex.field_offset(0).expect("has at least 1 field")
            }
        )?
        .with_input(
            owl::ThinInputAttribute::Integral { name: "colour".to_owned(), glsl_type: owl::IntegralAttributeType::UVec3,
                data_format: owl::IntegralVertexFormat::Size3(owl::IntegralDataType::UnsignedByte) },
            owl::AttributePointer {
                buffer: &vertex_buffer,
                stride: test_vertex.stride(),
                offset: test_vertex.field_offset(1).expect("has at least 2 fields")
            }
        )?;

    //
    // Shader Pipeline
    //
    let shader_program = owl::ShaderPipeline::new(430)?
        .inputs_from_vertex_array(&vertex_array_object)
        .pipe(owl::Pipe {
            targets: owl::PipeTargets::VertexFragment,
            attribute: owl::Attribute { name: "vertColour".to_owned(), glsl_type: owl::AttributeType::Vec3,
                length: owl::AttributeLength::Single }
            })
        // TODO: from file
        .vertex_body(r"
            void main() {
                vertColour = vec3(colour) / 255.0;
                gl_Position = vec4(pos, 0.0, 1.0);
            }
            ").expect("no nul bytes")
        .fragment_body(r"
            void main() {
                colour = vec4(vertColour, 1.0);
            }
            ",
            owl::Attribute { name: "colour".to_string(), glsl_type: owl::AttributeType::Vec4,
                length: owl::AttributeLength::Single })
            .expect("no nul bytes")
        .compile()?;
    
    let triangle = owl::Mesh { start: 0, count: 3, vertex_array: &vertex_array_object };

    event_loop.run(|event, elwt| {
        if let Event::WindowEvent {event, ..} = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    owl::screen::clear_colour(owl::Colour::greyscale_float(0.5));
                    if let Err(e) = triangle.draw(owl::DrawMode::Triangles, &shader_program) {
                        eprintln!("{e}");
                    }
                    context.swap_buffers();
                },
                _ => ()
            }
        }
    })?;
    Ok(())
}
