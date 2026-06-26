use crate::platform::wayland::models::shared_memory::WaylandMemory;


pub fn create_shared_memory(size: usize) -> Result<WaylandMemory, String> {
    let fd = unsafe {
        libc::memfd_create(c"azure-shm".as_ptr(), 0)
    };
    if fd == -1 {
        return Err("Failed to create shared memory".to_string());
    }
    let result = unsafe {
        libc::ftruncate(fd, size as i64)
    };

    if result == -1 {
        return Err("Failed to resize shared memory".to_string());
    }
    Ok(WaylandMemory::new(fd, size))
}

pub fn map_memory(memory: &WaylandMemory) -> Result<*mut u8, String> {
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

    if (ptr == libc::MAP_FAILED) {
        return Err("Failed to map memory".to_string());
    }

    Ok(ptr as *mut u8)
}

pub fn unmap_memory(ptr: *mut u8, size: usize) -> Result<(), String> {
    let result = unsafe {
        libc::munmap(ptr as *mut libc::c_void, size)
    };

    if (result == -1){
        return Err("Failed to unmap memory".to_string());
    }
    Ok(())
}