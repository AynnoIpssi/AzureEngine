use crate::rendering::models::canvas::Canvas;
use crate::rendering::models::color::Color;
use crate::rendering::services::effects::apply_aa;
use crate::rendering::services::effects::blend_pixel;


//-----------------------(EmptyCirle)---------------------------->

pub fn draw_circle(cx: i32, cy: i32, radius: u32, color: &Color, canvas: &mut Canvas) {
    let r = radius as i32;
    for py in (cy - r - 1)..(cy + r + 1) {
        for px in (cx - r - 1)..(cx + r + 1) {
            let dist = (((px - cx).pow(2) + (py - cy).pow(2)) as f32).sqrt() - radius as f32;
            let dist_abs = dist.abs();
            let alpha = apply_aa(dist_abs - 0.5);
            if alpha > 0 && px >= 0 && py >= 0 {
                let blended_color = Color::new(color.r, color.g, color.b, alpha);
                blend_pixel(&mut canvas.buffer, px as u32, py as u32, canvas.width, canvas.height, &blended_color);
            }
        }
    }
}


//-----------------------(FilledCirle)---------------------------->

pub fn draw_circle_filled(cx: i32, cy: i32, radius: u32, color: &Color, canvas: &mut Canvas) {
    let r = radius as i32;
    for py in (cy - r - 1)..(cy + r + 1) {
        for px in (cx - r - 1)..(cx + r + 1) {
            let dist = (((px - cx).pow(2) + (py - cy).pow(2)) as f32).sqrt() - radius as f32;
            let alpha = apply_aa(dist);
            if alpha > 0 && px >= 0 && py >= 0 {
                let blended_color = Color::new(color.r, color.g, color.b, alpha);
                blend_pixel(&mut canvas.buffer, px as u32, py as u32, canvas.width, canvas.height, &blended_color);
            }
        }
    }
}