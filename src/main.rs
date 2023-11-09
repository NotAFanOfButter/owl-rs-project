use winit;
use raw_gl_context;
use gl;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Should have created an event loop");
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(800,600))
        .with_title("opengl-winit")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop).expect("Failed to create a window");
    let context = unsafe { raw_gl_context::GlContext::create(&window,
        raw_gl_context::GlConfig { version: (3,3), profile: raw_gl_context::Profile::Core,
            red_bits: 8, blue_bits: 8, green_bits: 8, alpha_bits: 8, depth_bits: 24, stencil_bits: 8,
            samples: None, srgb: true, double_buffer: true, vsync: false })
        .expect("Failed to create an OpenGL context") };
    unsafe {
        context.make_current();
    }
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    event_loop.run(|event, elwt| {
        match event {
            winit::event::Event::WindowEvent {event, ..} => {
                match event {
                    winit::event::WindowEvent::CloseRequested => elwt.exit(),
                    winit::event::WindowEvent::RedrawRequested => {
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT);
                        }
                        context.swap_buffers();
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }).expect("Failed to run an event loop");
}
