use std::ops::IndexMut;
use cgmath::{BaseFloat, Vector3};
use egui::Ui;
use egui::WidgetType::DragValue;
use crate::render::debug::{debug_colorRBG, Debugable};
use crate::render::shader::{MultiUniform, ShaderResult, ShaderUniformHandler};

pub struct GlobalLighting {
    color : Vector3<f32>,
    direction : Vector3<f32>,
    ambient_min : f32,
    color_uniform : Option<ShaderUniformHandler>,
    direction_uniform : Option<ShaderUniformHandler>,
    ambient_min_uniform : Option<ShaderUniformHandler>
}

impl  GlobalLighting {
    pub fn new<T>(color : T, direction : T, ambient_min : f32) -> Self where T : Into<Vector3<f32>> {
        GlobalLighting {
            color : color.into(),
            direction : direction.into(),
            ambient_min,
            .. Self::default()
        }
    }

    pub fn set_color<T>(&mut self, color: T) where T : Into<Vector3<f32>> {
        self.color = color.into();
        self.update_shader();
    }

    fn update_shader(&self) {
        if self.color_uniform.is_some() {
            self.color_uniform.as_ref().unwrap().update_uniform(self.color)
        }
    }
}

impl Default for GlobalLighting {
    fn default() -> Self {
        GlobalLighting {
            color: Vector3::new(1.0, 1.0, 1.0),
            direction: Vector3::new(0.0, -1.0, 1.0),
            ambient_min: 0.2,
            color_uniform: None,
            direction_uniform: None,
            ambient_min_uniform: None
        }
    }
}

impl MultiUniform for GlobalLighting {
    fn provide_handle_provider(&mut self, provider: impl Fn(&str) -> Option<ShaderUniformHandler>) {
        self.color_uniform = provider("global_light_color");
        self.update_shader();
    }
}

impl Debugable for GlobalLighting {
    fn debug(&mut self, ui: &mut Ui, enabled: bool) -> bool {
        let changed = debug_colorRBG(ui, enabled, &mut self.color);
        if changed {println!("Changed!"); self.update_shader()}
        changed
    }
}