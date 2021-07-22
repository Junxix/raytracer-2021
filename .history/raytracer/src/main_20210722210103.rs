mod aabb;
mod aarect;
mod bbbox;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
pub use crate::bbbox::*;
pub use crate::bvh::*;
use crate::camera::Camera;
pub use crate::constant_medium::*;
pub use crate::material::*;
pub use crate::moving_sphere::*;
use crate::rtweekend::random_double1;
pub use crate::sphere::Sphere;
use aarect::*;
use indicatif::ProgressBar;
pub use std::thread;
pub use threadpool::ThreadPool;

use image::{ImageBuffer, RgbImage};
//use indicatif::ProgressBar;
pub use ray::Ray;
pub use rtweekend::clamp;
//use std::rc::Rc;
use std::sync::Arc;
pub use vec3::Vec3;
pub use Vec3 as point3;
pub use Vec3 as color;

use std::sync::mpsc::channel;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
pub const MAXDEPTH: i64 = 50;

pub struct ThreadTemp {
    pub x: u32,
    pub color: Vec<[u8; 3]>,
}

fn ray_color(r: Ray, background: color, world: &HittableList, depth: i64) -> color {
    let mut rec = HitRecord::new0();
    if depth <= 0 {
        return color::new(0.0, 0.0, 0.0);
    }
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return background;
    }
    let mut scattered = Ray::new0();
    let mut attenuation = color::new0();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
    let tmp_rec = rec.clone();
    if !rec
        .mat_ptr
        .scatter(r, &tmp_rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted
        + Vec3::elemul(
            attenuation,
            ray_color(scattered, background, world, depth - 1),
        )
}

