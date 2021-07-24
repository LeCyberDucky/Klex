pub struct Line {}

pub struct Point {}

pub struct BinaryImage {
    width: u32,
    height: u32,
    data: Vec<bool>
}

impl BinaryImage {
    pub fn new(width: u32, height: u32, data: Vec<bool>) -> Self { Self { width, height, data } }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &Vec<bool> {
        &self.data
    }
}