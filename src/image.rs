use std::fs::File;
use std::io::Write;

use crate::geometry::Color;

const MAX_COLOR: u8 = 255;

pub(crate) struct PPMImage(File);

impl PPMImage {
    pub(crate) fn create(width: u32, height: u32, file_name: &str) -> Self {
        let mut file = File::create(file_name).unwrap();
        write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
        Self(file)
    }

    pub(crate) fn add_color(&mut self, color: Color) {
        write!(
            self.0,
            "{} {} {}\n",
            (color.0 * MAX_COLOR as f64) as u8,
            (color.1 * MAX_COLOR as f64) as u8,
            (color.2 * MAX_COLOR as f64) as u8,
        ).unwrap();
    }
}
