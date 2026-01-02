use crate::{Vec3, dot};

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3{
    *v - 2.0 * dot(v, n) * n
}

/// > Note: uv and n must both be unit vectors
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = dot(&-uv, &n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + &(cos_theta * n));
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}