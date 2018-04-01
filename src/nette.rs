use raster::Image;
use utils::convolution;

pub fn sharpen(img : &mut Image){
    let kernel: Vec<Vec<f64>> = vec![vec![0.0,-1.0,0.0],vec![-1.0,5.0,-1.0],vec![0.0,-1.0,0.0]];
    let mult : f64 = 1.0;

    convolution(img, mult, &kernel);

}
