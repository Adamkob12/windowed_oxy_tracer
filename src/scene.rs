use crate::vec3::*;
use crate::primitives::*;

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,

    pub canvas_width: i32,
    pub canvas_height: i32,

    pub viewframe_width: f64,
    pub viewframe_height: f64,
    pub viewframe_distance: f64,

    pub camera: V3,
    pub recursion_limit: i32,
}

#[derive(Debug, Clone)]
pub enum Light {
    Point((V3, f64)),
    Directional((V3, f64)),
}

impl Light {
    pub fn contents(&self) -> (V3, f64) {
        match self {
            Light::Point(t) => (t.0, t.1),
            Light::Directional(t) => (t.0, t.1),
        }
    }
}

pub fn new_scene() -> Scene{
    Scene {
        primitives: Vec::new(),
        lights: Vec::new(),
        ambient_light: 0.0,
        canvas_width: 0,
        canvas_height: 0,
        viewframe_width: 0.0,
        viewframe_height: 0.0,
        viewframe_distance: 0.0,
        camera: new_vec(0.0, 0.0, 0.0),
        recursion_limit: 0,
    }
}