use glam::{Vec2, Vec3};
use crate::ray::PrimitiveRay;
use image::Rgb;

pub struct Scene {
    pub objects: Vec<Box<dyn SceneObject>>
}

pub trait SceneObject {
    fn check_hit(&self, ray: &PrimitiveRay) -> Option<Rgb<u8>>;
    fn position(&self) -> Vec3;
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl SceneObject for Sphere {
    fn check_hit(&self, ray: &PrimitiveRay) -> Option<Rgb<u8>> {
        let offset = ray.origin - self.position;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * offset.dot(ray.direction);
        let c = offset.dot(offset) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return None
        }

        //return Some(Rgb::<u8>([(255.0 - discriminant.sqrt() * 255.0) as u8, 0, 0]))
        Some(Rgb::<u8>([255, 255, 0]))
    }

    fn position(&self) -> Vec3 {
        self.position
    }
}