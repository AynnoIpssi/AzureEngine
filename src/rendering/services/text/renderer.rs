use crate::rendering::models::glyph::Glyph;
use crate::rendering::models::color::Color;
use crate::rendering::models::canvas::Canvas;
use crate::rendering::services::buffer::get_pixel_index;

const AA_SAMPLES: usize = 8;

#[inline]
fn srgb_to_linear(c: u8) -> f32 {
    let f = c as f32 / 255.0;
    if f <= 0.04045 { f / 12.92 } else { ((f + 0.055) / 1.055).powf(2.4) }
}

#[inline]
fn linear_to_srgb(c: f32) -> u8 {
    let s = if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
    (s.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

pub fn draw_glyph(glyph: &Glyph, x: u32, y: u32, color: &Color, canvas: &mut Canvas) {
    let height = glyph.height as u32;
    let width = (glyph.width.ceil() as u32).max(1);
    let cov_len = (width + 1) as usize;
    let mut coverage = vec![0.0f32; cov_len];

    // Pre-convert foreground color to linear light once
    let fg_r = srgb_to_linear(color.r);
    let fg_g = srgb_to_linear(color.g);
    let fg_b = srgb_to_linear(color.b);
    let fg_a = color.a as f32 / 255.0;

    for py in 0..height {
        coverage.fill(0.0);

        for s in 0..AA_SAMPLES {
            // Sample at 8 evenly spaced sub-pixel positions within this row
            let py_f = glyph.height - (py as f32 + (s as f32 + 0.5) / AA_SAMPLES as f32);
            let mut intersections: Vec<f32> = Vec::new();

            for contour in &glyph.contours {
                for i in 0..contour.len() {
                    let p1 = contour[i];
                    let p2 = contour[(i + 1) % contour.len()];
                    // Half-open interval avoids double-counting shared vertices
                    if (p1.1 <= py_f && p2.1 > py_f) || (p2.1 <= py_f && p1.1 > py_f) {
                        let t = (py_f - p1.1) / (p2.1 - p1.1);
                        intersections.push(p1.0 + t * (p2.0 - p1.0));
                    }
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for pair in intersections.chunks(2) {
                if pair.len() == 2 {
                    let x_start = pair[0].max(0.0);
                    let x_end = pair[1].min(width as f32);
                    if x_end <= x_start { continue; }

                    let px_start = x_start.floor() as usize;
                    let px_end = (x_end.ceil() as usize).min(cov_len - 1);

                    for px in px_start..=px_end {
                        // Exact fractional coverage of [px, px+1] inside the span
                        let left  = (px as f32).max(x_start);
                        let right = ((px + 1) as f32).min(x_end);
                        if px < cov_len {
                            coverage[px] += (right - left).max(0.0);
                        }
                    }
                }
            }
        }

        for (px, &cov) in coverage.iter().enumerate() {
            if cov <= 0.0 { continue; }

            let screen_x = x + px as u32;
            let screen_y = y + py;
            if screen_x >= canvas.width || screen_y >= canvas.height { continue; }

            // Combined alpha: glyph coverage × color's own alpha
            let alpha = (fg_a * cov / AA_SAMPLES as f32).min(1.0);

            let idx = get_pixel_index(screen_x, screen_y, canvas.width);
            let buf = &mut canvas.buffer;

            // Read background and convert to linear light for correct blending
            let bg_r = srgb_to_linear(buf[idx]);
            let bg_g = srgb_to_linear(buf[idx + 1]);
            let bg_b = srgb_to_linear(buf[idx + 2]);

            // Porter-Duff "over" in linear light, then back to sRGB
            buf[idx]     = linear_to_srgb(fg_r * alpha + bg_r * (1.0 - alpha));
            buf[idx + 1] = linear_to_srgb(fg_g * alpha + bg_g * (1.0 - alpha));
            buf[idx + 2] = linear_to_srgb(fg_b * alpha + bg_b * (1.0 - alpha));
            buf[idx + 3] = 255;
        }
    }
}
