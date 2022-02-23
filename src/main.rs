mod render;
mod util;

use glow::*;
use crate::render::{createGlutinContext, buffer::VBO, Renderable, texture::Texture};
use cgmath::{Vector3, Vector2, Matrix4, SquareMatrix, Rad, Deg, perspective};
use crate::render::buffer::VAO;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::render::shader::ShaderProgram;
use std::time::SystemTime;
use crate::render::debug::{Debugable, UIRenderType};
use crate::render::debug::UIRenderType::*;
use crate::render::transform::Transform;

fn main() -> Result<(), String> {
    let (gl, shader_version, window, event_loop, mut egui_glow) = createGlutinContext("Hello Triangle!");

    let verts : Vec<Vector3<f32>> = vec![
        Vector3::new(-0.5, 0.5, 0.0),
        Vector3::new(0.5, 0.5,0.0),
        Vector3::new( -0.5, -0.5, 0.0),
        Vector3::new(0.5, -0.5, 0.0)
    ];

    let uvs : Vec<Vector2<f32>> = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0)
    ];

    let mut vert_vbo = VBO::new(&gl).unwrap();
    vert_vbo.load_vec3s(&gl, verts);

    let mut uv_vbo = VBO::new(&gl)?;
    uv_vbo.load_vec2s(&gl, uvs);

    let mut vao = VAO::new(&gl).unwrap();
    vao.addIndexBuffer(&gl, vec![0, 2, 1, 1, 2, 3]);
    vao.add_vbo(&gl ,0, &vert_vbo);
    vao.add_vbo(&gl, 1, &uv_vbo);

    let mut program = ShaderProgram::new(&gl)?;
    program.load_vertex_shader(&gl, "static_vert.glsl");
    program.load_fragment_shader(&gl, "static_frag.glsl");
    program.link(&gl);

    program.uniform("test", Vector3::new(0.0, 0.0, 0.0));
    program.uniform("color_shift", Vector3::new(0.0, 0.0, 0.0));

    let texture = Texture::new(&gl, "copper_block.png");

    texture.bind(&gl);
    program.bind(&gl);
    vao.bind(&gl);

    let mut transform = Transform::new();

    program.uniform("transform", transform);
    program.uniform_debug_type("transform", MUTABLE);

    event_loop.run(move |event, _, control_flow| {
        let (test, list) = egui_glow.run(window.window(), |egui_ctx| {
            egui::SidePanel::left("side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello World!");
                if ui.button("Quit").clicked() {
                    println!("Quit")
                }
                program.debug(ui, &UIRenderType::MUTABLE);
            });
        });

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { ref event, .. } => {
                egui_glow.on_event(event);
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
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    texture.bind(&gl);
                    program.bind(&gl);
                    program.update_uniforms(&gl);
                    vao.render(&gl);
                    egui_glow.paint(&window, &gl, list);
                    window.swap_buffers().unwrap();
                }
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {
                egui_glow.destroy(&gl);
                program.destroy(&gl);
                texture.destroy(&gl);
                vao.destroy(&gl);
                vert_vbo.destroy(&gl);
                uv_vbo.destroy(&gl);
            }
        }
    });

    Ok(())
}
