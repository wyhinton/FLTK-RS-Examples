#[derive(Copy, Clone, Debug)]
pub struct Offset{
    pub x: i32,
    pub y: i32,
}

impl Offset{
    pub fn new(x: i32 , y:i32 )->Self{
        return Offset{x: x, y: y}
    }
}
impl From<(i32, i32)> for Offset{
    fn from (val: (i32, i32))->Self{
        Offset{
            x: val.0,
            y: val.1,
        }
    }
}

