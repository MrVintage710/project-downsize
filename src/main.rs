mod render;
mod util;

use glow::*;
use crate::render::{createGlutinContext, buffer::VBO, Renderable, texture::Texture};
use cgmath::{Vector3, Vector2, Matrix4, SquareMatrix, Rad, Deg, perspective};
use crate::render::buffer::{FBO, VAO};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::render::shader::{ShaderBuilder, UniformValue};
use std::time::{Instant, SystemTime};
use crate::render::debug::{Debugable, UIRenderType};
use crate::render::debug::UIRenderType::*;
use crate::render::transform::Transform;
use egui::Align2;
use crate::render::downsize::Downsize;
use crate::render::lighting::GlobalLighting;
use crate::render::shader::UniformValue::VEC3;

fn main() -> Result<(), String> {
    let (render_context, shader_version, event_loop, mut egui_glow) = createGlutinContext("Downsize");

    {
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

        let norm : Vec<Vector3<f32>> = vec![
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 0.0, 1.0),
        ];

        let mut vert_vbo = VBO::new(&render_context.gl).unwrap();
        vert_vbo.load_vec3s(&render_context.gl, verts);

        let mut uv_vbo = VBO::new(&render_context.gl)?;
        uv_vbo.load_vec2s(&render_context.gl, uvs);

        let mut norm_vbo = VBO::new(&render_context.gl)?;
        norm_vbo.load_vec3s(&render_context.gl, norm);

        let mut vao = VAO::new(&render_context.gl).unwrap();
        vao.addIndexBuffer(&render_context.gl, vec![0, 2, 1, 1, 2, 3]);
        vao.add_vbo(&render_context.gl ,0, &vert_vbo);
        vao.add_vbo(&render_context.gl, 1, &uv_vbo);
        vao.add_vbo(&render_context.gl, 2, &norm_vbo);

        let texture = Texture::new(&render_context.gl, "copper_block.png");

        let mut transform = Transform::new();

        let mut camera_transform = Transform::new();
        camera_transform.set_pos((0.0, 0.0, -1.0));

        let mut global_lighting = GlobalLighting::default();

        ///Testing new Shader Code
        let mut shdr = ShaderBuilder::new()
            .with_vert_shader("static_vert.glsl")
            .with_frag_shader("static_frag.glsl")
            .build(&render_context).expect("Unable to create shader.");

        shdr.add_uniform("camera", &mut camera_transform);
        shdr.add_uniform("transform", &mut transform);
        shdr.add_multi_uniform(&mut global_lighting);

        //shdr.send_uniform("global_light_color", (1.0, 1.0, 1.0)).expect("Test");

        global_lighting.set_color((1.0, 1.0, 1.0));

        let mut downsize = Downsize::new(&render_context.gl, 240);
        let mut should_animate = true;

        let mut last_frame_end = Instant::now();
        let mut current_frame_start = last_frame_end.elapsed();

        event_loop.run(move |event, test, control_flow| {
            let (test, list) = egui_glow.run(render_context.window.window(), |egui_ctx| {
                let window = egui::Window::new("Debug")
                    .collapsible(false)
                    .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                    .title_bar(false)
                    .resizable(false);

                window.show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("FPS: {:.2}", 1.0 / current_frame_start.as_secs_f64()));
                    });
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Pixel Density:");
                        downsize.debug(ui, true);
                    });
                    ui.collapsing("Camera Transform", |ui| {
                        camera_transform.debug(ui, true);
                    });
                    ui.collapsing("Plane Transform", |ui| {
                        transform.debug(ui, true);
                    });
                    global_lighting.debug(ui, true);

                    ui.separator();
                    ui.checkbox(&mut should_animate, "Should Animate")
                });
            });

            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { ref event, .. } => {
                    egui_glow.on_event(event);
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            render_context.window.resize(*physical_size);
                            unsafe { render_context.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32); }
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
                    render_context.window.window().request_redraw()
                }
                Event::RedrawRequested(_) => {
                    current_frame_start = last_frame_end.elapsed();
                    last_frame_end = Instant::now();

                    unsafe {
                        render_context.gl.clear(COLOR_BUFFER_BIT);

                        if should_animate {
                            transform.add_rot((0.0, 0.5, 0.0));
                        }

                        downsize.render(&render_context.gl, render_context.window.window().inner_size(), |gl, aspect_ratio| {
                            let pers = perspective(Deg(80.0), aspect_ratio, 0.00001, 200.0);
                            shdr.send_uniform("perspective", pers);

                            texture.bind(&render_context.gl);
                            shdr.bind();
                            unsafe { vao.render(&render_context.gl); }
                        });

                        egui_glow.paint(&render_context.window, &render_context.gl, list);
                        render_context.window.swap_buffers().unwrap();
                    }
                }
                Event::RedrawEventsCleared => {}
                Event::LoopDestroyed => {
                    egui_glow.destroy(&render_context.gl);
                    vao.destroy(&render_context.gl);
                    vert_vbo.destroy(&render_context.gl);
                    uv_vbo.destroy(&render_context.gl);
                    downsize.delete(&render_context.gl);
                    shdr.delete();
                }
            }
        });
    }

    Ok(())
}