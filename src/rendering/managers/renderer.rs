use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::shapes::rect;
use crate::rendering::services::text::loader::load_font;
use crate::rendering::services::text::glyph::exctract_glyph;
use crate::rendering::services::text::renderer::draw_glyph;
use crate::rendering::services::text::kerning::get_advance;

pub fn draw_rect(x: u32, y: u32, width: u32, height: u32, color: &Color, canvas: &mut Canvas) {

    rect::draw_rect(x, y, width, height, color, canvas);
}

pub fn draw_text(text: &str, font_path: &str, x: u32, y: u32, size: f32, color: &Color, canvas: &mut Canvas) -> Result<(), String> {
    let font_data = load_font(font_path)?;
    let mut cursor_x = x;

    for character in text.chars() {
        let glyph = exctract_glyph(&font_data, character, size)?;
        let advance = get_advance(&glyph);
        draw_glyph(&glyph, cursor_x, y, color, canvas);
        cursor_x += advance as u32;
    }

    Ok(())
}