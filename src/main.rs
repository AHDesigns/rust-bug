#[macro_use]
extern crate glium;

use glium::glutin;
use glium::Surface;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.3, 0.0, 0.7, 1.0);
        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    })
}
