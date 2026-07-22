pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32,) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec!(0u8; (width * height * 4) as usize),
        }
    }
}