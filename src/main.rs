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

fn grey_mean(img: &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            let avg : u8 =  color.r/3 + color.g/3 + color.b/3;
            img.set_pixel(x,y, Color::rgb(avg, avg, avg)).unwrap();
        }
    }
}

fn sepia(img: &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();

            let r = color.r as f32;
            let g = color.g as f32;
            let b = color.b as f32;


            let sr = 0.393*r + 0.769*g + 0.189*b;
            let sg = 0.349*r + 0.686*g + 0.168*b;
            let sb = 0.272*r + 0.534*g + 0.131*b;

            img.set_pixel(x,y,Color::rgb(
                    if sr > 255.0 {255} else {sr as u8}, 
                    if sg > 255.0 {255} else {sg as u8}, 
                    if sb > 255.0 {255} else {sb as u8}
                )).unwrap();
        }
    }
}

fn solarised_high(img: &mut Image){
   let treshold = 125;
    for y in 0..img.height {
        for x in 0..img.width {
        let color = img.get_pixel(x,y).unwrap();

        img.set_pixel(x,y,Color::rgb(
                    if color.r > treshold {255-color.r} else {color.r}, 
                    if color.g > treshold {255-color.g} else {color.g},
                    if color.b > treshold {255-color.b} else {color.b},
                )).unwrap();
       }
   }
}

fn solarised_low(img : &mut Image){
   let treshold = 125;
    for y in 0..img.height {
        for x in 0..img.width {
        let color = img.get_pixel(x,y).unwrap();

        img.set_pixel(x,y,Color::rgb(
                    if color.r < treshold {255-color.r} else {color.r}, 
                    if color.g < treshold {255-color.g} else {color.g},
                    if color.b < treshold {255-color.b} else {color.b},
                )).unwrap();
       }
   }
}

fn invert(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(
                    255-color.r,
                    255-color.g,
                    255-color.b
                )).unwrap();
        }
    }
}
