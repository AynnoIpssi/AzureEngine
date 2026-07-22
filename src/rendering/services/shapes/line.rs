use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::buffer::set_pixel;


pub fn draw_line_horizontal(x: u32, x_end: u32,y: u32, color: &Color, canvas: &mut Canvas) {
    for x in x..x_end {
        set_pixel(&mut canvas.buffer, x, y, canvas.width, canvas.height, color);
    }
}

pub fn draw_line_vertical(y: u32, y_end: u32,x: u32, color: &Color, canvas: &mut Canvas) {
    for y in y..y_end {
        set_pixel(&mut canvas.buffer, x, y, canvas.width, canvas.height, color);
    }
}

pub fn draw_line(x: i32, y: i32, x_end: i32, y_end: i32, color: &Color, canvas: &mut Canvas) {
    let dx = (x_end - x).abs();
    let dy = (y_end - y).abs();
    let sx = if x < x_end { 1 } else { -1 };
    let sy = if y < y_end { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x1 = x;
    let mut y1 = y;

    while x1 != x_end || y1 != y_end {
        set_pixel(&mut canvas.buffer, x1 as u32, y1 as u32, canvas.width, canvas.height, color);
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}