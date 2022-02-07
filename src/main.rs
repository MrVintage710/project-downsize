#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod render;
mod util;

use crate::render::*;
use cgmath::Vector3;
use crate::render::buffer::{VBO, VAO};
use crate::render::shader::ShaderProgram;
use egui_glow::glow::HasContext;
use anymap::AnyMap;
use crate::render::generic::{GenericDrawable, GenericDebug};

fn main() {
    let mut clear_color = [0.1, 0.1, 0.1];

    let (window, event_loop) = render::createGlutinContext("Test");
    let mut render_context = RenderContext::new(&window, &event_loop);

    let gl = &render_context.gl;

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

    let generic_drawable = GenericDrawable::new(move |gl| unsafe {
        program.bind(gl);
        vao.render(gl);
    });

    let generic_debug = GenericDebug::new(|ui| {
        ui.label("Test");
    });

    let rendergroup = RenderGroup::new("Test group".to_string(), 0)
        .with_drawable(generic_drawable)
        .with_debugable(generic_debug);

    render_context.add_render_group(rendergroup);

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
                render_context.render();
                render_context.debug(&window);
                window.swap_buffers().unwrap();
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
                    window.resize(physical_size);
                }

                render_context.on_event(&event);

                window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                render_context.destroy()
            }

            _ => (),
        }
    });
}