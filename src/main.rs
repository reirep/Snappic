//je sais que c'est mal mais quand tu développe des filtres tu ne sais pas tous les appliquer à
//chaque fois
#![allow(dead_code)]

mod color;
mod reduction;
mod utils;
mod mirror;
mod nette;

extern crate raster;

fn main() {
    let mut img = raster::open("in.jpg").unwrap();
    nette::sharpen(&mut img);
    color::grey_mean(&mut img);
    raster::save(&img,"out.jpg").unwrap();
}

