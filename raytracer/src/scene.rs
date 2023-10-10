use glam::{Vec2, Vec3};
use crate::ray::PrimitiveRay;
use image::Rgb;

pub struct Scene {
    pub objects: Vec<Box<dyn SceneObject>>
}

pub trait SceneObject {
    fn check_hit(&self, ray: &PrimitiveRay) -> Option<f32>;
    fn position(&self) -> Vec3;
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl SceneObject for Sphere {
    fn check_hit(&self, ray: &PrimitiveRay) -> Option<f32> {
        let offset = ray.origin - self.position;

        let a = ray.direction.length_squared();
        let half_b = offset.dot(ray.direction);
        let c = offset.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return None
        }

        //return Some(Rgb::<u8>([(255.0 - discriminant.sqrt() * 255.0) as u8, 0, 0]))
        Some((-half_b - discriminant.sqrt()) / a)
    }

    fn position(&self) -> Vec3 {
        self.position
    }
}