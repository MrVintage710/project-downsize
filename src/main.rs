mod render;
mod util;

use glow::*;
use crate::render::{createGlutinContext, buffer::VBO, Renderable, texture::Texture, Deletable};
use cgmath::{Vector3, Vector2, Matrix4, SquareMatrix, Rad, Deg, perspective};
use crate::render::buffer::{FBO, VAO};
use glutin::event::{Event, MouseButton, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::render::shader::{ShaderBuilder, UniformValue};
use std::time::{Instant, SystemTime};
use crate::render::debug::{Debugable, UIRenderType};
use crate::render::debug::UIRenderType::*;
use crate::render::transform::Transform;
use egui::{Align2, Color32, Pos2, DragValue};
use crate::render::downsize::Downsize;
use crate::render::lighting::GlobalLighting;
use crate::render::model::OBJModel;
use crate::render::shader::UniformValue::VEC3F;
use crate::util::input::InputState;

fn main() -> Result<(), String> {
    let (render_context, shader_version, event_loop, mut egui_glow) = createGlutinContext("Downsize");
    let mut input = InputState::default();

    let texture = Texture::new(&render_context.gl, "copper_block.png");

    let mut transform = Transform::default();

    let mut camera_transform = Transform::default();
    camera_transform.set_pos((0.0, 0.0, -3.0));
    camera_transform.set_rot((35.264, 45.0, 0.0));

    let mut global_lighting = GlobalLighting::default();

    ///Testing new Shader Code
    let mut shdr = ShaderBuilder::new()
        .with_vert_shader("static_vert.glsl")
        .with_frag_shader("static_frag.glsl")
        .build(&render_context).expect("Unable to create shader.");

    shdr.add_uniform("camera", &mut camera_transform);
    shdr.add_uniform("transform", &mut transform);
    shdr.add_multi_uniform(&mut global_lighting);

    let model = OBJModel::new(&render_context, "torus.obj", shdr).expect("Could not load model");

    let mut downsize = Downsize::new(&render_context.gl, 240);
    let mut should_animate = false;

    let mut last_frame_end = Instant::now();
    let mut current_frame_start = last_frame_end.elapsed();
    let mut number_of_bands = 4.0;
    let mut use_texture = false;

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
                ui.horizontal(|ui| {
                    ui.label("Number of Bands:");
                    ui.add(DragValue::new(&mut number_of_bands).speed(1.0).clamp_range(1.0..=100.0))
                });
                ui.separator();
                ui.checkbox(&mut should_animate, "Should Animate");
                ui.checkbox(&mut use_texture, "Should Use Texture");
            });
        });

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { ref event, .. } => {
                egui_glow.on_event(event);
                input.update_state(event, None);
                // input.update_state(event, Some(egui_glow.egui_ctx.available_rect()));
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
                render_context.window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                current_frame_start = last_frame_end.elapsed();
                last_frame_end = Instant::now();

                unsafe {
                    render_context.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

                    if should_animate {
                        transform.add_rot_wrap((0.0, 0.5, 0.0), 0.0, 360.0);
                    }

                    if use_texture {
                        model.shader.send_uniform("use_texture", 1);
                    } else {
                        model.shader.send_uniform("use_texture", 0);
                    }

                    input.mouse.on_drag(MouseButton::Right, |x, y| {
                        camera_transform.add_rot_clamp_xz((y, x, 0.0), -90.0, 90.0);
                    });

                    input.mouse.on_scroll(|x, y| {
                        camera_transform.add_pos((0.0, 0.0, y));
                    });

                    downsize.render(&render_context.gl, render_context.window.window().inner_size(), |gl, aspect_ratio| {
                        let pers = perspective(Deg(80.0), aspect_ratio, 0.00001, 200.0);
                        model.shader.send_uniform("perspective", pers);
                        model.shader.send_uniform("transform", transform.clone());
                        model.shader.send_uniform("level_amout", number_of_bands);
                        texture.bind(&render_context.gl);
                        model.render(&render_context.gl)
                    });

                    egui_glow.paint(&render_context.window, &render_context.gl, list);
                    render_context.window.swap_buffers().unwrap();

                    input.update();
                }
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {
                egui_glow.destroy(&render_context.gl);
                //vao.destroy(&render_context.gl);
                //vert_vbo.destroy(&render_context.gl);
                //uv_vbo.destroy(&render_context.gl);
                downsize.delete(&render_context.gl);
                unsafe { model.delete(&render_context.gl) }
                //unsafe { shdr.delete(&render_context.gl); }
            }
        }
    });


    Ok(())
}