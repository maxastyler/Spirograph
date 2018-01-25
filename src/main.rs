extern crate image;

use std::fs::File;
use std::f64::consts::*;

// Some path that takes t from 0 -> 1 and should close on itself
fn path(t: f64) -> (f64, f64) {
    let theta = 2.*PI*t;
    let r = theta.cos();
    (r*theta.cos(), r*theta.sin())

}

fn envelope(t: f64) -> f64 {
    (PI*t).cos()
}

fn main() {
    println!("Hello, world!");
    let mut imgbuf = image::ImageBuffer::new(800, 800);
    let fout = &mut File::create("spiro.png").unwrap();

    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}
