
#[derive(Debug)]
pub struct Line{
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Line{
    pub fn new(x1:i32,y1:i32,x2:i32,y2:i32) -> Self{
        Line{
            x1,
            y1,
            x2,
            y2
        }
    }
}