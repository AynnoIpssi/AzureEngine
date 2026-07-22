use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::shapes::rect;

pub fn draw_rect(x: u32, y: u32, width: u32, height: u32, color: &Color, canvas: &mut Canvas) {

    rect::draw_rect(x, y, width, height, color, canvas);
}