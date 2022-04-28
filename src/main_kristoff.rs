use glow::*;
use crate::render::{createGlutinContext, buffer::VBO, Renderable, texture::Texture};
use cgmath::{Vector3, Vector2, Matrix4, SquareMatrix, Rad, Deg, perspective};
use crate::render::buffer::{FBO, VAO};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::render::shader::{ShaderProgram, UniformValue};
use std::time::SystemTime;
use crate::render::debug::{Debugable, UIRenderType};
use crate::render::debug::UIRenderType::*;
use crate::render::transform::Transform;
use egui::Align2;
use crate::render::downsize::Downsize;
use crate::render::model::OBJModel;
use crate::render::shader::UniformValue::TRANSFORM;


pub fn kristoff_main() -> Result<(), String> {

    let (gl, shader_version, window, event_loop, mut egui_glow) = createGlutinContext("Hello Cube!");
    println!("Hello Rust Graphics :)");
    let file_name = "cube.obj";
    let mut model = OBJModel::new(&gl, file_name)
        .expect("Expected object, no Object named {file_name} found");



    let window_size = window.window().inner_size();
    let aspect_ratio =  (window_size.width as f32 / window_size.height as f32);
    println!("Aspect Ratio: {}", aspect_ratio);
    let perspective_matrix = perspective(Deg(45.0), aspect_ratio, 0.00001, 200.0);

    let mut transform = Transform::new();
    transform.pos = (0.0, 0.0, -1.0).into();

    model.program.uniform("perspective", perspective_matrix);
    model.program.uniform("transform", transform);
    model.program.uniform_debug_type("transform", MUTABLE);

    let mut downsize = Downsize::new(&gl, 240);
    let mut should_animate = true;

    event_loop.run(move |event, test, control_flow| {
        let (test, list) = egui_glow.run(window.window(), |egui_ctx| {
            let window = egui::Window::new("Debug")
                .collapsible(false)
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .title_bar(false)
                .resizable(false);

            window.show(egui_ctx, |ui| {
                ui.label("Test");
                downsize.debug(ui, &UIRenderType::MUTABLE);
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
                    gl.clear(COLOR_BUFFER_BIT);

                    if should_animate {
                        model.program.mutate_uniform("transform", |uniform| {
                            match uniform {
                                TRANSFORM(t) => {t.rotation.y += 0.5}
                                _ => {}
                            }
                        });
                    }

                    downsize.render(&gl, window.window().inner_size(), |gl, aspect_ratio| {
                        let pers = perspective(Deg(90.0), aspect_ratio, 0.00001, 200.0);
                        model.program.uniform("perspective", pers);

                        model.update_uniforms(&gl);
                        unsafe { model.render(&gl); }
                    });

                    egui_glow.paint(&window, &gl, list);
                    window.swap_buffers().unwrap();
                }
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {
                egui_glow.destroy(&gl);
                model.vao.destroy(&gl);
                model.verts.destroy(&gl);
                model.uvs.destroy(&gl);
                model.program.destroy(&gl);
                downsize.destroy(&gl);
            }
        }
    });

    Ok(())
}