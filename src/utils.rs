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
