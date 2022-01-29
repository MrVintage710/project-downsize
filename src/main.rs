mod render;
mod util;

use glow::*;
use crate::render::{createGlutinContext, buffer::VBO, Renderable};
use cgmath::Vector3;
use crate::render::buffer::VAO;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::render::shader::ShaderProgram;
use std::time::SystemTime;

fn main() -> Result<(), String> {
    let (gl, shader_version, window, event_loop) = createGlutinContext("Hello Triangle!");

    let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

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

    let mut program = ShaderProgram::new(&gl)?;
    program.load_vertex_shader(&gl, "static_vert.glsl");
    program.load_fragment_shader(&gl, "static_frag.glsl");
    program.link(&gl);

    program.create_uniform_vec3(&gl, "color_shift", Vector3::new(-0.1, -0.1, -0.1));

    vao.bind(&gl);

    let start = SystemTime::now();
    let mut last = SystemTime::now();

    let mut clear_color = [0.1, 0.1, 0.1];

    unsafe {
        gl.debug_message_callback(|m, e, s, u, message| {println!("{}", message)})
    }

    event_loop.run(move |event, _, control_flow| {
        let (test, list) = egui_glow.run(window.window(), |egui_ctx| {

        });

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { ref event, .. } => {
                egui_glow.on_event(&event);
                match event {
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                        unsafe { gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32); }
                    }
                    WindowEvent::CloseRequested => {
                        unsafe {
                            *control_flow = ControlFlow::Exit
                        }

                    }
                    _ => (),
                }
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {
                window.window().request_redraw()
            }
            Event::RedrawRequested(_) => {
                unsafe {

                    let total_time = start.elapsed().expect("Timing error").as_secs_f32();
                    program.bind(&gl);
                    program.uniform_vec3(&gl, "color_shift", Vector3::new(total_time.sin(), total_time.sin(), total_time.sin()));

                    gl.clear(glow::COLOR_BUFFER_BIT);
                    vao.pre_render(&gl);
                    vao.render(&gl);

                    if gl.get_error() != NO_ERROR {
                        println!("Error! {:?}", gl.get_debug_message_log(1024))
                    }

                    if test {egui_glow.paint(&window, &gl, list)}
                    window.swap_buffers().unwrap();
                }
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {
                egui_glow.destroy(&gl)
            }
        }
    });

    Ok(())
}
