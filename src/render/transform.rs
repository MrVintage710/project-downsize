use std::borrow::BorrowMut;
use cgmath::{Matrix4, Quaternion, Vector3, SquareMatrix, Zero, Deg, Angle, Rad, BaseFloat, Transform as TransformMatrix};
use egui::emath::Numeric;
use crate::render::debug::{Debugable, UIRenderType};
use egui::{Grid, Ui};
use cgmath::Rotation3;
use crate::render::shader::{ShaderUniformHandler, Uniform, UniformValue};
use crate::util::math::wrap_vec3;
use crate::util::variable::UpdateVariable;

#[derive(Clone)]
pub struct Transform {
    pos : Vector3<f32>,
    scale: Vector3<f32>,
    rotation : Vector3<f32>,
    origin : Vector3<f32>,
    uniform_handler: Option<ShaderUniformHandler>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            pos: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0),
            uniform_handler: None
        }
    }
}
impl Transform {
    pub fn new<T>(pos: T, scale: T, rotation: T, origin: T)
        -> Self where T: Into<Vector3<f32>> {
        Transform {
            pos: pos.into(),
            scale: scale.into(),
            rotation: rotation.into(),
            origin: origin.into(),
            uniform_handler: None
        }
    }

    pub fn calc_mat(&self) -> Matrix4<f32> {
        let origin_mat = Matrix4::from_translation(self.origin);
        let transform_mat = Matrix4::from_translation(self.pos);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation_x_mat = Matrix4::from_angle_x(Deg(self.rotation.x));
        let rotation_y_mat = Matrix4::from_angle_y(Deg(self.rotation.y));
        let rotation_z_mat = Matrix4::from_angle_z(Deg(self.rotation.z));
        let mat =  transform_mat * rotation_x_mat * rotation_y_mat * rotation_z_mat  * scale_mat * origin_mat;
        mat
    }

    pub fn calc_cam_mat(&self) -> Matrix4<f32> {
        let origin_mat = Matrix4::from_translation(-self.origin);
        let transform_mat = Matrix4::from_translation(-self.pos);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation_x_mat = Matrix4::from_angle_x(Deg(self.rotation.x));
        let rotation_y_mat = Matrix4::from_angle_y(Deg(self.rotation.y));
        let rotation_z_mat = Matrix4::from_angle_z(Deg(self.rotation.z));
        let mat =  transform_mat * rotation_x_mat * rotation_y_mat * rotation_z_mat  * scale_mat * origin_mat;
        mat
    }

    pub fn set_pos<T>(&mut self, value : T) -> &mut Self where T: Into<Vector3<f32>> {
        self.pos = value.into();
        self.update_uniform();
        self
    }

    pub fn set_rot<T>(&mut self, value : T) -> &mut Self where T: Into<Vector3<f32>> {
        self.rotation = value.into();
        wrap_vec3(self.rotation.borrow_mut(), 0.0, 360.0);
        self.update_uniform();
        self
    }

    pub fn set_scale<T>(&mut self, value : T) -> &mut Self where T: Into<Vector3<f32>> {
        self.scale = value.into();
        self.update_uniform();
        self
    }

    pub fn add_pos<T>(&mut self, value : T) -> &mut Self where T : Into<Vector3<f32>> {
        self.pos += value.into();
        self.update_uniform();
        self
    }

    pub fn add_rot<T>(&mut self, value : T) -> &mut Self where T : Into<Vector3<f32>> {
        self.rotation += value.into();
        wrap_vec3(self.rotation.borrow_mut(), 0.0, 360.0);
        self.update_uniform();
        self
    }

    pub fn update_uniform(&self) {
        if self.uniform_handler.is_some() {
            self.uniform_handler.as_ref().unwrap().update_uniform(self.calc_mat())
        }
    }
}

impl Uniform for Transform {
    fn provide_handle(&mut self, handle: ShaderUniformHandler) {
        handle.update_uniform(self.calc_mat());
        unsafe { self.uniform_handler = Some(handle) }
    }
}

impl Debugable for Transform {
    fn debug(&mut self, ui: &mut Ui, enabled: bool) -> bool {
        let changed = ui.add_enabled_ui(enabled, |ui|{
            let mut grid = Grid::new("main_grid")
                .num_columns(2)
                .spacing([20.0, 4.0])
                .striped(true);

            grid.show(ui, |ui| {
                    ui.label("Position:");
                    let b1 = self.pos.debug(ui, true);
                    ui.end_row();

                    ui.label("Scale:");
                    let b2 = self.scale.debug(ui, true);
                    ui.end_row();

                    ui.label("Rotation:");
                    let b3 = self.rotation.debug(ui, true);
                    ui.end_row();

                    ui.label("Origin:");
                    let b4 = self.origin.debug(ui, true);
                    ui.end_row();

                    b1 || b2 || b3 || b4
                }
            ).inner
        }).inner;

        if changed {
            self.update_uniform()
        }

        changed
    }
}

///This a transform meant to control a camera. It wraps transform and inverts all incoming changes.
pub struct CameraTransform {

}