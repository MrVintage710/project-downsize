mod render;
mod util;

use glow::*;
use crate::render::{createGlutinContext, buffer::VBO};
use cgmath::Vector3;
use crate::render::buffer::VAO;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

fn main() {
    let (gl, shader_version, window, event_loop) = createGlutinContext("Hello Triangle!");

    let verts : Vec<Vector3<f32>> = vec![
        Vector3::new(-0.5, 0.5, 0.0),
        Vector3::new(0.5, 0.5,0.0),
        Vector3::new( -0.5, -0.5, 0.0),
        Vector3::new(0.5, -0.5, 0.0)
    ];

    let mut vbo = VBO::new(&gl).unwrap();
    vbo.load_vec3s(&gl, verts);

    let mut vao = VAO::new(&gl).unwrap();
    vao.addIndexBuffer(&gl, vec![0, 2, 1, 1, 2, 3]);
    vao.add_vbo(&gl ,0, &vbo);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    window.resize(*physical_size);
                }
                WindowEvent::CloseRequested => {
                    unsafe {
                        *control_flow = ControlFlow::Exit
                    }

                }
                _ => (),
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                unsafe {
                    vao.bind(&gl);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.enable_vertex_attrib_array(0);
                    gl.draw_elements(glow::TRIANGLES, 6, UNSIGNED_INT,  0);
                    window.swap_buffers().unwrap();
                }
            }
            Event::RedrawEventsCleared => {
                window.window().request_redraw()
            }
            Event::LoopDestroyed => {}
        }
    })
}
