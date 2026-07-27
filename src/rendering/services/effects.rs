use crate::rendering::{models::{canvas::Canvas, color::Color}, services::buffer::{get_pixel_index, set_pixel}};

//-------------------------(Opacity)------------------------------>

pub fn blend_pixel(buffer: &mut Vec<u8>, x: u32, y: u32, width: u32, height: u32, color: &Color){
    if x >= width || y >= height { return; }

    let index_pixel = get_pixel_index(x,y,width);
    
    let bg_r = buffer[index_pixel];
    let bg_g = buffer[index_pixel + 1];
    let bg_b = buffer[index_pixel + 2];

    let r = ((color.r as u16 * color.a as u16 + bg_r as u16 * (255 - color.a as u16)) / 255) as u8; 
    let g = ((color.g as u16 * color.a as u16 + bg_g as u16 * (255 - color.a as u16)) / 255) as u8;
    let b = ((color.b  as u16 * color.a as u16 + bg_b as u16 * (255 - color.a as u16)) / 255) as u8;

    buffer[index_pixel] = r;
    buffer[index_pixel + 1] = g;
    buffer[index_pixel + 2] = b;
    buffer[index_pixel + 3] = 255;
}

//-----------------------------(Horizontal Gradient)---------------------------------->

pub fn draw_gradient_horizontal(x: u32, y: u32, width: u32, height: u32, color_start: &Color, color_end: &Color, canvas: &mut Canvas){
    for py in y..y + height {
        for px in x..x + width {
            let t = (px - x) as f32 / width as f32;
            let r = (color_start.r as f32 * (1.0 - t) + color_end.r as f32 * t) as u8;
            let g = (color_start.g as f32 * (1.0 - t) + color_end.g as f32 * t) as u8;
            let b = (color_start.b as f32 * (1.0 - t) + color_end.b as f32 * t) as u8;
            let mut color = Color::new(r, g, b, 255);
            set_pixel(&mut canvas.buffer, px, py, canvas.width, canvas.height, &color);
        }
    }
}

//-----------------------------(vertical Gradient)---------------------------------->

pub fn draw_vertical_gradient(x: u32, y: u32, width: u32, height: u32, color_start: &Color, color_end: &Color, canvas: &mut Canvas){
    for px in x..x + width { 
        for py in y..y + height  {
            let t = (py - y) as f32 / height as f32;
            let r = (color_start.r as f32 * (1.0 - t) + color_end.r as f32 * t) as u8;
            let g = (color_start.g as f32 * (1.0 - t) + color_end.g as f32 * t) as u8;
            let b = (color_start.b as f32 * (1.0 - t) + color_end.b as f32 * t) as u8;
            let mut color = Color::new(r, g, b, 255);
            set_pixel(&mut canvas.buffer, px, py, canvas.width, canvas.height, &color);
        }
    }
}

//------------------------------------(Angular gradiant)--------------------------------------->

pub fn draw_angular_gradiant(x: u32, y: u32, width: u32, height: u32, angle: f32, color_start: &Color, color_end: &Color, canvas: &mut Canvas){
    let radians = angle * std::f32::consts::PI / 180.0;
    let dx = radians.cos();
    let dy = radians.sin();

    for px in x..x + width{
        for py in y..y + height{
            let t = (px as f32 - x as f32) * dx + (py as f32 - y as f32) * dy;
            let max_t = width as f32 * dx.abs() + height as f32 * dy.abs();
            let t = (t / max_t).clamp(0.0, 1.0);

            let r = (color_start.r as f32 * (1.0 - t) + color_end.r as f32 * t) as u8;
            let g = (color_start.g as f32 * (1.0 - t) + color_end.g as f32 * t) as u8;
            let b = (color_start.b as f32 * (1.0 - t) + color_end.b as f32 * t) as u8;
            let mut color = Color::new(r, g, b, 255);
            set_pixel(&mut canvas.buffer, px, py, canvas.width, canvas.height, &color);
        }
    }
}

//----------------------------------------(Anti Analiasing)--------------------------------------------->

pub fn apply_aa(distance: f32) -> u8{

    if distance < 0.0 {
        255  
    } else if distance > 1.0 {
        0
    } else {
        ((1.0 - distance) * 255.0) as u8
    }
}