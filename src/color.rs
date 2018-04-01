use raster::{Image,Color};
use utils::{add,sub,limit};

//set brightness

pub fn brightness_add(img : &mut Image, change : u8){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(
                    add(color.r,change),
                    add(color.g,change),
                    add(color.b,change)
                )).unwrap();
        }
    }
}

pub fn brightness_sub(img : &mut Image, change : u8){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(
                    sub(color.r,change),
                    sub(color.g,change),
                    sub(color.b,change)
                )).unwrap();
        }
    }
}

//change the contrast
pub fn contrast(img : &mut Image, change : f32){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(
                    px_contrast(color.r,change),
                    px_contrast(color.g,change),
                    px_contrast(color.b,change)
                )).unwrap();
        }
    } 
}

fn px_contrast(px: u8, val : f32) -> u8
{
    let decal = 127.5; 
    limit(((px as f32 - decal) * val + decal).round())
}

//change the gamma
pub fn gamma(img : &mut Image, change : f32){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(
                        px_gamma(color.r,change),
                        px_gamma(color.g,change),
                        px_gamma(color.b,change)
                    )).unwrap();
        }
    }
}

fn px_gamma(px: u8, val : f32) -> u8
{
    let decal =  255.0;
    limit((px as f32 / decal).powf(val)*decal)
}

//somme amusing filters

pub fn grey_mean(img: &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            let avg : u8 =  color.r/3 + color.g/3 + color.b/3;
            img.set_pixel(x,y, Color::rgb(avg, avg, avg)).unwrap();
        }
    }
}

pub fn sepia(img: &mut Image){
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

pub fn solarised_high(img: &mut Image){
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

pub fn solarised_low(img : &mut Image){
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

pub fn invert(img : &mut Image){
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

//saturate a color canal

pub fn full_red(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(255, color.g, color.b)).unwrap();
        }
    }
}

pub fn full_green(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, 255, color.b)).unwrap();
        }
    }
}

pub fn full_blue(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, color.g, 255)).unwrap();
        }
    }
}


//remove a color canal

pub fn no_red(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(0, color.g, color.b)).unwrap();
        }
    }
}

pub fn no_green(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, 0, color.b)).unwrap();
        }
    }
}

pub fn no_blue(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, color.g, 0)).unwrap();
        }
    }
}
