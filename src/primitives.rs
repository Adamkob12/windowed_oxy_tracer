use crate::vec3::*;
use crate::ray::*;
use image::Rgb;


//rest of program should only interact with this type
#[derive(Debug, Clone)]
pub enum Primitive {
    Sphere(Sphere),
}

pub trait Collideable {
    fn ray_intersection(&self, ray: Ray) -> Option<(f64, f64)>;
    fn normal_at_point(&self, point: V3) -> V3;
}

impl Collideable for Primitive {
    fn ray_intersection(&self, ray: Ray) -> Option<(f64, f64)> {
        match self {
            Primitive::Sphere(s) => s.ray_intersection(ray)
        }
    }

    fn normal_at_point(&self, point: V3) -> V3 {
        match self {
            Primitive::Sphere(s) =>s.normal_at_point(point),
        }
    }
}

impl Primitive {
    pub fn color(&self) -> V3 {
        match self {
            Primitive::Sphere(t) => t.color
        }
    }

    pub fn specular(&self) -> f64 {
        match self {
            Primitive::Sphere(t) => t.specular
        }
    }

    pub fn reflective(&self) -> f64 {
        match self {
            Primitive::Sphere(t) => t.reflective
        }
    }
}



//sphere
#[derive(Debug, Clone)]
pub struct Sphere {
    center: V3,
    radius: f64,
    color: V3,
    specular: f64, //larger value is more shiny.
    reflective: f64, //value should be between 0 and 1. 1 is a mirror.
}

impl Collideable for Sphere {
    /*returns scalar of ray at point(s) where ray collides with the sphere.
    t1 and t2 may be equal, meaning there is only one collision */
    
    fn ray_intersection(&self, ray: Ray) -> Option<(f64, f64)> { 
        let r = self.radius;
        let co = ray.origin - self.center;

        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(co, ray.direction);
        let c = dot(co, co) - r*r;

        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return None;
        }

        let rhs = f64::sqrt(discriminant); //rhs is the righthand side of the quadratic formula
        let two_a = 2.0 * a; //so as to not compute twice

        let t1 = (rhs - b) / two_a;
        let t2 = (-b - rhs) / two_a;

        return Some((t1, t2));
    }

    fn normal_at_point(&self, point: V3) -> V3 {
        (point - self.center).normalize()
    }
}





//pub functions
//reflective value is between 0 and 1
pub fn new_sphere(center: V3, radius: f64, color: V3, specular: f64, reflective: f64) -> Primitive {
    Primitive::Sphere(
        Sphere {
            center: center,
            radius: radius,
            color: color,
            specular: specular,
            reflective: reflective,
        }
    )
}