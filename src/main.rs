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

    let mut program = ShaderProgram::new(&gl)?;
    program.load_vertex_shader(&gl, "static_vert.glsl");
    program.load_fragment_shader(&gl, "static_frag.glsl");
    program.link(&gl);

    program.uniform("color_shift", Vector3::new(0.0, 0.0, 0.0));

    let texture = Texture::new(&gl, "copper_block.png");

    let fbo = FBO::new(&gl)?.with_texture_attachment(&gl, 32, 32, 0)?;

    texture.bind(&gl);
    program.bind(&gl);
    vao.bind(&gl);

    let mut transform = Transform::new();
    transform.pos.z = -1.0;

    let mut camera = Transform::new();

    let window_size = window.window().inner_size();
    let aspect_ratio =  (window_size.width as f32 / window_size.height as f32);
    println!("Aspect Ratio: {}", aspect_ratio);
    let perspective = perspective(Deg(90.0), aspect_ratio, 0.00001, 200.0);

    program.uniform("transform", transform);
    program.uniform_debug_type("transform", MUTABLE);

    program.uniform("perspective", perspective);

    program.uniform("camera", camera.get_inverted_mat());

    event_loop.run(move |event, _, control_flow| {
        let (test, list) = egui_glow.run(window.window(), |egui_ctx| {
            egui::SidePanel::left("side_panel").show(egui_ctx, |ui| {
                program.debug(ui, &UIRenderType::MUTABLE);
                camera.debug(ui, &UIRenderType::MUTABLE);
            });

            let window = egui::Window::new("Debug")
                .collapsible(false)
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .title_bar(false)
                .resizable(false);
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
                    program.bind(&gl);
                    program.uniform("camera", camera.get_inverted_mat());
                    program.update_uniforms(&gl);
                    fbo.bind(&gl);

                    texture.bind(&gl);
                    vao.render(&gl);
                    FBO::unbind(&gl);
                    gl.clear_color(0.0, 0.0, 0.0, 0.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    fbo.enable_color_attachment(&gl, 0);
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