/*fn ray_color(r: Ray, background: color, world: &HittableList, depth: i32) -> color {
    let mut rec = HitRecord::new0();

    if depth <= 0 {
        return color::new(0.0, 0.0, 0.0);
    }

    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return background;
    }

    let mut scattered = Ray::default_new();
    let mut attenuation = color::zero();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
    let mut tmp_rec = rec.clone();
    if !rec
        .mat_ptr
        .scatter(r, &mut tmp_rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted + Vec3::(elemulray_color(scattered, background, world, depth - 1), attenuation)
}*/
fn cornell_smoke() -> HittableList {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::new_by_color(color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_by_color(color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_by_color(color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Arc::new(Solid::new(color::new(
        7.0, 7.0, 7.0,
    )))));

    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Box::new(
        &point3::new(0.0, 0.0, 0.0),
        &point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::new_by_color(
        bbbox1,
        0.01,
        color::new(0.0, 0.0, 0.0),
    )));

    let bbbox2 = Arc::new(bbbox::new(
        &point3::new(0.0, 0.0, 0.0),
        &point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let bbbox2 = Arc::new(RotateY::new(bbbox2, -18.0));
    let bbbox2 = Arc::new(Translate::new(bbbox2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::new_by_color(
        bbbox2,
        0.01,
        color::new(1.0, 1.0, 1.0),
    )));

    world
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        point3::new(0.0, -1000.0, -0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        point3::new(0.0, 2.0, -0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    objects
}
/*pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new());

    objects.objects.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    objects.objects.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));
    return objects;
}*/

fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("e.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));

    let globe = Arc::new(Sphere::new(point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut objects = HittableList::new();
    objects.add(globe);
    objects
    /*let mut objects = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new_by_pathstr(&String::from("e.jpg")));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    objects.add(Arc::new(Sphere::new(Vec3::zero(), 2.0, earth_surface)));
    return objects;*/
}
fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_by_color(
        color::new(0.2, 0.3, 0.1),
        color::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        point3::new(0.0, -10.0, -0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        point3::new(0.0, 10.0, -0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));

    objects
}
/*
fn cornell_bbbox() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_by_color(color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_by_color(color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_by_color(color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_by_color(color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    objects
}
*/
fn cornell_bbbox() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_by_color(color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_by_color(color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_by_color(color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_by_color(color::new(15., 15., 15.)));

    objects.add(Arc::new(YzRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YzRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XzRect::new(213., 343., 227., 332., 554., light)));
    objects.add(Arc::new(XzRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Arc::new(XzRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    let bbbox1 = Arc::new(bbbox::new(
        &point3::new(0.0, 0.0, 0.0),
        &point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let bbbox1 = Arc::new(RotateY::new(bbbox1, 15.0));
    let bbbox1 = Arc::new(Translate::new(bbbox1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(bbbox1);

    let bbbox2 = Arc::new(bbbox::new(
        &point3::new(0.0, 0.0, 0.0),
        &point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let bbbox2 = Arc::new(RotateY::new(bbbox2, -18.0));
    let bbbox2 = Arc::new(Translate::new(bbbox2, &Vec3::new(130.0, 0.0, 65.0)));

    objects.add(bbbox2);

    objects
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        point3::new(0.0, -1000.0, -0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        point3::new(0.0, 2.0, -0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_by_color(color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_by_color(
        color::new(0.2, 0.3, 0.1),
        color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
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
                    //diffuse
                    let albedo = Vec3::elemul(color::random1(), color::random1());
                    let sphere_material = Arc::new(Lambertian::new_by_color(albedo));
                    let center2 = center + Vec3::new(0.0, random_double2(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = color::random2(0.5, 1.0);
                    let fuzz = random_double2(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_by_color(color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
fn final_scene() -> HittableList {
    let mut bbboxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new_by_color(color::new(0.48, 0.83, 0.53)));

    let bbboxes_per_side = 20;
    for i in 0..bbboxes_per_side {
        for j in 0..bbboxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double2(1.0, 101.0);
            let z1 = z0 + w;

            bbboxes1.add(Arc::new(bbbox::new(
                &point3::new(x0, y0, z0),
                &point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BvhNode::from(&mut bbboxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_by_color(color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new_by_color(color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(color::new(0.8, 0.8, 0.9), 10.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_by_color(
        boundary,
        0.02,
        color::new(0.2, 0.4, 0.9),
    )));
    let boundary1 = Arc::new(Sphere::new(
        point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_by_color(
        boundary1,
        0.0001,
        color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian {
        albedo: Arc::new(ImageTexture::new("e.jpg")),
    });
    world.add(Arc::new(Sphere::new(
        point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    world.add(Arc::new(Sphere::new(
        point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    let mut bbboxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new_by_color(color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        bbboxes2.add(Arc::new(Sphere::new(
            point3::random2(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::from(&mut bbboxes2, 0.0, 1.0)),
            15.0,
        )),
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    world
}

fn main() {
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width: u32 = 400;
    let mut samples_per_pixel: u32 = 50;
    let mut background = color::new0();
    let _x = background;

    //world
    let mut world = HittableList::new();
    let _xx = world;

    let mut lookfrom = point3::new0();
    let mut lookat = point3::new0();
    let mut vfov = 40.0;
    let aperture = 0.0;
    let _xx = aperture;
    let _xx = lookfrom;
    let _xx = lookat;
    let _xx = vfov;

    let _xx = 0;

    match _xx {
        1 => {
            world = random_scene();
            lookfrom = point3::new(13.0, 2.0, 3.0);
            lookat = point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            background = color::new(0.70, 0.80, 1.00);
        }
        2 => {
            world = two_spheres();
            lookfrom = point3::new(13.0, 2.0, 3.0);
            lookat = point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            background = color::new(0.70, 0.80, 1.00);
        }
        3 => {
            world = two_perlin_spheres();
            lookfrom = point3::new(13.0, 2.0, 3.0);
            lookat = point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            background = color::new(0.70, 0.80, 1.00);
        }
        4 => {
            world = earth();
            lookfrom = point3::new(13.0, 2.0, 3.0);
            lookat = point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            background = color::new(0.70, 0.80, 1.00);
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            lookfrom = point3::new(26.0, 3.0, 6.0);
            lookat = point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            background = color::new(0.0, 0.0, 0.0);
        }
        6 => {
            world = cornell_bbbox();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = color::new(0.0, 0.0, 0.0);
            lookfrom = point3::new(278.0, 278.0, -800.0);
            lookat = point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            samples_per_pixel = 200;
            lookfrom = point3::new(278.0, 278.0, -800.0);
            lookat = point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            background = color::new(0.0, 0.0, 0.0);
        }
        8 => {
            world = final_scene();
            aspect_ratio = 1.0;
            samples_per_pixel = 100;
            background = color::new(0.0, 0.0, 0.0);
            lookfrom = point3::new(478.0, 278.0, -600.0);
            lookat = point3::new(278., 278., 0.);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            samples_per_pixel = 100;
            background = color::new(0.0, 0.0, 0.0);
            lookfrom = point3::new(478.0, 278.0, -600.0);
            lookat = point3::new(278., 278., 0.);
            vfov = 40.0;
        }
    }

    let k = image_width as f64;
    let image_height = k / aspect_ratio;
    let image_height = image_height as u32;
    //camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let mut cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    cam = Camera::new2(&cam, 0.0, 1.0);

    // Render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);

    //let pixel_color: [[Color; width as usize]; height as usize] = [[Color::new(0.0, 0.0, 0.0); width as usize]; height as usize];

    println!("width:{} height:{}", image_width, image_height);

    let thread_num = 32; //if is_ci() { 2 } else { 8 };

    let (tx, rx) = channel();

    for i in 0..thread_num {
        let start = i * image_width / thread_num;
        let end = (i + 1) * image_width / thread_num;

        let _tx = tx.clone();
        let _world = world.clone();
        let _cam = cam.clone();
        thread::spawn(move || {
            for x in start..end {
                let mut temp = ThreadTemp { x, color: vec![] };
                for y in 0..image_height {
                    let mut pixel_color = color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = (x as f64 + random_double2(0.0, 1.0)) / (image_width - 1) as f64;
                        let v = ((image_height - y) as f64 + random_double2(0.0, 1.0))
                            / (image_height - 1) as f64;
                        let r = _cam.get_ray(u, v);
                        pixel_color += ray_color(r, background, &_world, MAXDEPTH);
                    }
                    let mut r = pixel_color.x;
                    let mut g = pixel_color.y;
                    let mut b = pixel_color.z;

                    let scale = 1.0 / samples_per_pixel as f64;
                    r = (scale * r).sqrt();
                    g = (scale * g).sqrt();
                    b = (scale * b).sqrt();

                    temp.color.push([
                        (clamp(r, 0.0, 0.999) * 255.0) as u8,
                        (clamp(g, 0.0, 0.999) * 255.0) as u8,
                        (clamp(b, 0.0, 0.999) * 255.0) as u8,
                    ]);
                }
                _tx.send(temp).expect("failed to send");
            }
        });
    }
    for receive in rx.iter().take(image_width as usize) {
        let x = receive.x;
        //print!("{}\n", x);
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb(receive.color[y as usize]);
        }
        bar.inc(1);
    }
    img.save("output/test.png").unwrap();
    bar.finish();
    /*//render
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
                pixel_color += ray_color(_r, background, &world, MAXDEPTH);
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
    img.save("output/test.png").unwrap();*/
}
/*fn write_color(pixel: &mut image::Rgb<u8>, pixel_color: &Vec3, samples_per_pixel: u32) {
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
*/
