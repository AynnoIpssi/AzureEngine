pub mod platform;
pub mod rendering;

#[cfg(test)]
mod tests8 {
    use crate::platform::wayland::managers::window_manager::window_create;
    use crate::platform::wayland::managers::surface_manager::run_event_loop;
    use crate::platform::wayland::managers::surface_manager::{commit, damage_buffer};
    use crate::platform::wayland::managers::shared_memory_manager::unmap_memory;
    use crate::platform::wayland::managers::xdg_manager::attach;
    use crate::rendering::models::canvas::Canvas;
    use crate::rendering::models::color::Color;
    use crate::rendering::managers::renderer::draw_rect;
    use crate::rendering::services::shapes::rect::draw_rect_rounded;
    use crate::rendering::services::shapes::line::{draw_line_horizontal, draw_line_vertical, draw_line};
    use crate::rendering::services::shapes::circle::{draw_circle, draw_circle_filled};
    use crate::rendering::services::effects::{blend_pixel, draw_gradient_horizontal, draw_vertical_gradient, draw_angular_gradiant};
    use crate::rendering::managers::renderer::draw_text;

    #[test]
    fn test_empty_window() {
        let mut window = window_create(800, 800).expect("Failed to create window");

        let xdg_surface_id = window.xdg_surface_id();
        let xdg_toplevel_id = window.xdg_toplevel_id();
        let surface_id = window.surface_id();
        let xdg_wm_id = window.xdg_wm_id();

        let mut canvas = Canvas::new(800, 800);

        // Fond sombre
        canvas.buffer.chunks_mut(4).for_each(|p| {
            p[0] = 30;
            p[1] = 30;
            p[2] = 50;
            p[3] = 255;
        });

        let red   = Color::new(255, 0, 0, 255);
        let green = Color::new(0, 255, 0, 255);

        // ─── Ligne 1 : Formes (y = 50..200) ───────────────────────────────────
        // Rectangle plein
        draw_rect(20, 50, 160, 120, &red, &mut canvas);

        // Rectangle arrondi
        draw_rect_rounded(220, 50, 160, 120, 15, &red, &mut canvas);

        // Cercle contour (AA)
        draw_circle(520, 110, 60, &green, &mut canvas);

        // Cercle rempli
        draw_circle_filled(680, 110, 60, &green, &mut canvas);

        // ─── Ligne 2 : Lignes (y = 250..400) ──────────────────────────────────
        // Ligne horizontale
        draw_line_horizontal(20, 750, 300, &red, &mut canvas);

        // Ligne verticale
        draw_line_vertical(320, 390, 100, &red, &mut canvas);

        // Ligne diagonale (Bresenham AA)
        draw_line(400, 250, 750, 390, &red, &mut canvas);

        // ─── Ligne 3 : Effets (y = 430..800) ──────────────────────────────────
        // Alpha blending
        let color_blend = Color::new(255, 0, 0, 128);
        for y in 430..530u32 {
            for x in 20..200u32 {
                blend_pixel(&mut canvas.buffer, x, y, 800, 800, &color_blend);
            }
        }

        // Dégradé horizontal
        let color_start = Color::new(255, 0, 0, 255);
        let color_end   = Color::new(0, 0, 255, 255);
        draw_gradient_horizontal(220, 430, 180, 100, &color_start, &color_end, &mut canvas);

        // Dégradé vertical
        let color_start = Color::new(255, 0, 0, 255);
        let color_end   = Color::new(0, 0, 255, 255);
        draw_vertical_gradient(420, 430, 180, 100, &color_start, &color_end, &mut canvas);

        // Dégradé angulaire (60°)
        let color_start = Color::new(255, 0, 0, 255);
        let color_end   = Color::new(0, 0, 255, 255);
        draw_angular_gradiant(620, 430, 160, 100, 60.0, &color_start, &color_end, &mut canvas);

        draw_text("Hello", "src/Sora-VariableFont_wght.ttf", 100, 700, 32.0, &red, &mut canvas)
        .expect("Failed to draw text");

        // Copie canvas → mémoire Wayland
        unsafe {
            std::ptr::copy_nonoverlapping(
                canvas.buffer.as_ptr(),
                window.ptr(),
                canvas.buffer.len(),
            );
        }

        // Affichage
        let win_width  = window.width();
        let win_height = window.height();
        let buffer_id  = window.buffer_id();
        attach(window.connection_mut(), surface_id, buffer_id).expect("Failed to attach");
        damage_buffer(window.connection_mut(), surface_id, 0, 0, win_width, win_height).expect("Failed to damage");
        commit(window.connection_mut(), surface_id).expect("Failed to commit");

        run_event_loop(&mut window, xdg_surface_id, xdg_toplevel_id, surface_id, xdg_wm_id)
            .expect("Event loop failed");

        unmap_memory(window.ptr(), (window.width() * window.height() * 4) as usize)
            .expect("Failed to unmap memory");
    }
}