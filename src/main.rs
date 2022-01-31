#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod render;
mod util;

use crate::render::*;
use cgmath::Vector3;
use crate::render::buffer::{VBO, VAO};
use crate::render::shader::ShaderProgram;
use egui_glow::glow::HasContext;

fn create_display(
    event_loop: &glutin::event_loop::EventLoop<()>,
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    glow::Context,
) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("egui_glow example");

    let gl_window = unsafe {
        glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };

    unsafe {
        use glow::HasContext as _;
        gl.enable(glow::FRAMEBUFFER_SRGB);
    }

    (gl_window, gl)
}

fn main() {
    let mut clear_color = [0.1, 0.1, 0.1];

    let (gl, version, gl_window, event_loop) = render::createGlutinContext("Test");

    let mut egui_glow = egui_glow::EguiGlow::new(&gl_window, &gl);

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

    let mut program = ShaderProgram::new(&gl).expect("Shader unable to be Created.");
    program.load_vertex_shader(&gl, "static_vert.glsl");
    program.load_fragment_shader(&gl, "static_frag.glsl");
    program.link(&gl);

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let mut quit = false;

            let (needs_repaint, list) = egui_glow.run(gl_window.window(), |egui_ctx| {
                egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                    ui.collapsing("Test", |ui|{
                        ui.label("Found Me!")
                    });

                    ui.heading("Hello World!");
                    if ui.button("Quit").clicked() {
                        quit = true;
                    }
                    ui.color_edit_button_rgb(&mut clear_color);
                });
            });

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                gl_window.window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                unsafe {
                    use glow::HasContext as _;
                    gl.clear_color(clear_color[0], clear_color[1], clear_color[2], 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                }

                program.bind(&gl);
                render::render(&gl, &vao);

                egui_glow.paint(&gl_window, &gl, list);

                // draw things on top of egui here
                //render::disable_shader_program(&gl);

                gl_window.swap_buffers().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::Resized(physical_size) = event {
                    gl_window.resize(physical_size);
                }

                egui_glow.on_event(&event);

                gl_window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                egui_glow.destroy(&gl);
                unsafe {
                    vao.destroy(&gl);
                    vbo.destroy(&gl);
                }

            }

            _ => (),
        }
    });
}