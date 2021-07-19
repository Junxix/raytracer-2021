pub use crate::ray::Ray;
pub use crate::rtweekend::*;
pub use crate::vec3::Vec3;
pub use Vec3 as point3;
pub use Vec3 as color;
pub struct Camera {
    origin: point3,
    lower_left_corner: point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: point3,
        lookat: point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let ww = (lookfrom - lookat).unit();
        let uu = (Vec3::cross(vup, ww)).unit();
        let vv = Vec3::cross(ww, uu);

        let origin2 = lookfrom;
        let horizontal2 = uu * viewport_width * focus_dist;
        let vertical2 = vv * viewport_height * focus_dist;
        return Self {
            origin: lookfrom,
            horizontal: horizontal2,
            vertical: vertical2,
            lower_left_corner: origin2 - horizontal2 / 2.0 - vertical2 / 2.0 - ww * focus_dist,
            u: uu,
            v: vv,
            w: ww,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let _x = self.w;
        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
