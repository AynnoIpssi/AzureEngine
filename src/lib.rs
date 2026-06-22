 mod platform;
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

 #[cfg(test)]
 mod tests7 {
     use std::thread;
     use std::time::Duration;
     use crate::platform::wayland::managers::connection_manager::connect;
     use crate::platform::wayland::managers::registry_manager::{get_registry, find_global};
     use crate::platform::wayland::managers::bind_manager::bind_global;
     use crate::platform::wayland::managers::shared_memory_manager::create_shared_memory;
     use crate::platform::wayland::managers::shm_manager::{create_shm_pool, create_buffer};
     use crate::platform::wayland::managers::compositor_manager::create_surface;
     use crate::platform::wayland::managers::xdg_manager::{get_xdg_surface, get_toplevel, ack_configure, attach};
     use crate::platform::wayland::managers::surface_manager::{commit, wait_for_configure};

     #[test]
     fn test_empty_window() {
         let mut connection = connect().expect("Failed to connect");
         println!("1: connected");

         let registry = get_registry(&mut connection).expect("get registry");
         println!("1b: registry created");

         let shm_name = find_global(&registry, "wl_shm").expect("wl_shm not found");
         let compositor_name = find_global(&registry, "wl_compositor").expect("wl_compositor not found");
         let xdg_wm_base_name = find_global(&registry, "xdg_wm_base").expect("xdg_wm_base not found");
         println!("1c: globals found: shm={}, compositor={}, xdg_wm_base={}", shm_name, compositor_name, xdg_wm_base_name);

         bind_global(&mut connection, shm_name, "wl_shm", 2, 4).expect("bind wl_shm");
         println!("2: wl_shm bound");

         let memory = create_shared_memory(160000).expect("create memory");
         println!("3: memory created");

         let ptr = unsafe {
             libc::mmap(
                 std::ptr::null_mut(),
                 memory.size(),
                 libc::PROT_READ | libc::PROT_WRITE,
                 libc::MAP_SHARED,
                 memory.fd(),
                 0,
             )
         };
         let pixels = unsafe {
             std::slice::from_raw_parts_mut(ptr as *mut u8, memory.size())
         };
         pixels.fill(0xFF);

         let pool_id = create_shm_pool(&mut connection, memory.fd(), memory.size()).expect("create pool");
         println!("4: pool created, id={}", pool_id);

         let buffer_id = create_buffer(&mut connection, pool_id, 200, 200).expect("create buffer");
         println!("5: buffer created, id={}", buffer_id);

         let compositor_id = bind_global(&mut connection, compositor_name, "wl_compositor", 4, 7).expect("bind compositor");
         println!("6: compositor bound, id={}", compositor_id);

         let surface_id = create_surface(&mut connection, compositor_id).expect("create surface");
         println!("7: surface created, id={}", surface_id);

         let xdg_wm_base_id = bind_global(&mut connection, xdg_wm_base_name, "xdg_wm_base", 1, 9).expect("bind xdg_wm_base");
         println!("8: xdg_wm_base bound, id={}", xdg_wm_base_id);

         let xdg_surface_id = get_xdg_surface(&mut connection, xdg_wm_base_id, surface_id).expect("get xdg_surface");
         println!("9: xdg_surface created, id={}", xdg_surface_id);

         get_toplevel(&mut connection, xdg_surface_id).expect("get toplevel");
         println!("10: toplevel created");

         commit(&mut connection, surface_id).expect("initial commit");
         println!("11: initial commit sent");

         let serial = wait_for_configure(&mut connection, xdg_surface_id).expect("wait for configure");
         println!("12: got configure, serial={}", serial);

         ack_configure(&mut connection, xdg_surface_id, serial).expect("ack configure");
         println!("13: ack sent");

         attach(&mut connection, surface_id, buffer_id).expect("attach buffer");
         println!("14: buffer attached");

         commit(&mut connection, surface_id).expect("final commit");
         println!("15: final commit, window should be visible!");

         thread::sleep(Duration::from_secs(10   ));
     }
 }