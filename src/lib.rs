     pub mod platform;
    pub mod rendering;
     // //
    // // pub fn add(left: u64, right: u64) -> u64 {
    // //     left + right
    // // }
    // //
    // // #[cfg(test)]
    // // mod tests1 {
    // //     use super::*;
    // //
    // //     #[test]
    // //     fn it_works() {
    // //         let result = add(2, 2);
    // //         assert_eq!(result, 4);
    // //     }
    // // }
    // //
    // // #[cfg(test)]
    // // mod tests {
    // //     use crate::platform::wayland::managers::connection_manager::connect;
    // //
    // //     #[test]
    // //     fn test_wayland_connection() {
    // //         let result = connect();
    // //         assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
    // //     }
    // // }
    // //
    // // #[cfg(test)]
    // // mod tests3 {
    // //     use crate::platform::wayland::managers::connection_manager::connect;
    // //     use crate::platform::wayland::managers::registry_manager::get_registry;
    // //
    // //     #[test]
    // //     fn test_wayland_registry() {
    // //         let mut connection = connect().expect("Failed to connect to Wayland");
    // //         let registry = get_registry(&mut connection).expect("Failed to get registry");
    // //         println!("Globals found: {}", registry.get_globals().len());
    // //         for global in registry.get_globals() {
    // //             println!("  {} v{} (id={})", global.name(), global.version(), global.id());
    // //         }
    // //         assert!(!registry.get_globals().is_empty(), "No globals found");
    // //     }
    // // }
    // //
    // // #[cfg(test)]
    // // mod test_memory {
    // //     use crate::platform::wayland::managers::shared_memory_manager::create_shared_memory;
    // //
    // //     #[test]
    // //     fn test_memory() {
    // //         let result = create_shared_memory(4096);
    // //         assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
    // //     }
    // // }
    //
    // #[cfg(test)]
    // mod tests {
    //     use crate::platform::wayland::managers::connection_manager::connect;
    //     use crate::platform::wayland::managers::shm_manager::bind_shm;
    //
    //     #[test]
    //     fn test_bind_shm() {
    //         let mut connection = connect().expect("Failed to connect to Wayland");
    //         let shm_id = bind_shm(&mut connection).expect("Failed to bind wl_shm");
    //         println!("Bound wl_shm with new id: {}", shm_id);
    //         assert_eq!(shm_id, 4);
    //     }
    // }
    //
    // // #[cfg(test)]
    // // mod tests2 {
    // //     use crate::platform::wayland::managers::connection_manager::connect;
    // //     use crate::platform::wayland::managers::shared_memory_manager::create_shared_memory;
    // //     use crate::platform::wayland::managers::shm_manager::{bind_shm, create_shm_pool};
    // //
    // //     #[test]
    // //     fn test_create_shm_pool() {
    // //         let mut connection = connect().expect("Failed to connect to Wayland");
    // //         let memory = create_shared_memory(1024).expect("Failed to create shared memory");
    // //         bind_shm(&mut connection).expect("Failed to bind wl_shm");
    // //         let pool_id = create_shm_pool(&mut connection, memory.fd(), memory.size())
    // //             .expect("Failed to create shm pool");
    // //         println!("Created shm pool with id: {}", pool_id);
    // //         assert_eq!(pool_id, 5);
    // //     }
    // // }
    //
    // #[cfg(test)]
    // mod tests3 {
    //     use crate::platform::wayland::managers::connection_manager::connect;
    //     use crate::platform::wayland::managers::shared_memory_manager::create_shared_memory;
    //     use crate::platform::wayland::managers::shm_manager::{create_shm_pool, create_buffer};
    //     use crate::platform::wayland::managers::bind_manager::bind_global;
    //
    //     #[test]
    //     fn test_create_buffer() {
    //         let mut connection = connect().expect("Failed to connect to Wayland");
    //         let memory = create_shared_memory(1024).expect("Failed to create shared memory");
    //         bind_global(&mut connection, 2, "wl_shm", 2, 4).expect("Failed to bind wl_shm");
    //         let pool_id = create_shm_pool(&mut connection, memory.fd(), memory.size())
    //             .expect("Failed to create shm pool");
    //         let buffer_id = create_buffer(&mut connection, pool_id, 16, 16)
    //             .expect("Failed to create buffer");
    //         println!("Created buffer with id: {}", buffer_id);
    //         assert_eq!(buffer_id, 6);
    //     }
    // }
    //
    // #[cfg(test)]
    // mod tests4 {
    //     use crate::platform::wayland::managers::connection_manager::connect;
    //     use crate::platform::wayland::managers::bind_manager::bind_global;
    //     use crate::platform::wayland::managers::compositor_manager::create_surface;
    //
    //     #[test]
    //     fn test_create_surface() {
    //         let mut connection = connect().expect("Failed to connect to Wayland");
    //         let compositor_id = bind_global(&mut connection, 1, "wl_compositor", 4, 7)
    //             .expect("Failed to bind wl_compositor");
    //         let surface_id = create_surface(&mut connection, compositor_id)
    //             .expect("Failed to create surface");
    //         println!("Created surface with id: {}", surface_id);
    //         assert_eq!(surface_id, 8);
    //     }
    // }
    //
    //  #[cfg(test)]
    //  mod tests5 {
    //      use crate::platform::wayland::managers::connection_manager::connect;
    //      use crate::platform::wayland::managers::bind_manager::bind_global;
    //      use crate::platform::wayland::managers::compositor_manager::create_surface;
    //      use crate::platform::wayland::managers::xdg_manager::get_xdg_surface;
    //
    //      #[test]
    //      fn test_get_xdg_surface() {
    //          let mut connection = connect().expect("Failed to connect to Wayland");
    //          let compositor_id = bind_global(&mut connection, 1, "wl_compositor", 4, 7)
    //              .expect("Failed to bind wl_compositor");
    //          let surface_id = create_surface(&mut connection, compositor_id)
    //              .expect("Failed to create surface");
    //          let xdg_wm_base_id = bind_global(&mut connection, 9, "xdg_wm_base", 1, 9)
    //              .expect("Failed to bind xdg_wm_base");
    //          let xdg_surface_id = get_xdg_surface(&mut connection, xdg_wm_base_id, surface_id)
    //              .expect("Failed to get xdg_surface");
    //          println!("Created xdg_surface with id: {}", xdg_surface_id);
    //          assert_eq!(xdg_surface_id, 10);
    //      }
    //  }
    //
    // #[cfg(test)]
    // mod tests6 {
    //     use crate::platform::wayland::managers::connection_manager::connect;
    //     use crate::platform::wayland::managers::bind_manager::bind_global;
    //     use crate::platform::wayland::managers::compositor_manager::create_surface;
    //     use crate::platform::wayland::managers::xdg_manager::{get_xdg_surface, get_toplevel};
    //
    //     #[test]
    //     fn test_get_toplevel() {
    //         let mut connection = connect().expect("Failed to connect to Wayland");
    //         let compositor_id = bind_global(&mut connection, 1, "wl_compositor", 4, 7)
    //             .expect("Failed to bind wl_compositor");
    //         let surface_id = create_surface(&mut connection, compositor_id)
    //             .expect("Failed to create surface");
    //         let xdg_wm_base_id = bind_global(&mut connection, 9, "xdg_wm_base", 1, 9)
    //             .expect("Failed to bind xdg_wm_base");
    //         let xdg_surface_id = get_xdg_surface(&mut connection, xdg_wm_base_id, surface_id)
    //             .expect("Failed to get xdg_surface");
    //         let toplevel_id = get_toplevel(&mut connection, xdg_surface_id)
    //             .expect("Failed to get toplevel");
    //         println!("Created toplevel with id: {}", toplevel_id);
    //         assert_eq!(toplevel_id, 11);
    //     }
    // }
     //
     // #[cfg(test)]
     // mod tests7 {
     //     use crate::platform::wayland::managers::window_manager::window_create;
     //     use crate::platform::wayland::managers::surface_manager::run_event_loop;
     //     use crate::platform::wayland::managers::shared_memory_manager::unmap_memory;
     //
     //     #[test]
     //     fn test_empty_window() {
     //         let mut window = window_create(800, 600)
     //             .expect("Failed to create window");
     //
     //         let xdg_surface_id = window.xdg_surface_id();
     //         let xdg_toplevel_id = window.xdg_toplevel_id();
     //         let surface_id = window.surface_id();
     //         let xdg_wm_id = window.xdg_wm_id();
     //
     //         run_event_loop(
     //             &mut window,
     //             xdg_surface_id,
     //             xdg_toplevel_id,
     //             surface_id,
     //             xdg_wm_id,
     //         ).expect("Event loop failed");
     //
     //         unmap_memory(window.ptr(), (window.width() * window.height() * 4) as usize)
     //             .expect("Failed to unmap memory");
     //     }
     // }

     #[cfg(test)]
     mod tests8 {
         use crate::platform::wayland::managers::window_manager::window_create;
         use crate::platform::wayland::managers::surface_manager::run_event_loop;
         use crate::platform::wayland::managers::shared_memory_manager::unmap_memory;
         use crate::rendering::models::canvas::Canvas;
         use crate::rendering::models::color::Color;
         use crate::rendering::managers::renderer::draw_rect;
         use crate::platform::wayland::managers::surface_manager::{commit, damage_buffer};
         use crate::platform::wayland::managers::xdg_manager::attach;
         use crate::rendering::services::shapes::line::{draw_line_horizontal, draw_line_vertical, draw_line};
         use crate::rendering::services::shapes::circle::{draw_circle, draw_circle_filled};
         use crate::rendering::services::shapes::rect::draw_rect_rounded;

         #[test]
         fn test_empty_window() {
             let mut window = window_create(800, 600)
                 .expect("Failed to create window");

             let xdg_surface_id = window.xdg_surface_id();
             let xdg_toplevel_id = window.xdg_toplevel_id();
             let surface_id = window.surface_id();
             let xdg_wm_id = window.xdg_wm_id();



             let mut canvas=Canvas::new(800, 600);
             let mut color = Color::new(255, 0, 0, 255); // rouge
             canvas.buffer.chunks_mut(4).for_each(|p| {
                 p[0] = 30;   // B
                 p[1] = 30;   // G
                 p[2] = 50; // R
                 p[3] = 255; // A
             });

             let x = 100;
             let y = 50;
             let width = 200;
             let height = 100;

             draw_rect(x, y, width, height, &color, &mut canvas);

             let x = 400;
             let y = 50;
             let width = 200;
             let height = 100;
             let radius = 10;

             draw_rect_rounded(x, y, width, height, radius, &color, &mut canvas);

             let x = 310;
             let x_end = 450;
             let y = 50;

             draw_line_vertical(x, x_end, y, &color, &mut canvas);

             let x = 460;
             let y = 50;
             let y_end = 190;

             draw_line_horizontal(y, y_end, x, &color, &mut canvas);

             let x: i32 = 460;
             let y: i32 = 50;
             let y_end: i32 = 190;
             let x_end: i32 = 500;
             
             draw_line(x, y, x_end, y_end, &color, &mut canvas);

             let cx = 300;
             let cy = 300;
             let radius = 100;
             let color = Color::new(31, 255, 0, 255);

             draw_circle(cx, cy, radius, &color, &mut canvas);

             let cx = 600;
             let cy = 300;
             let radius = 100;
             let color = Color::new(0, 255, 0, 255);

             draw_circle_filled(cx, cy, radius, &color, &mut canvas);

            use crate::rendering::services::buffer::get_pixel_index;
            println!("centre cercle filled: {},{},{},{}",
                canvas.buffer[get_pixel_index(700, 300, 800)],
                canvas.buffer[get_pixel_index(700, 300, 800) + 1],
                canvas.buffer[get_pixel_index(700, 300, 800) + 2],
                canvas.buffer[get_pixel_index(700, 300, 800) + 3],
            );

             unsafe {
                 std::ptr::copy_nonoverlapping(
                     canvas.buffer.as_ptr(),
                     window.ptr(),
                     canvas.buffer.len(),
                 );
             }


             let win_width = window.width();
             let win_height = window.height();
             let buffer_id = window.buffer_id();
             attach(window.connection_mut(), surface_id, buffer_id).expect("Failed to attach");
             damage_buffer(window.connection_mut(), surface_id, 0, 0, win_width, win_height).expect("Failed to damage");
             commit(window.connection_mut(), surface_id).expect("Failed to commit");
             run_event_loop(
                 &mut window,
                 xdg_surface_id,
                 xdg_toplevel_id,
                 surface_id,
                 xdg_wm_id,
             ).expect("Event loop failed");

             unmap_memory(window.ptr(), (window.width() * window.height() * 4) as usize)
                 .expect("Failed to unmap memory");


         }
     }