use crate::platform::wayland::models::connection::WaylandConnection;
pub fn get_xdg_surface(connection: &mut WaylandConnection, xdg_wm_base_id: u32, surface_id: u32) -> Result<u32, String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&xdg_wm_base_id.to_le_bytes());
    msg.extend_from_slice(&((16u32 << 16 | 2u32).to_le_bytes()));
    msg.extend_from_slice(&10u32.to_le_bytes());
    msg.extend_from_slice(&surface_id.to_le_bytes());
    connection.send(&msg)?;
    Ok(10)
}

pub fn get_toplevel(connection: &mut WaylandConnection, xdg_surface_id: u32) -> Result<u32, String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&xdg_surface_id.to_le_bytes());
    msg.extend_from_slice(&((12u32 << 16 | 1u32).to_le_bytes()));
    msg.extend_from_slice(&11u32.to_le_bytes());
    connection.send(&msg)?;
    Ok(11)
}

pub fn ack_configure(connection: &mut WaylandConnection, xdg_surface_id: u32, serial: u32) -> Result<(), String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&xdg_surface_id.to_le_bytes());
    msg.extend_from_slice(&((12u32 << 16 | 4u32).to_le_bytes()));
    msg.extend_from_slice(&serial.to_le_bytes());
    connection.send(&msg)?;
    Ok(())
}

pub fn attach(connection: &mut WaylandConnection, surface_id: u32, buffer_id: u32) -> Result<(), String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&surface_id.to_le_bytes());
    msg.extend_from_slice(&((20u32 << 16 | 1u32).to_le_bytes()));
    msg.extend_from_slice(&buffer_id.to_le_bytes());
    msg.extend_from_slice(&0u32.to_le_bytes()); //X
    msg.extend_from_slice(&0u32.to_le_bytes()); //Y
    connection.send(&msg)?;
    Ok(())
}