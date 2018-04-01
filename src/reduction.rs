use raster::{Image,Color};

//black, red, green, yellow, blue, magenta, cyan, wite
static REF_RED   : [u8;8] = [  0,255,  0,255,  0,255,  0,255];
static REF_GREEN : [u8;8] = [  0,  0,255,255,  0,  0,255,255];
static REF_BLUE  : [u8;8] = [  0,  0,  0,  0,255,255,255,255];

pub fn to_tree_bits(img : &mut Image){
    for y in 0..img.height {
        for x in 0..img.width {
            let color = img.get_pixel(x,y).unwrap();
            let (r,g,b) = get_closest(color.r, color.g, color.b);
            img.set_pixel(x,y,Color::rgb(r,g,b)).unwrap();
        }
    }
}

fn get_closest(r : u8, g : u8, b : u8) -> (u8,u8,u8)
{
    let mut dist = 65535; 
    let mut index = 9;

    for i in 0..8 {
        let dist_current = sub(r,REF_RED[i]) as u16 + sub(g,REF_GREEN[i]) as u16 + sub(b,REF_BLUE[i]) as u16;
        if dist_current < dist {
            dist = dist_current;
            index = i;
        }
    }
    (REF_RED[index], REF_GREEN[index], REF_BLUE[index])
}

fn sub(a: u8, b : u8) -> u8
{
    if a > b {
        a-b
    }
    else {
        b-a
    }
}
