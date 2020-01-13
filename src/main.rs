#[macro_use]
extern crate glium;
mod shapes;
use shapes::triangle::Triangle;

fn main() {
    use glium::glutin;
    use glium::Surface;

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let mut events_loop = glutin::EventsLoop::new();
    let dimensions = glutin::dpi::LogicalSize::new(200.0, 200.0);
    let wb = glutin::WindowBuilder::new().with_dimensions(dimensions);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let shape = Triangle::new(-0.5, -0.5, 0.0, 0.5, 0.5, -0.25);

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let mut closed = false;
    let mut holding_cmd = true;
    let mut t: f32 = -0.5;
    while !closed {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0f32],
            ]
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input {
                    glutin::KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(glutin::VirtualKeyCode::Escape), glutin::ElementState::Pressed) => {
                            closed = true
                        }
                        (Some(glutin::VirtualKeyCode::LWin), glutin::ElementState::Pressed) => {
                            println!("Holding cmd {}", holding_cmd);
                            holding_cmd = true;
                        }
                        (Some(glutin::VirtualKeyCode::LWin), glutin::ElementState::Released) => {
                            println!("Released cmd {}", holding_cmd);
                            holding_cmd = false;
                        }
                        (Some(glutin::VirtualKeyCode::W), glutin::ElementState::Pressed) => {
                            println!("Pressed W");
                            if holding_cmd {
                                closed = true;
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        });
    }
}
