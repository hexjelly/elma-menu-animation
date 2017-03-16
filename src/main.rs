#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(500, 500)
        .with_title(format!("Elma Menu bounce balls deluxe")).build_glium().unwrap();

    implement_vertex!(Vertex, position);

    let circle_detail = 30_u8;
    let circle_radius = 0.2_f64;
    let circle_center = [0_f64, 0_f64];
    let mut circle_vertices = vec![];
    let twice_pi = std::f64::consts::PI * 2.;

    circle_vertices.push(Vertex { position: [circle_center[0], circle_center[1]] });
    for n in 0..circle_detail + 1 {
        circle_vertices.push(Vertex { position: [circle_center[0] + (circle_radius * (n as f64 * twice_pi / circle_detail as f64).cos()),
                             circle_center[1] + (circle_radius * (n as f64 * twice_pi / circle_detail as f64).sin())] });
	}

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;

        void main() {
            color = vec4(0.49, 0.66, 0.46, 1);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &circle_vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
