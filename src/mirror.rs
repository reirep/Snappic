use raster::Image;

pub fn vertical(img : &mut Image)
{
    let width = img.width;
    for y in 0..img.height {
        for x in 0..(width/2) {
            let color1 = img.get_pixel(x,y).unwrap();
            let color2 = img.get_pixel(width-x-1,y).unwrap();

            img.set_pixel(width-x-1,y,color1).unwrap();
            img.set_pixel(x,y, color2).unwrap();
        }
    }
}

pub fn horizontal(img : &mut Image){
    let height = img.height;

    for y in 0..(height/2) {
        for x in 0..img.width {
            let color1 = img.get_pixel(x,y).unwrap();
            let color2 = img.get_pixel(x,height-y-1).unwrap();

            img.set_pixel(x,height-y-1,color1).unwrap();
            img.set_pixel(x,y, color2).unwrap();
        }
    }
}
