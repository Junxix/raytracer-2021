use crate::perlin::*;
use image::ImageBuffer;
pub use std::path::*;
use std::sync::Arc;
pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct Solid {
    pub color_value: color,
}
impl Solid {
    pub fn new0() -> Self {
        Self {
            color_value: Vec3::zero(),
        }
    }

    pub fn new(c: color) -> Self {
        Self { color_value: c }
    }

    pub fn solid(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(t0: Arc<dyn Texture>, t1: Arc<dyn Texture>) -> Self {
        Self { odd: t0, even: t1 }
    }
    pub fn new_by_color(c1: Vec3, c2: Vec3) -> Self {
        Self {
            even: Arc::new(Solid::new(c1)),
            odd: Arc::new(Solid::new(c2)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl NoiseTexture {
    pub fn new0() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 0.0,
        }
    }
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        color::new(1.0, 1.0, 1.0)
            * (1.0 + (self.noise.turb(p) * 10.0 + self.scale * p.z).sin())
            * 0.5
    }
}

pub struct ImageTexture {
    pub data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    pub width: i64,
    pub height: i64,
    pub bytes_per_scanline: i64,
    pub bytes_per_pixel: i64,
}
impl ImageTexture {
    /*pub fn new0() -> Self {
        Self {
            data: Vec::new(),
            bytes_per_pixel: 3,
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }*/

    pub fn new(filename: &str) -> Self {
        let bytes_per_pixel: i64 = 3;
        //let components_per_pixel = bytes_per_pixel;
        let data2 = image::open(filename).unwrap().to_rgb8();
        let width2 = data2.width();
        let height2 = data2.height();

        let bytes_per_scanline = bytes_per_pixel * (width2 as i64);
        Self {
            bytes_per_pixel,
            data: data2,
            width: width2 as i64,
            height: height2 as i64,
            bytes_per_scanline,
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &point3) -> point3 {
        if self.data.len() == 0 {
            color::new(0.0, 1.0, 1.0)
        } else {
            let u2 = clamp(u, 0.0, 1.0);
            let v2 = 1.0 - clamp(v, 0.0, 1.0);

            let mut i = (u2 * self.width as f64) as usize;
            let mut j = (v2 * self.height as f64) as usize;

            if i >= self.width as usize {
                i = (self.width - 1) as usize;
            }
            if j >= self.height as usize {
                j = (self.height - 1) as usize;
            }

            let color_scale = 1.0 / 255.0;

            let pixel = self.data.get_pixel(i as u32, j as u32);
            let [red, green, blue] = pixel.0;
            color::new(
                color_scale * (red as f64),
                color_scale * (green as f64),
                color_scale * (blue as f64),
            )
        }
    }
}

/*pub struct ImageTexture {
    pub data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
}
impl ImageTexture {
    pub fn new_by_pathstr(dir: &str) -> Self {
        return Self {
            data: image::open(&Path::new(dir)).unwrap().to_rgb(),
        };
    }
    pub fn width(&self) -> u32 {
        return self.data.width();
    }
    pub fn height(&self) -> u32 {
        return self.data.height();
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i: u32 = (u * self.width() as f64) as u32;
        let mut j: u32 = (v * self.height() as f64) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width() {
            i = self.width() - 1;
        }
        if j >= self.height() {
            j = self.height() - 1;
        }

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i, j);
        let [red, green, blue] = pixel.0;
        return Vec3::new(
            red as f64 * COLOR_SCALE,
            green as f64 * COLOR_SCALE,
            blue as f64 * COLOR_SCALE,
        );
    }
}
*/
