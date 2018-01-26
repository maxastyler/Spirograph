extern crate image;

use std::fs::File;
use std::f64::consts::*;

// let img = spiro_image((2000, 2000), linspace(-2., 2., 20), path_points(100000), &path, &gen_envelope);

// Some path that takes t from 0 -> 1 and should close on itself
fn path(t: f64) -> (f64, f64) {
    let theta = 2.*PI*t;
    let r = 500.*(1.-theta.cos()*(3.*theta).sin());
    (r*theta.cos(), r*theta.sin())
}

fn spiral_path(t: f64) -> (f64, f64) {
    let theta = 4.*PI*t;
    let r = 250.*(2.+(theta/2.).sin()*theta.cos());
    (r*theta.cos(), r*theta.sin())
}

fn normal(f: &Fn(f64) -> (f64, f64), dx: f64, t: f64) -> (f64, f64) {
    let x1 = f(t-(dx/2.));
    let x2 = f(t+(dx/2.));
    let diff = (x2.0 - x1.0, x2.1 - x1.1);
    let mag_diff = (diff.0.powf(2.) + diff.1.powf(2.)).sqrt();
    (diff.0/mag_diff, -diff.1/mag_diff)
}

fn envelope(t: f64) -> f64 {
    (2.*PI*t).cos()*100.+(4.*PI*t).sin()*50.
}

// Put in a height from -1 -> 1
fn gen_envelope(h: f64) -> Box<Fn(f64) -> f64> {
    Box::new(
        move |t: f64| (2.*PI*t).cos()*100.+(4.*PI*t).sin()*50.*h
        )
}

fn double_envelope(h: f64) -> Box<Fn(f64) -> f64> {
    Box::new(
        move |t: f64| (2.*PI*t).cos()*600.+(4.*PI*t).sin()*50.*h
        )
}

fn envelope_path<'a>(path: &'a Fn(f64) -> (f64, f64), envelope: Box<Fn(f64) -> f64 + 'a>) -> Box<Fn(f64) -> (f64, f64) + 'a> {
    let a = move |t: f64| {
        let base_p = path(t);
        let norm = normal(path, 0.001, t);
        let e = envelope(t);
        (base_p.0 + norm.0*e, base_p.1 + norm.1*e)
    };
    Box::new(a)
}

fn linspace(a: f64, b: f64, n: u32) -> Vec<f64>{
    (0..n).map(|x| {
        let xf = (x as f64)/(n as f64 -1.) ;
        a+(b-a)*xf
    }
               ).collect()
}

fn spiro_image(img_res: (u32, u32), path_spread: Vec<f64>, path_points: Vec<f64>, path: & Fn(f64) -> (f64, f64), envelope_generator: &Fn(f64) -> Box<Fn(f64) -> f64>) -> image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>> {
    let mut imgbuf = image::ImageBuffer::new(img_res.0, img_res.1);
    for s in path_spread.iter() {
        let env_path = envelope_path(path, envelope_generator(*s));
        for t in path_points.iter() {
            let (mut xf, mut yf) = env_path(*t);
            xf+=img_res.0 as f64 / 2.0;
            yf+=img_res.1 as f64 / 2.0;
            let (x, y) = (xf.round() as u32, yf.round() as u32);
            if (x<img_res.0) & (y<img_res.1) {
                let pix = imgbuf.get_pixel_mut(x, y);
                *pix = image::Luma([255 as u8]);
            }
        }
    }
    imgbuf
}

fn path_points(r: u32) -> Vec<f64> {
    (0..r).map(|i| (i as f64) / (r as f64 - 1.0)).collect()
}

fn main() {
    let fout = &mut File::create("double.png").unwrap();
    let img = spiro_image((2000, 2000), linspace(-1., 1., 20), path_points(100000), &spiral_path, &gen_envelope);
    image::ImageLuma8(img).save(fout, image::PNG).unwrap();
}
