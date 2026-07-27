use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::buffer::set_pixel;
use  crate::rendering::services::effects::{apply_aa,blend_pixel};


//-----------------------(HorizontalLine)------------------------->

pub fn draw_line_horizontal(x: u32, x_end: u32,y: u32, color: &Color, canvas: &mut Canvas) {
    for x in x..x_end {
        set_pixel(&mut canvas.buffer, x, y, canvas.width, canvas.height, color);
    }
}

//-----------------------(VerticalLine)------------------------->

pub fn draw_line_vertical(y: u32, y_end: u32,x: u32, color: &Color, canvas: &mut Canvas) {
    for y in y..y_end {
        set_pixel(&mut canvas.buffer, x, y, canvas.width, canvas.height, color);
    }
}

//-----------------------(AllDirectionLine)------------------------->

pub fn draw_line(x: i32, y: i32, x_end: i32, y_end: i32, color: &Color, canvas: &mut Canvas) {
    let dx = (x_end - x) as f32;
    let dy = (y_end - y) as f32;
    let length = (dx * dx + dy * dy).sqrt();
    
    let min_x = x.min(x_end) - 1;
    let max_x = x.max(x_end) + 1;
    let min_y = y.min(y_end) - 1;
    let max_y = y.max(y_end) + 1;
    
    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let dist = ((dy * px as f32 - dx * py as f32 + x_end as f32 * y as f32 - y_end as f32 * x as f32) / length).abs();
            let alpha = apply_aa(dist - 0.5);
            if alpha > 0 && px >= 0 && py >= 0 {
                let blended_color = Color::new(color.r, color.g, color.b, alpha);
                blend_pixel(&mut canvas.buffer, px as u32, py as u32, canvas.width, canvas.height, &blended_color);
            }
        }
    }
}