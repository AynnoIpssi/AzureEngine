use crate::platform::wayland::models::connection::WaylandConnection;
use std::os::fd::RawFd;

pub fn create_shm_pool(connection: &mut WaylandConnection, fd: RawFd, size: usize) -> Result<u32, String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&4u32.to_le_bytes());
    msg.extend_from_slice(&((16u32 << 16 | 0u32).to_le_bytes()));
    msg.extend_from_slice(&5u32.to_le_bytes());
    msg.extend_from_slice(&(size as i32).to_le_bytes());
    connection.send_with_fd(&msg, fd)?;
    Ok(5)
}

pub fn create_buffer(connection: &mut WaylandConnection, pool_id: u32, width: i32, height: i32) -> Result<u32, String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&pool_id.to_le_bytes());
    msg.extend_from_slice(&((32u32 << 16 | 0u32).to_le_bytes()));
    msg.extend_from_slice(&6u32.to_le_bytes());
    msg.extend_from_slice(&0u32.to_le_bytes());
    msg.extend_from_slice(&width.to_le_bytes());
    msg.extend_from_slice(&height.to_le_bytes());
    msg.extend_from_slice(&(width * 4).to_le_bytes());
    msg.extend_from_slice(&0u32.to_le_bytes());
    connection.send(&msg)?;
    Ok(6)
}


