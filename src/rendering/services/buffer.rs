use crate::rendering::models::color::Color;

pub fn get_pixel_index(x: u32, y: u32, width: u32) -> usize {
    return ((y * width + x) * 4) as usize;
}

    pub fn set_pixel(buffer: &mut Vec<u8>, x: u32, y: u32, width: u32, height: u32, color: &Color) {

        if x >= width || y >= height {
            return;
        }

        let index = get_pixel_index(x, y, width);
        buffer[index] = color.r;
        buffer[index + 1] = color.g;
        buffer[index +2] = color.b;
        buffer[index + 3] = color.a;
    }