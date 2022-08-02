mod geometry;
mod image;

use std::io::stderr;
use std::io::Write;

use geometry::*;
use image::PPMImage;

// Image and camera
const FILE_NAME: &str = "./image.ppm";
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;

fn pbi(i: u32) {
    let mut stderr = stderr();
    write!(stderr, "\rProcessing line {:<6}", i).unwrap();
    stderr.flush().unwrap();
}

fn pbd() {
    let mut stderr = stderr();
    write!(stderr, "\nDone.\n").unwrap();
}

static s: Sphere = Sphere{
    center: Vector(0.0, 0.0, -1.0),
    radius: 0.5
};

fn get_color(r: Ray) -> Color {
    if let Some(p) = s.cross_point(r) {
        let n = s.normal_at(p);
        return (n + Color::new(1.0, 1.0, 1.0)).shrink(2.0);
    }
    let d = r.direction.unit();
    let t = (d.1 + 1.0) / 2.0;
    Color::new(1.0, 1.0, 1.0).scale(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
}

fn main() {
    let origin = Point::origin();
    let focal_length = 1.0;
    let horizontal = Vector(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector(0.0, VIEWPORT_HEIGHT, 0.0);
    let focal = Vector(0.0, 0.0, focal_length);
    let lower_left_corner = origin - horizontal.shrink(2.0) - vertical.shrink(2.0) - focal;

    let mut image = PPMImage::create(IMAGE_WIDTH, IMAGE_HEIGHT, FILE_NAME);
    for j in (0..IMAGE_HEIGHT).rev() {
        pbi(j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal.scale(u) + vertical.scale(v) - origin);
            let c = get_color(r);
            image.add_color(c);
        }
    }
    pbd();
}
