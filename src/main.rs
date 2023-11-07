use winit;

fn main() {
    let event_loop = match winit::event_loop::EventLoop::new() {
        Ok(lp) => lp,
        Err(_) => std::process::exit(0)
    };
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(600,400))
        .with_title("opengl-winit")
        .with_enabled_buttons(winit::window::WindowButtons::CLOSE)
        .build(&event_loop);

    event_loop.run(|event, elwt| {
        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested, ..
            } => elwt.exit(),
            _ => ()
        }
    }).unwrap();
}
