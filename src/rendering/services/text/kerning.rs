use crate::rendering::models::glyph::Glyph;

pub fn get_advance(glyph: &Glyph) -> f32 {
    glyph.advance_width
}