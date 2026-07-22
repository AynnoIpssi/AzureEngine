use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::buffer::set_pixel;
use crate::rendering::services::shapes::line::draw_line_horizontal;

pub fn draw_circle(cx: i32, cy: i32, radius: u32, color: &Color, canvas: &mut Canvas){
    let mut x = 0;
    let mut y = radius as i32;
    let mut d = 3 - 2 * radius as i32;

    while x <= y {
        set_pixel(&mut canvas.buffer, (cx + x) as u32, (cy + y) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx - x) as u32, (cy - y) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx + x) as u32, (cy - y) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx - x) as u32, (cy + y) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx + y) as u32, (cy + x) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx - y) as u32, (cy - x) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx + y) as u32, (cy - x) as u32, canvas.width, canvas.height, color);
        set_pixel(&mut canvas.buffer, (cx - y) as u32, (cy + x) as u32, canvas.width, canvas.height, color);

        if d < 0{

            d += (4 * x + 6);
        } else {

            y -= 1; 
            d += 4 * (x - y) + 10;
        }
            
        x += 1;
    }
}


pub fn draw_circle_filled(cx: i32, cy: i32, radius: u32, color: &Color, canvas: &mut Canvas){
    let mut x = 0;
    let mut y = radius as i32;
    let mut d = 3 - 2 * radius as i32;

    while x <= y {
        draw_line_horizontal((cx - x) as u32, (cx + x) as u32, (cy + y) as u32, color, canvas);
        draw_line_horizontal((cx - x) as u32, (cx + x) as u32, (cy - y) as u32, color, canvas);
        draw_line_horizontal((cx - y) as u32, (cx + y) as u32, (cy + x) as u32, color, canvas);
        draw_line_horizontal((cx - y) as u32, (cx + y) as u32, (cy - x) as u32, color, canvas);
        
        if d < 0 {
            d += 4 * x + 6;
        } else {
            y -= 1;
            d += 4 * (x - y) + 10;
        }
        x += 1;
    }
}