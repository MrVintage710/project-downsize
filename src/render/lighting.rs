use std::ops::IndexMut;
use cgmath::{BaseFloat, Vector3};
use egui::{DragValue, Ui};
use crate::render::debug::{debug_colorRBG, Debugable};
use crate::render::shader::{MultiUniform, ShaderResult, ShaderUniformHandler};

pub struct GlobalLighting {
    color : Vector3<f32>,
    direction : Vector3<f32>,
    ambient: f32,
    color_uniform : Option<ShaderUniformHandler>,
    direction_uniform : Option<ShaderUniformHandler>,
    ambient_uniform: Option<ShaderUniformHandler>
}

impl  GlobalLighting {
    pub fn new<T>(color : T, direction : T, ambient: f32) -> Self where T : Into<Vector3<f32>> {
        GlobalLighting {
            color : color.into(),
            direction : direction.into(),
            ambient,
            .. Self::default()
        }
    }

    pub fn set_color<T>(&mut self, color: T) where T : Into<Vector3<f32>> {
        self.color = color.into();
        self.update_color_to_shader();
    }

    fn update_color_to_shader(&self) {
        if self.color_uniform.is_some() {
            self.color_uniform.as_ref().unwrap().update_uniform(self.color)
        }
    }

    fn update_ambient_to_shader(&self) {
        if self.ambient_uniform.is_some() {
            self.ambient_uniform.as_ref().unwrap().update_uniform(self.ambient)
        }
    }
}

impl Default for GlobalLighting {
    fn default() -> Self {
        GlobalLighting {
            color: Vector3::new(1.0, 1.0, 1.0),
            direction: Vector3::new(0.0, -1.0, 1.0),
            ambient: 0.2,
            color_uniform: None,
            direction_uniform: None,
            ambient_uniform: None
        }
    }
}

impl MultiUniform for GlobalLighting {
    fn provide_handle_provider(&mut self, provider: impl Fn(&str) -> Option<ShaderUniformHandler>) {
        self.color_uniform = provider("global_light_color");
        self.update_color_to_shader();
        self.ambient_uniform = provider("global_ambient");
        self.update_ambient_to_shader();
    }
}

impl Debugable for GlobalLighting {
    fn debug(&mut self, ui: &mut Ui, enabled: bool) -> bool {
        let color_changed = debug_colorRBG(ui, enabled, &mut self.color);
        if color_changed {println!("Changed!"); self.update_color_to_shader()}
        let ambient_changed = ui.add(DragValue::new(&mut self.ambient).speed(0.005).clamp_range(0.01..=1.0)).changed();
        if ambient_changed {self.update_ambient_to_shader()}
        color_changed || ambient_changed
    }
}