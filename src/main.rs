#[macro_use]
extern crate glium;
extern crate cgmath;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glium::{glutin, Surface, Rect, Frame};
use cgmath::*;

type Vec2 = Vector2<f32>;
type Vec4 = Vector4<f32>;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: Vec4,
}

impl From<Vertex> for Vec2 {
    fn from(item: Vertex) -> Self {
        vec2(item.position[0], item.position[1])
    }
}

implement_vertex!(Vertex, position);

pub fn main() {
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
    .with_title("Software Rasterizer")
    .with_inner_size(glutin::dpi::LogicalSize::new(800, 600));

    let context_builder = ContextBuilder::new();

    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let vertexes = [
        Vertex { position: [200.0, 200.0], color: vec4(1.0, 0.0, 0.0, 1.0) },
        Vertex { position: [400.0, 200.0], color: vec4(0.0, 1.0, 0.0, 1.0) },
        Vertex { position: [500.0, 400.0], color: vec4(0.0, 0.0, 1.0, 1.0) },
    ];

    event_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            }
            _ => (),
        }

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        draw_triangle(&mut frame, vertexes[0], vertexes[1], vertexes[2]);
        frame.finish().unwrap();
    });
}

fn draw_pixel(frame: &mut Frame, x: u32, y: u32, color: Vec4) {
    frame.clear(Some(&Rect {bottom: y, left: x, width: 1, height: 1}), Some((color.x, color.y, color.z, color.w)), true, None, None);
}

fn draw_triangle(frame: &mut Frame, v0: Vertex, v1: Vertex, v2: Vertex) {
    let min_x = f32::min(f32::min(v0.position[0], v1.position[0]), v2.position[0]).floor() as u32;
    let min_y = f32::min(f32::min(v0.position[1], v1.position[1]), v2.position[1]).floor() as u32;
    let max_x = f32::max(f32::max(v0.position[0], v1.position[0]), v2.position[0]).ceil() as u32;
    let max_y = f32::max(f32::max(v0.position[1], v1.position[1]), v2.position[1]).ceil() as u32;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let weight = get_barycentric(vec2(x as f32, y as f32), v0.into(), v1.into(), v2.into());
            if weight[0] >= 0.0 && weight[1] >= 0.0 && weight[2] >= 0.0 {
                draw_pixel(frame, x, y, vec4(weight[0], weight[1], weight[2], 1.0));
            }
        }
    }
}

#[inline]
fn edge_function(p0: Vec2, p1: Vec2, p2: Vec2) -> f32 {
    return (p2.x - p0.x) * (p1.y - p0.y) - (p2.y - p0.y) * (p1.x - p0.x);
}

fn get_barycentric(target: Vec2, p0: Vec2, p1: Vec2, p2: Vec2) -> [f32; 3] {
    let area = edge_function(p0, p1, p2);

    let w0 = edge_function(p1, p2, target) / area;
    let w1 = edge_function(p2, p0, target) / area;
    let w2 = 1.0 - w0 - w1;

    [w0, w1, w2]
}
