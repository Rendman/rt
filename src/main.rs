use color::Color;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use image::{Rgb, RgbImage};

mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;

const ASPECT_RATIO:f64 = 16.0/9.0;

const VIEWPORT_HEIGHT:f64 = 2.0;
const VIEWPORT_WIDTH:f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH:f64 = 1.0;

const ORIGIN:Vec3 = Vec3 {e0: 0.0, e1: 0.0, e2: 0.0};
const HORIZONTAL:Vec3 = Vec3 {e0: VIEWPORT_WIDTH, e1: 0.0, e2: 0.0};
const VERTICAL:Vec3 = Vec3 {e0: 0.0, e1: VIEWPORT_HEIGHT, e2: 0.0};

fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(Vec3 {e0: 0.0, e1: 0.0, e2: -1.0}, 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3 { e0: 0.0, e1: 0.0, e2: -1.0}).unit_vector();
        return Color(Vec3 {e0: n.x()+1.0, e1: n.y()+1.0, e2: n.z()+1.0}.mul_scalar(0.5));
    }
    let unit_direction = r.direction.unit_vector();
    t = (unit_direction.y()+1.0)*0.5;
    let val = Vec3 {e0: 1.0, e1: 1.0, e2: 1.0}.mul_scalar(1.0-t) + Vec3 {e0: 0.5, e1: 0.7, e2: 1.0}.mul_scalar(t);
    Color(val)
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - center;

    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b*half_b - a*c;
    
    
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - f64::sqrt(discriminant) ) / a;
    }
}

fn main() {
    let image_width:u32 = 1024;
    let image_height:u32 = (image_width as f64/ASPECT_RATIO) as u32;

    let mut output_image = RgbImage::new(image_width, image_height);
    let lower_left_corner:Vec3 = (ORIGIN - HORIZONTAL).div_scalar(2.0) - VERTICAL.div_scalar(2.0) - Vec3 {e0: 0.0, e1: 0.0, e2: FOCAL_LENGTH };

    for y in (0..image_height).rev() {
        //println!("Scanlines remaining: {}", y);
        for x in 0..image_width {

            let u = x as f64 / (image_width as f64 - 1.0);
            let v = y as f64 / (image_height as f64 - 1.0);

            let r = Ray{ origin: ORIGIN, direction: lower_left_corner + HORIZONTAL.mul_scalar(u) + VERTICAL.mul_scalar(v) - ORIGIN};
        
            //let color = Color(Vec3{ e0: x as f64 / (image_width as f64), e1: y as f64 / (image_height as f64), e2: 0.25});         
            let color = ray_color(r);


            output_image.put_pixel(x, y, Rgb([color.r(), color.g(), color.b()]));
        }
    }

    output_image.save("output.ppm").unwrap();
}
