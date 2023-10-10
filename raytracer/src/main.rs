use std::io::Cursor;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb, Pixel};
use glam::{Vec2, Vec3};
mod ray;
mod scene;
use std::boxed::Box;

struct Camera {
    viewport_height: f32,
    focal_length: f32,
    position: Vec3,
}

fn main() {
    let aspect_ratio = 16.0 / 12.0;
    let img_width = 512;
    let img_height = (img_width as f32 / aspect_ratio) as u32;

    let img: RgbImage = ImageBuffer::new(img_width, img_height);

    let cam = Camera {
        viewport_height: 2.0,
        focal_length: 1.0,
        position: Vec3::new(0.0, 0.0, 0.0)
    };

    let mut scene = scene::Scene {
        objects: Vec::new(),
    };

    scene.objects.push(
        Box::new(scene::Sphere {
            position: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.5,
        })
    );

    let img = render(img, cam, scene);

    img.save("out.png").unwrap();
}

fn render(mut image: RgbImage, cam: Camera, scene: scene::Scene) -> RgbImage {
    println!("Starting render...");

    let img_width = image.width();
    let img_height = image.height();

    let viewport_width = cam.viewport_height * (img_width as f32 / img_height as f32);

    // Calc vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -cam.viewport_height, 0.0);

    let delta_u = viewport_u / img_width as f32;
    let delta_v = viewport_v / img_height as f32;


    let upper_left_corner = cam.position - Vec3::new(0.0, 0.0, cam.focal_length) - viewport_u/2.0 - viewport_v/2.0;

    let upper_left_starting_pos = upper_left_corner + 0.5 * (delta_u + delta_v);

    for y in 0..img_height {
        for x in 0..img_width {
            let pixel_center = upper_left_starting_pos + (x as f32 * delta_u) + (y as f32 * delta_v);

            let ray_dir = (pixel_center - cam.position).normalize(); // Focal point calcs

            let casted = ray::PrimitiveRay::new(cam.position, ray_dir);
            
            let result = casted.get_result(&scene);

            image.put_pixel(x, y, result);
        }
    }

    println!("Render complete!");

    return image;
}