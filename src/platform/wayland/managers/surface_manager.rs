use crate::platform::wayland::managers::xdg_manager::ack_configure;
use crate::platform::wayland::models::connection::WaylandConnection;
use crate::platform::wayland::models::window::Window;
use crate::rendering::models::canvas::Canvas;
use crate::platform::wayland::managers::xdg_manager::attach;

pub fn damage_buffer(connection: &mut WaylandConnection, surface_id: u32, x: i32, y: i32, width: i32, height: i32) -> Result<(), String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&surface_id.to_le_bytes());
    msg.extend_from_slice(&((24u32 << 16 | 9u32).to_le_bytes()));
    msg.extend_from_slice(&x.to_le_bytes());
    msg.extend_from_slice(&y.to_le_bytes());
    msg.extend_from_slice(&width.to_le_bytes());
    msg.extend_from_slice(&height.to_le_bytes());
    connection.send(&msg)?;
    Ok(())
}

pub fn commit(connection: &mut WaylandConnection, surface_id: u32) -> Result<(), String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&(surface_id).to_le_bytes());
    msg.extend_from_slice(&((8u32 << 16 | 6u32).to_le_bytes()));
    connection.send(&msg)?;
    Ok(())
}

pub fn wait_for_configure(connection: &mut WaylandConnection, xdg_surface_id: u32) -> Result<u32, String> {
    loop {
        let mut header = [0u8; 8];
        connection.receive(&mut header)?;
        let object_id = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let size_opcode = u32::from_le_bytes(header[4..8].try_into().unwrap());
        let size = (size_opcode >> 16) as u16;
        let opcode = (size_opcode & 0xFFFF) as u16;

        let args_size = (size - 8) as usize;
        let mut args = vec![0u8; args_size];
        connection.receive(&mut args)?;

        if object_id == 1 && opcode == 0 {
            let error_object_id = u32::from_le_bytes(args[0..4].try_into().unwrap());
            let error_code = u32::from_le_bytes(args[4..8].try_into().unwrap());
            let msg_len = u32::from_le_bytes(args[8..12].try_into().unwrap()) as usize;
            let message = String::from_utf8_lossy(&args[12..12 + msg_len - 1]).to_string();
            return Err(format!("Wayland error on object {}: code {} - {}", error_object_id, error_code, message));
        }

        if object_id == xdg_surface_id && opcode == 0 {
            let serial = u32::from_le_bytes(args[0..4].try_into().unwrap());
            return Ok(serial);
        }
    }
}

pub fn run_event_loop(connection: &mut Window, xdg_surface_id: u32, xdg_toplevel_id: u32, surface_id: u32, xdg_wm_base_id: u32) -> Result<(), String> {
    loop {
        let mut header = [0u8; 8];
        connection.connection_mut().receive(&mut header)?;
        let object_id = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let size_opcode = u32::from_le_bytes(header[4..8].try_into().unwrap());
        let size = (size_opcode >> 16) as u16;
        let opcode = (size_opcode & 0xFFFF) as u16;
        let args_size = (size - 8) as usize;
        let mut args = vec![0u8; args_size];
        connection.connection_mut().receive(&mut args)?;
        if object_id == xdg_toplevel_id && opcode == 0 {
            // ignore
        }
        if object_id == xdg_toplevel_id && opcode == 1 {
            return Ok(());
        }
        if object_id == xdg_surface_id && opcode == 0 {
            let serial = u32::from_le_bytes(args[0..4].try_into().unwrap());
            ack_configure(connection.connection_mut(), xdg_surface_id, serial)?;
            let buffer_id = connection.buffer_id();
            attach(connection.connection_mut(), surface_id, buffer_id)?;
        }
        if object_id == xdg_wm_base_id && opcode == 0 {
            let serial = u32::from_le_bytes(args[0..4].try_into().unwrap());
            let mut msg = Vec::new();
            msg.extend_from_slice(&(xdg_wm_base_id).to_le_bytes());
            msg.extend_from_slice(&((12u32 << 16 | 3u32).to_le_bytes()));
            msg.extend_from_slice(&serial.to_le_bytes());
            connection.connection_mut().send(&msg)?;
        }
        if object_id == 1 && opcode == 0 {
            let error_object_id = u32::from_le_bytes(args[0..4].try_into().unwrap());
            let error_code = u32::from_le_bytes(args[4..8].try_into().unwrap());
            let msg_len = u32::from_le_bytes(args[8..12].try_into().unwrap()) as usize;
            let message = String::from_utf8_lossy(&args[12..12 + msg_len - 1]).to_string();
            return Err(format!("Wayland error on object {}: code {} - {}", error_object_id, error_code, message));
        }
    }
}


