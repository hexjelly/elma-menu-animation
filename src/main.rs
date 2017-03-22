#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Default, Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}

struct Ball {
    vertex: Vertex,
    radius: f64,
    velocity: [f64; 3],
    tris: u8,
}

impl Ball {
    fn new(position: [f64; 2], radius: f64, velocity: [f64; 3]) -> Ball {
        Ball {
            vertex: Vertex { position: position },
            radius: radius,
            velocity: velocity,
            tris: 50,
        }
    }

    fn update(&mut self) {
        // negate velocity if pos + radius is on the edges
        if (self.vertex.position[0] - self.radius <= -1.) || (self.vertex.position[0] + self.radius >= 1.) {
            self.velocity[0] *= -1.;
        }
        if (self.vertex.position[1] - self.radius <= -1.) || (self.vertex.position[1] + self.radius >= 1.) {
            self.velocity[1] *= -1.;
        }
        // update position from current velocity
        self.vertex.position[0] += self.velocity[0];
        self.vertex.position[1] += self.velocity[1];
    }

    fn render(&self) -> Vec<Vertex> {
        let mut vertices = vec![];

        vertices.push(self.vertex.clone());
        for n in 0..self.tris + 1 {
            vertices.push(Vertex {
                position: [&self.vertex.position[0] + (&self.radius * (n as f64 * PI2 / self.tris as f64).cos()),
                           &self.vertex.position[1] + (&self.radius * (n as f64 * PI2 / self.tris as f64).sin())],
            });
        }

        vertices
    }
}

const PI2: f64 = std::f64::consts::PI * 2.;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(640, 480)
        .with_title(format!("Elma Menu bounce balls deluxe"))
        .build_glium()
        .unwrap();

    implement_vertex!(Vertex, position);

    let rand_range = Range::new(0.028648_f64, 89.848719_f64);
    let mut rng_gen = rand::thread_rng();
    let rand_degree = rand_range.ind_sample(&mut rng_gen) * std::f64::consts::PI / 180.0; // deg to rad
    let mut circles = vec![];

    circles.push(Ball::new([-0.375, 0.5], 0.075, [rand_degree.cos() / 2000.0, rand_degree.sin() / 2000.0, 0.0]));
    circles.push(Ball::new([0.0, 0.5], 0.09375, [0.0002, 0.0002, 0.0]));
    circles.push(Ball::new([0.375, 0.5], 0.15625, [0.0001, 0.0003, 0.0]));
    circles.push(Ball::new([-0.375, 0.0], 0.075, [-0.0001, 0.0005, 0.0]));
    circles.push(Ball::new([0.0, 0.0], 0.09375, [0.0, -0.0005, 0.0]));
    circles.push(Ball::new([0.375, 0.0], 0.15625, [0.0008, -0.0001, 0.0]));
    circles.push(Ball::new([-0.375, -0.5], 0.075, [-0.0005, 0.0001, 0.0]));
    circles.push(Ball::new([0.0, -0.5], 0.09375, [0.0001, 0.0006, 0.0]));
    circles.push(Ball::new([0.375, -0.5], 0.15625, [-0.0009, -0.00009, 0.0]));

    let vertex_shader_src = include_str!("shaders/ball.vert");
    let fragment_shader_src = include_str!("shaders/ball.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for circle in &mut circles {
            circle.update();
            target.draw(&glium::VertexBuffer::new(&display, &circle.render()).unwrap(),
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
