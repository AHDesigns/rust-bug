#[macro_use]
extern crate glium;

use glium::Surface;
use glium::{glutin, index, Display, Program};
use glutin::event::{Event, StartCause, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };

    let shape = vec![vertex1, vertex2, vertex3];

    let vbo = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    
    uniform float t;

    void main() {
        vec2 pos = position;
        pos.x += t;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;
    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("could not create program");

    let mut t: f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.46, 0.52, 0.64, 1.0);

        target
            .draw(
                &vbo,
                &indices,
                &program,
                &uniform! { t: t },
                &Default::default(),
            )
            .unwrap();

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
                _ => (),
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => (),
            },
            _ => (),
        }
    })
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
