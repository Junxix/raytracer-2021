pub use crate::hittable::*;
pub use crate::rtweekend::*;
pub use crate::texture::*;
pub use crate::vec3::*;
use crate::Ray;
//use std::rc::Rc;
use std::sync::Arc;
pub use Vec3 as point3;
pub use Vec3 as color;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &point3) -> color {
        let _x = _u;
        let _y = _v;
        let _z = _p;
        color::zero()
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
    pub fn new_by_color(a: Vec3) -> Self {
        Self {
            albedo: Arc::new(Solid::new(a)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, _ray_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

pub struct Metal {
    pub albedo: color,
    pub fuss: f64,
}
impl Metal {
    pub fn new(a: color, f: f64) -> Self {
        let x: f64;
        if f < 1.0 {
            x = f;
        } else {
            x = 1.0;
        }
        Self { albedo: a, fuss: x }
    }
    pub fn new1(a: color) -> Self {
        Self {
            albedo: a,
            fuss: 1.0,
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray_in.direction().unit(), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuss,
            ray_in.time(),
        );
        *attenuation = self.albedo;
        scattered.direction() * rec.normal > 0.0
    }
}

pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    pub fn new(x: f64) -> Self {
        Self { ir: x }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }
        let unit_direction = ray_in.direction().unit();

        let cos_theta = Vec3::fmin(-unit_direction * rec.normal, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::new0();
        let _x = direction;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double1()
        {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction, ray_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { emit: a }
    }
    pub fn new_by_color(c: color) -> Self {
        Self {
            emit: Arc::new(Solid::new(c)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray_in: Ray,
        _rec: &HitRecord,
        _attenuation: &mut point3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &point3) -> color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }

    pub fn new_from_color(c: color) -> Self {
        Self::new(Arc::new(Solid::new(c)))
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray {
            orig: rec.p,
            dir: Vec3::random_in_unit_sphere(),
            tm: r_in.tm,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
