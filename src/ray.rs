use crate::vec3::*;
use crate::primitives::*;
use crate::util::*;
use crate::scene::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: V3,
    pub direction: V3,
}

impl Ray {
    pub fn point_at_scale(&self, scalar: f64) -> V3{
        self.origin + self.direction * scalar
    }

    pub fn closest_point(&self, scene_primitives: &Vec<Primitive>, t_min: f64, t_max: f64) -> Option<(V3, Primitive)> {
        let mut closest_t: f64 = t_max;
        let mut closest_primitive = None;

        for current in scene_primitives.iter() {
            let (t1, t2) = match current.ray_intersection(*self) {
                Some(p) => p,
                None => {continue;},
            };

            if in_range(t1, t_min, t_max) && t1 < closest_t {
                closest_t = t1;
                closest_primitive = Some(current);
            }
            
            if t1 != t2 && in_range(t2, t_min, t_max) && t2 < closest_t {
                closest_t = t2;
                closest_primitive = Some(current);
            }
        }

        return match closest_primitive {
            None => None,
            Some(p) => Some((self.point_at_scale(closest_t), p.clone()))
        };      
    }
}

pub fn new_ray(origin: V3, direction: V3) -> Ray{
    Ray {
        origin: origin,
        direction: direction,
    }
}

fn reflect_ray(ray: V3, surface_normal: V3) -> V3{
    2.0 * surface_normal * dot(surface_normal, ray) - ray
}

pub fn trace_ray(scene: &Scene, ray: Ray, t_min: f64, t_max: f64, recursion_depth: i32) -> V3 {
    let temp = ray.closest_point(&scene.primitives, t_min, t_max);
    if temp.is_none() {
        return new_vec(255.0, 255.0, 255.0);
    }

    let (point, primitive) = temp.unwrap();
    
    let local_color = primitive.color() * compute_lighting(scene, point, &primitive);

    let current_reflective = primitive.reflective();
    if recursion_depth <= 0 || current_reflective <= 0.0 {
        return local_color;
    }

    let reflected = reflect_ray(-1.0 * ray.direction, primitive.normal_at_point(point));
    let reflected = new_ray(point, reflected);
    let reflected_color = trace_ray(scene, reflected, EPSILON, INF, recursion_depth - 1);

    return local_color * (1.0 - current_reflective) + reflected_color * current_reflective;
}

pub fn compute_lighting(scene: &Scene, point: V3, primitive: &Primitive) -> f64 {
    let mut lighting_factor = scene.ambient_light; //light strength at given point
    let normal = primitive.normal_at_point(point); 

    for light in scene.lights.iter() {
        let light_dir: V3;
        let t_max: f64; //furthest away to search when checking shadow collision

        match *light {
            Light::Point(temp) => {light_dir = temp.0 - point; t_max = 1.0}, //temp.0 is the V3 component of a light
            Light::Directional(temp) => {light_dir = temp.0; t_max = INF},
        }

        //shadow check
        match new_ray(point, light_dir)
            .closest_point(&scene.primitives, EPSILON, t_max) {
                Some(_) => continue,
                None => ()
            }

        let distance_factor = match *light {
            Light::Point((light_pos, _intensity)) => 1.0 / f64::sqrt((light_pos - point).length()),
            Light::Directional(_) => 1.0,
        };

        //diffuse reflection
        let n_dot_l = dot(normal, light_dir);
        if n_dot_l > 0.0 {
            lighting_factor += distance_factor * light.contents().1 * n_dot_l / (normal.length() * light_dir.length());
        } //light.contents().1 is the same as light.intensity()

        
        //specular reflection
        if primitive.specular() != -1.0 {
            let point_to_camera = -1.0 * point;
            let r = reflect_ray(light_dir, normal);
            let r_dot_v = dot(r, point_to_camera);

            if r_dot_v > 0.0 {
                lighting_factor += light.contents().1 * ((r_dot_v / (r.length() * point_to_camera.length())).powf(primitive.specular()));
            }
        }
    }

    //println!("{}", lighting_factor);
    lighting_factor
}