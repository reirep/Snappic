use raster::{Image,Color};

pub fn add(base : u8, change : u8) -> u8
{
    match base.checked_add(change) {
        Some(x) => x,
        None => 255
    }
}

pub fn sub(base : u8, change : u8) -> u8
{
    match base.checked_sub(change) {
        Some(x) => x,
        None => 0
    }
}

pub fn limit(nbr : f32) -> u8
{
    let value = nbr.round();
    if value < 0.0 {0}
    else if value > 255.0 {255}
    else {nbr as u8}
}

//les dimensions de kernel doivent être impaires
pub fn convolution(img: &mut Image, mult: f64, kernel : &Vec<Vec<f64>>)
{
    let mut dest = Image::blank(img.width,img.height);
    for y in 0..img.height {
        for x in 0..img.width {
            let mut r : f64 = 0.0;
            let mut g : f64 = 0.0;
            let mut b : f64 = 0.0;

            for kx in 0..kernel.len() {
                for ky in 0..kernel[0].len()  {
                    let ax : i32 = x + kx as i32 - (kernel.len()/2) as i32;
                    let ay : i32 = y + ky as i32 - (kernel[0].len()/2) as i32;
                    
                    let color = get_px(img, ax, ay);
                    
                    r += color.r as f64 * kernel[kx][ky];
                    g += color.g as f64 * kernel[kx][ky];
                    b += color.b as f64 * kernel[kx][ky];
                }
            }  
             
            r*=mult;
            g*=mult;
            b*=mult;

            dest.set_pixel(x,y,Color::rgb(r as u8, g as u8, b as u8)).unwrap();
        }
    }
    *img = dest;
}

//we use the mirror technique to get the pixel out of the picture for the convolution
fn get_px(img: &Image, x : i32, y : i32) -> Color 
{
    let real_x = inside_range(img.width, x);
    let real_y = inside_range(img.height, y);
    
    img.get_pixel(real_x, real_y).unwrap()
}

fn inside_range(max : i32, value : i32) -> i32
{
    let mut result = value;
    if result >= max {
        result *= -1;
    } 
    while result < 0 {
        result += max;
    }
    result
}
