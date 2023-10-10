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

    pub fn get_result(&self, scene: &Scene) -> Rgb<u8> { // TODO take parameters and scene
        for object in &scene.objects {
            let result = object.check_hit(self);
            match result {
                Some(colour) => {
                    return colour;
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