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