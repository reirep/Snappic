use raster::{Image,Color};

//set brightness

pub fn brightness_add(img : &mut Image, change : u8){
    for x in 0..img.width {
        for y in 0..img.height {
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
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(
                    sub(color.r,change),
                    sub(color.g,change),
                    sub(color.b,change)
                )).unwrap();
        }
    }
}

fn add(base : u8, change : u8) -> u8
{
    match base.checked_add(change) {
        Some(x) => x,
        None => 255
    }
}

fn sub(base : u8, change : u8) -> u8
{
    match base.checked_sub(change) {
        Some(x) => x,
        None => 0
    }
}

//saturate a color canal

pub fn full_red(img : &mut Image){
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(255, color.g, color.b)).unwrap();
        }
    }
}

pub fn full_green(img : &mut Image){
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, 255, color.b)).unwrap();
        }
    }
}

pub fn full_blue(img : &mut Image){
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, color.g, 255)).unwrap();
        }
    }
}


//remove a color canal

pub fn no_red(img : &mut Image){
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(0, color.g, color.b)).unwrap();
        }
    }
}

pub fn no_green(img : &mut Image){
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, 0, color.b)).unwrap();
        }
    }
}

pub fn no_blue(img : &mut Image){
    for x in 0..img.width {
        for y in 0..img.height {
            let color = img.get_pixel(x,y).unwrap();
            img.set_pixel(x,y,Color::rgb(color.r, color.g, 0)).unwrap();
        }
    }
}
