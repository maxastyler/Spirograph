extern crate image;

use std::fs::File;
use std::f64::consts::*;

const IMGX: u32 = 2000;
const IMGY: u32 = 2000;
const res: u32 = 100000;
const centre: (f64, f64) = (IMGX as f64 / 2., IMGY as f64 / 2.);

// Some path that takes t from 0 -> 1 and should close on itself
fn path(t: f64) -> (f64, f64) {
    let theta = 2.*PI*t;
    let r = 500.*(1.-theta.cos()*(3.*theta).sin());
    (r*theta.cos() + centre.0, r*theta.sin() + centre.1)
}

fn normal(f: &Fn(f64) -> (f64, f64), dx: f64, t: f64) -> (f64, f64) {
    let x1 = f(t-(dx/2.));
    let x2 = f(t+(dx/2.));
    let diff = (x2.0 - x1.0, x2.1 - x1.1);
    let mag_diff = (diff.0.powf(2.) + diff.1.powf(2.)).sqrt();
    (diff.0/mag_diff, -diff.1/mag_diff)
}

fn in_bounds(x: u32, y: u32) -> bool {
    if (x<=IMGX) & (y<=IMGY) { true } else { false }
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

fn envelope_path<'a>(path: &'a Fn(f64) -> (f64, f64), envelope: Box<Fn(f64) -> f64 + 'a>) -> Box<Fn(f64) -> (f64, f64) + 'a> {
    let a = move |t: f64| {
        let base_p = path(t);
        let norm = normal(path, 0.01, t);
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

fn main() {
    let mut imgbuf = image::ImageBuffer::new(IMGX, IMGY);
    let fout = &mut File::create("spiro.png").unwrap();
    for h in linspace(-2., 2., 20).iter(){
        let p = envelope_path(&path, gen_envelope(*h));
        for i in 0..res {
            let t = (i as f64) / (res as f64 - 1.0);
            let (xf, yf) = p(t);
            let (x, y) = (xf.round() as u32, yf.round() as u32);
            if in_bounds(x, y) {
                let pix = imgbuf.get_pixel_mut(x, y);
                *pix = image::Luma([255 as u8]);
            }
        }
    }

    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}
