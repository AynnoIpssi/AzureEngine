pub struct Glyph {
    pub width: f32,
    pub height: f32,
    pub advance_width: f32,
    pub contours: Vec<Vec<(f32, f32)>>,
}

impl Glyph {
    pub fn new(width: f32, height: f32,advance_width: f32, contours: Vec<Vec<(f32, f32)>>,) -> Glyph {
        Glyph { width, height, advance_width, contours}
    }
}


