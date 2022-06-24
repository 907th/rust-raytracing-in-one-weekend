use std::fs::File;
use std::io::prelude::*;
use std::io::stderr;

const FILE_NAME: &str = "./image.ppm";

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

const MAX_COLOR: u8 = 255;

fn pbi(i: u32) {
    let mut stderr = stderr();
    write!(stderr, "\rProcessing line {:<6}", i).unwrap();
    stderr.flush().unwrap();
}

fn pbd() {
    let mut stderr = stderr();
    write!(stderr, "\nDone.\n").unwrap();
}

fn main() {
    let mut file = File::create(FILE_NAME).unwrap();
    write!(file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    for j in (0..IMAGE_HEIGHT).rev() {
        pbi(j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / IMAGE_WIDTH as f64;
            let g = j as f64 / IMAGE_WIDTH as f64;
            let b = 0.25f64;
            let ir = (r * MAX_COLOR as f64) as u8;
            let ig = (g * MAX_COLOR as f64) as u8;
            let ib = (b * MAX_COLOR as f64) as u8;
            write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
    pbd();
}
