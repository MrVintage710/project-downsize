use cgmath::{BaseFloat, Vector3};

pub struct GlobalLighting {
    color : Vector3<f32>,
    direction : Vector3<f32>,
    ambient_min : f32
}

impl  GlobalLighting {
    pub fn new<T>(color : T, direction : T, ambient_min : f32) -> Self where T : Into<Vector3<f32>> {
        GlobalLighting {
            color : color.into(),
            direction : direction.into(),
            ambient_min
        }
    }
}

impl Default for GlobalLighting {
    fn default() -> Self {
        GlobalLighting {
            color: Vector3::new(1.0, 1.0, 1.0),
            direction: Vector3::new(0.0, -1.0, 1.0),
            ambient_min: 0.2
        }
    }
}