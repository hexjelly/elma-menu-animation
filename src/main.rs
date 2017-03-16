#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}

const PI2: f64 = std::f64::consts::PI * 2.;

fn make_circle(center: [f64; 2], radius: f64, triangle_count: u8) -> Vec<Vertex> {
    let mut circle_vertices = vec![];

    circle_vertices.push(Vertex { position: [center[0], center[1]] });
    for n in 0..triangle_count + 1 {
        circle_vertices.push(Vertex {
            position: [center[0] + (radius * (n as f64 * PI2 / triangle_count as f64).cos()),
                       center[1] + (radius * (n as f64 * PI2 / triangle_count as f64).sin())],
        });
    }

    circle_vertices
}

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(500, 500)
        .with_title(format!("Elma Menu bounce balls deluxe"))
        .build_glium()
        .unwrap();

    implement_vertex!(Vertex, position);

    let circles = vec![make_circle([0.0, 0.0], 0.4, 50), make_circle([0.7, 0.7], 0.2, 50)];

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

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut vbuffers = vec![];
    for circle in circles {
        vbuffers.push(glium::VertexBuffer::new(&display, &circle).unwrap());
    }
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for buffer in &vbuffers {
            target.draw(buffer,
                      &indices,
                      &program,
                      &glium::uniforms::EmptyUniforms,
                      &Default::default())
                .unwrap();
        }
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
