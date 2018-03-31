fn main() {
    let img = raster::open("in.jpg").unwrap();

    raster::save(&image,"ou.jpg").unwrap();
}
