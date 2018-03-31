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
