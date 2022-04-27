use cgmath::{BaseNum, Matrix4, Vector3};
use egui::emath::Numeric;


pub fn wrap_vec3<T>(vec : &mut Vector3<T>, min : T, max : T) where T: BaseNum {
    if vec.x > max {
        vec.x -= max
    } else if vec.x < min {
        vec.x += max
    }

    if vec.y > max {
        vec.y -= max
    } else if vec.y < min {
        vec.y += max
    }

    if vec.z > max {
        vec.z -= max
    } else if vec.z < min {
        vec.z += max
    }
}