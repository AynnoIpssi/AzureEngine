use crate::rendering::models::glyph::Glyph;

const FLATNESS: f32 = 0.5;
const MAX_DEPTH: u32 = 8;

struct GlyphOutline {
    contours: Vec<Vec<(f32, f32)>>,
    current: Vec<(f32, f32)>,
}

impl GlyphOutline {
    pub fn new() -> GlyphOutline {
        GlyphOutline {
            contours: Vec::new(),
            current: Vec::new(),
        }
    }
}

fn subdivide_quad(out: &mut Vec<(f32, f32)>, p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), depth: u32) {
    let dx = p2.0 - p0.0;
    let dy = p2.1 - p0.1;
    let len_sq = dx * dx + dy * dy;
    let d = if len_sq < 1e-6 {
        let ex = p1.0 - p0.0;
        let ey = p1.1 - p0.1;
        (ex * ex + ey * ey).sqrt()
    } else {
        ((p1.0 - p0.0) * dy - (p1.1 - p0.1) * dx).abs() / len_sq.sqrt()
    };

    if depth >= MAX_DEPTH || d < FLATNESS {
        out.push(p2);
        return;
    }
    let m01 = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5);
    let m12 = ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5);
    let mid = ((m01.0 + m12.0) * 0.5, (m01.1 + m12.1) * 0.5);
    subdivide_quad(out, p0, m01, mid, depth + 1);
    subdivide_quad(out, mid, m12, p2, depth + 1);
}

fn subdivide_cubic(out: &mut Vec<(f32, f32)>, p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), depth: u32) {
    let dx = p3.0 - p0.0;
    let dy = p3.1 - p0.1;
    let len_sq = dx * dx + dy * dy;
    let (d1, d2) = if len_sq < 1e-6 {
        let e1 = { let ex = p1.0 - p0.0; let ey = p1.1 - p0.1; (ex * ex + ey * ey).sqrt() };
        let e2 = { let ex = p2.0 - p0.0; let ey = p2.1 - p0.1; (ex * ex + ey * ey).sqrt() };
        (e1, e2)
    } else {
        let len = len_sq.sqrt();
        (
            ((p1.0 - p0.0) * dy - (p1.1 - p0.1) * dx).abs() / len,
            ((p2.0 - p0.0) * dy - (p2.1 - p0.1) * dx).abs() / len,
        )
    };

    if depth >= MAX_DEPTH || d1 + d2 < FLATNESS {
        out.push(p3);
        return;
    }
    let m01  = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5);
    let m12  = ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5);
    let m23  = ((p2.0 + p3.0) * 0.5, (p2.1 + p3.1) * 0.5);
    let m012 = ((m01.0 + m12.0) * 0.5, (m01.1 + m12.1) * 0.5);
    let m123 = ((m12.0 + m23.0) * 0.5, (m12.1 + m23.1) * 0.5);
    let mid  = ((m012.0 + m123.0) * 0.5, (m012.1 + m123.1) * 0.5);
    subdivide_cubic(out, p0, m01, m012, mid, depth + 1);
    subdivide_cubic(out, mid, m123, m23, p3, depth + 1);
}

impl ttf_parser::OutlineBuilder for GlyphOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        if !self.current.is_empty() {
            self.contours.push(self.current.clone());
            self.current.clear();
        }
        self.current.push((x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.current.push((x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let p0 = *self.current.last().unwrap_or(&(0.0, 0.0));
        subdivide_quad(&mut self.current, p0, (x1, y1), (x, y), 0);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let p0 = *self.current.last().unwrap_or(&(0.0, 0.0));
        subdivide_cubic(&mut self.current, p0, (x1, y1), (x2, y2), (x, y), 0);
    }

    fn close(&mut self) {
        if !self.current.is_empty() {
            self.contours.push(self.current.clone());
            self.current.clear();
        }
    }
}

pub fn exctract_glyph(font_data: &[u8], character: char, size: f32) -> Result<Glyph, String> {
    let face = ttf_parser::Face::parse(font_data, 0).map_err(|e| e.to_string())?;
    let glyph_id = face.glyph_index(character).ok_or("Glyph not found")?;

    let mut outline = GlyphOutline::new();
    face.outline_glyph(glyph_id, &mut outline);

    if !outline.current.is_empty() {
        outline.contours.push(outline.current.clone());
    }

    let bbox = face.glyph_bounding_box(glyph_id).ok_or("No bounding box")?;
    let scale = size / face.units_per_em() as f32;
    let width = (bbox.x_max - bbox.x_min) as f32 * scale;
    let height = (bbox.y_max - bbox.y_min) as f32 * scale;
    let advance_width = face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32 * scale;

    let x_min = bbox.x_min as f32;
    let y_min = bbox.y_min as f32;

    let scaled_contours = outline.contours.iter().map(|contour| {
        contour.iter().map(|(px, py)| {
            ((px - x_min) * scale, (py - y_min) * scale)
        }).collect()
    }).collect();

    Ok(Glyph::new(width, height, advance_width, scaled_contours))
}
