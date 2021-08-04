use ndarray;

pub struct Line {}

pub struct Point {}

pub struct Image<PixelKind> {
    data: ndarray::Array2<PixelKind>,
    width: usize,
    height: usize,
}

type BinaryImage = Image<bool>;
type RGBAImage = Image<pixel::RGBA<u8>>;
type RGBImage = Image<pixel::RGB<u8>>;

mod pixel {
    pub struct RGB<T> {
        r: T,
        g: T,
        b: T,
    }

    pub struct RGBA<T> {
        r: T,
        g: T,
        b: T,
        a: T,
    }
}

pub struct BinImage {
    width: u32,
    height: u32,
    data: Vec<bool>,
}

impl BinImage {
    pub fn new(width: u32, height: u32, data: Vec<bool>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

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
