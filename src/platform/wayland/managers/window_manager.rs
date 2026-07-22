    use crate::platform::wayland::models::connection::WaylandConnection;
    use crate::platform::wayland::models::window::Window;
    use crate::platform::wayland::managers::connection_manager::connect;
    use crate::platform::wayland::managers::registry_manager::{get_registry, find_global};
    use crate::platform::wayland::managers::bind_manager::bind_global;
    use crate::platform::wayland::managers::shared_memory_manager::{create_shared_memory, map_memory};
    use crate::platform::wayland::managers::shm_manager::{create_shm_pool, create_buffer};
    use crate::platform::wayland::managers::compositor_manager::create_surface;
    use crate::platform::wayland::managers::surface_manager::{commit, wait_for_configure, damage_buffer};
    use crate::platform::wayland::managers::xdg_manager::{get_xdg_surface, get_toplevel, ack_configure, attach};
    use crate::platform::wayland::models::object_id_allocator::ObjectIdAllocator;

    pub fn window_create(width: i32, height: i32) ->Result<Window, String>{
        let mut allocator = ObjectIdAllocator::new();
        let mut connection = connect()?;
        let registry = get_registry(&mut connection)?;
        let shm_name = find_global(&registry, "wl_shm").ok_or("wl_shm not found".to_string())?;
        let compositor_name = find_global(&registry, "wl_compositor").ok_or("wl_compositor not found".to_string())?;
        let xdg_wm_base_name = find_global(&registry, "xdg_wm_base").ok_or("xdg_wm_base not found".to_string())?;
        bind_global(&mut connection, shm_name, "wl_shm", 2, 4)?;
        let memory = create_shared_memory((width * height * 4) as usize)?;
        let ptr = map_memory(&memory)?;
        let pixels = unsafe { std::slice::from_raw_parts_mut(ptr, memory.size()) };
        //pixels.fill(0xFF);

        let pool_id = create_shm_pool(&mut connection, memory.fd(), memory.size())?;
        let buffer_id = create_buffer(&mut connection, pool_id, width, height)?;
        let compositor_id = bind_global(&mut connection, compositor_name, "wl_compositor", 4, 7)?;
        let surface_id = create_surface(&mut connection, compositor_id)?;
        let xdg_wm_base_id = bind_global(&mut connection, xdg_wm_base_name, "xdg_wm_base", 1, 9)?;
        let xdg_surface_id = get_xdg_surface(&mut connection, xdg_wm_base_id, surface_id)?;
        let toplevel_id = get_toplevel(&mut connection, xdg_surface_id)?;
        commit(&mut connection, surface_id)?;
        let serial = wait_for_configure(&mut connection, xdg_surface_id)?;
        ack_configure(&mut connection, xdg_surface_id, serial)?;
        damage_buffer(&mut connection, surface_id, 0, 0, width, height)?;
        attach(&mut connection, surface_id, buffer_id)?;
        commit(&mut connection, surface_id)?;

        let window = Window::new(surface_id, buffer_id, pool_id, toplevel_id, xdg_wm_base_id, xdg_surface_id, width, height, ptr, connection);
        Ok((window))
    }
