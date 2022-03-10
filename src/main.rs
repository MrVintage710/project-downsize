mod render;
mod util;

use glow::*;
use crate::render::{createGlutinContext, buffer::VBO, Renderable, texture::Texture};
use cgmath::{Vector3, Vector2, Matrix4, SquareMatrix, Rad, Deg, perspective};
use crate::render::buffer::{FBO, VAO};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::render::shader::ShaderProgram;
use std::time::SystemTime;
use crate::render::debug::{Debugable, UIRenderType};
use crate::render::debug::UIRenderType::*;
use crate::render::transform::Transform;
use egui::Align2;

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

    let texture = Texture::new(&gl, "copper_block.png");

    let mut program = ShaderProgram::new(&gl)?;
    program.load_vertex_shader(&gl, "static_vert.glsl");
    program.load_fragment_shader(&gl, "static_frag.glsl");
    program.link(&gl);

    //The fbo that we are saving
    let fbo = FBO::new(&gl)?
        .with_texture_attachment(&gl, 64, 64, 1)?;

    let window_size = window.window().inner_size();
    let aspect_ratio =  (window_size.width as f32 / window_size.height as f32);
    println!("Aspect Ratio: {}", aspect_ratio);
    let perspective = perspective(Deg(90.0), aspect_ratio, 0.00001, 200.0);

    let transform = Transform::new();
    transform.pos =

    program.uniform("perspective", perspective);
    program.uniform("")

    fbo.bind(&gl);
    texture.bind_index(&gl, 0);
    program.bind(&gl);



    FBO::unbind(&gl);



    event_loop.run(move |event, _, control_flow| {
        let (test, list) = egui_glow.run(window.window(), |egui_ctx| {
            let window = egui::Window::new("Debug")
                .collapsible(false)
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .title_bar(false)
                .resizable(false);

            window.show(egui_ctx, |ui| {
                ui.label("Test")
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
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    egui_glow.paint(&window, &gl, list);
                    window.swap_buffers().unwrap();
                }
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {
                egui_glow.destroy(&gl);
                vao.destroy(&gl);
                vert_vbo.destroy(&gl);
                uv_vbo.destroy(&gl);
            }
        }
    });

    Ok(())
}
