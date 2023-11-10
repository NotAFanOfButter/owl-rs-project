use winit::{self, event_loop};
use raw_gl_context;
use gl;

fn main() {
    use winit::{
        dpi::LogicalSize,
        event::{Event,WindowEvent},
        event_loop::EventLoop,
        window,
    };
    use raw_gl_context::{GlConfig,GlContext};

    let event_loop = EventLoop::new().expect("Should have created an event loop");
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    let window = window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800,600))
        .with_title("opengl-winit")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop).expect("Failed to create a window");
    let context = unsafe { GlContext::create(&window,
        GlConfig { version: (4,3), ..Default::default() })
        .expect("Failed to create an OpenGL context") };
    unsafe {
        context.make_current();
    }
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let vertices: [f32; 9] = [
        0.0, -0.5, 0.0,
        -0.5, 0.5, 0.0,
        0.5, 0.5, 0.0
    ];
    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao,0);
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
    
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
            (3 * std::mem::size_of::<f32>()).try_into().unwrap(),
            0 as *const std::ffi::c_void);
        gl::EnableVertexAttribArray(0);
    }
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        let vertex_shader_source: std::ffi::CString = std::ffi::CString::new(r#"
            #version 430 core
            layout (location = 0) in vec3 aPos;

            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
        "#).expect("Failed to convert shader source to c-string: vertex");
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);
    }
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        let fragment_shader_source = std::ffi::CString::new(r#"
            #version 430 core
            out vec4 fragColour;

            void main() {
                fragColour = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#).expect("Failed to covert shader source to c-string: fragment");
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);
    }
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
    }

    event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                            gl::UseProgram(shader_program);
                            gl::BindVertexArray(vao);
                            gl::DrawArrays(gl::TRIANGLES, 0, 3);
                        }
                        context.swap_buffers();
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }).expect("Failed to run an event loop");

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        gl::DeleteProgram(shader_program);
    }
}
