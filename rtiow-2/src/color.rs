use std::fs::File;
use std::io::prelude::*;

use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

//gamma/srgb shi because humans see colors nonlinearly and that stuff
//note that I do not fully understand colorspaces it needs work
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}

pub fn write_color(pixel_color: &Color, file: &mut File) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity: Interval = Interval::new(0.000, 0.999);

    let rbyte = 256. * intensity.clamp(r);
    let gbyte = 256. * intensity.clamp(g);
    let bbyte = 256. * intensity.clamp(b);

    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}
