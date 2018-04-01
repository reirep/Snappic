//je sais que c'est mal mais quand tu développe des filtres tu ne sais pas tous les appliquer à
//chaque fois
#![allow(dead_code)]

mod color;
mod reduction;
mod utils;
mod mirror;

extern crate raster;
use raster::{Image,Color};

fn main() {
    let mut img = raster::open("in.jpg").unwrap();
    for _x in 0..10000 {
        color::full_blue(&mut img)
    }
    raster::save(&img,"out.jpg").unwrap();
}

