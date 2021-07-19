mod camera;
mod hittable;
mod material;
mod ray;
mod rtweekend;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;
use crate::camera::Camera;
pub use crate::material::*;
use crate::rtweekend::random_double1;
pub use crate::sphere::Sphere;
use image::{ImageBuffer, RgbImage};
//use indicatif::ProgressBar;
pub use ray::Ray;
pub use rtweekend::clamp;
use std::rc::Rc;
use std::sync::Arc;
pub use vec3::Vec3;
pub use Vec3 as point3;
pub use Vec3 as color;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
pub const MAXDEPTH: i64 = 50;

fn ray_color(r: Ray, world: &HittableList, depth: i64) -> color {
    let mut rec = HitRecord::new0();
    if depth <= 0 {
        return color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, rtweekend::INFINITY, &mut rec) {
        let mut scattered = Ray::default_new();
        let mut attenuation = color::zero();
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return Vec3::elemul(attenuation, ray_color(scattered, world, depth - 1));
        }
        return color::zero();
    }
    let unit_direction = Vec3::unit(&r.direction());
    let t = (unit_direction.y + 1.0) * 0.5;
    return color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::new(0.5, 0.7, 1.0) * t;
}
fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double1();
            let aa = a as f64;
            let bb = b as f64;
            let center = point3::new(
                aa + 0.9 * random_double1(),
                0.2,
                bb + 0.9 * random_double1(),
            );

            if (center - point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::elemul(color::random1(), color::random1());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = color::random2(0.5, 1.0);
                    let fuzz = random_double2(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let _k = image_width as f64;
    let image_height = _k / aspect_ratio;
    let image_height = image_height as u32;
    let samples_per_pixel: u32 = 50;

    //world
    let world = random_scene();

    //camera
    let lookfrom = point3::new(13.0, 2.0, 3.0);
    let lookat = point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    //render
    /*let _x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", _x);*/
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    //println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = color::zero();
            let pixel = img.get_pixel_mut(i, j);
            for _s in 0..samples_per_pixel {
                let _i: f64 = i as f64;
                let _j: f64 = (image_height - 1 - j) as f64;
                let _i: f64 = (_i + random_double1()) as f64;
                let _j: f64 = (_j + random_double1()) as f64;
                let _p: f64 = (image_width - 1) as f64;
                let _q: f64 = (image_height - 1) as f64;
                let _u: f64 = _i / _p;
                let _v: f64 = _j / _q;
                //println!("{:?}", r);
                let _r: Ray = Camera::get_ray(&cam, _u, _v);
                pixel_color += ray_color(_r, &world, MAXDEPTH);
            }

            write_color(pixel, &pixel_color, samples_per_pixel);

            /* let pixel = img.get_pixel_mut(i, j);
            let i: f64 = i as f64;
            let j: f64
            = (image_height - 1 - j) as f64;
            let p: f64 = (image_width - 1) as f64;
            let q: f64 = (image_height - 1) as f64;
            let b: f64 = 0.25;
            let u: f64 = i / p;
            let v: f64 = j / q;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r, &world);
            write_color(pixel, &pixel_color);*/
        }
    }
    img.save("output/test.png").unwrap();
}
fn write_color(pixel: &mut image::Rgb<u8>, pixel_color: &Vec3, samples_per_pixel: u32) {
    let mut _r = pixel_color.x;
    let mut _g = pixel_color.y;
    let mut _q = pixel_color.z;

    let samples_per_pixel = samples_per_pixel as f64;
    let scale = 1.0 / samples_per_pixel;
    _r = (_r * scale).sqrt();
    _g = (_g * scale).sqrt();
    _q = (_q * scale).sqrt();

    let _a: u8 = (256.0 * clamp(_r, 0.0, 0.999)) as u8;
    let _b: u8 = (256.0 * clamp(_g, 0.0, 0.999)) as u8;
    let _c: u8 = (256.0 * clamp(_q, 0.0, 0.999)) as u8;
    *pixel = image::Rgb([_a, _b, _c]);
    //println!("{} {} {}", a, b, c);
}
