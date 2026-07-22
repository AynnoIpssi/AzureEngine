use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::buffer::set_pixel;
use crate::rendering::services::shapes::circle::draw_circle_filled;

pub fn draw_rect(x: u32, y: u32, width: u32, height: u32, color: &Color, canvas: &mut Canvas) {

    for y in y..y + height {
        for x in x..x + width {
            set_pixel(&mut canvas.buffer, x, y, canvas.width, canvas.height, color);
        }
    }
}

pub fn draw_rect_rounded(x: u32, y: u32, width: u32, height: u32, radius: u32, color: &Color, canvas: &mut Canvas) {
    draw_circle_filled((x + radius) as i32, (y + radius) as i32, radius, color, canvas);
    draw_circle_filled((x + width - radius) as i32 , (y + radius) as i32, radius, color, canvas);
    draw_circle_filled((x + radius) as i32, (y + height - radius) as i32, radius, color, canvas);
    draw_circle_filled((x + width - radius) as i32, (y + height - radius) as i32, radius, color, canvas);
    draw_rect(x, y + radius, width, height - 2*radius, color, canvas);
    draw_rect(x + radius, y, width - 2*radius, radius, color, canvas);
    draw_rect(x + radius, y + height - radius, width - 2*radius, radius + 1, color, canvas);
}