use glam::{Vec2, Vec3};
use image::Rgb;
use crate::scene::Scene;

pub struct PrimitiveRay { // This will get much more complicated when we add gravitational effects
    pub origin: Vec3,
    pub direction: Vec3,
}

impl PrimitiveRay {
    pub fn new(origin: Vec3, direction: Vec3) -> PrimitiveRay {
        PrimitiveRay { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t*self.direction
    }

    pub fn get_result(&self, scene: &Scene) -> Rgb<u8> { // TODO take parameters and scene
        for object in &scene.objects {
            let result = object.check_hit(self);
            match result {
                Some(t) => {
                    let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
                    return Rgb::<u8>([((n.x + 1.0) * 255.0 * 0.5) as u8, ((n.y + 1.0) * 255.0 * 0.5) as u8, ((n.z + 1.0) * 255.0 * 0.5) as u8])
                }
                None => {
                    continue;
                }
            }
        }
        let a = 0.5 * (self.direction.normalize().y + 1.0);
        let r = (1.0 - a)*1.0 + a*0.5;
        let g = (1.0-a)*1.0 + a*0.7;
        let b = (1.0-a)*1.0 + a*1.0;

        Rgb::<u8>([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8])
    }
}