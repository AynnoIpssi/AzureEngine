use crate::platform::wayland::models::connection::WaylandConnection;

pub fn bind_global(connection: &mut WaylandConnection, name: u32, interface: &str, version: u32, new_id: u32) -> Result<u32, String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&2u32.to_le_bytes());
    let mut arg = Vec::new();
    arg.extend_from_slice(&name.to_le_bytes());
    arg.extend_from_slice(&((interface.len() as u32) + 1).to_le_bytes());
    arg.extend_from_slice(interface.as_bytes());
    arg.push(0u8);
    let data_len = interface.len() + 1;
    let padded_len = (data_len + 3) & !3;
    let padding = padded_len - data_len;
    for _ in 0..padding {
        arg.push(0u8);
    }
    arg.extend_from_slice(&version.to_le_bytes());
    arg.extend_from_slice(&new_id.to_le_bytes());
    let total_size = 8 + arg.len();
    msg.extend_from_slice(&((total_size as u32) << 16 | 0u32).to_le_bytes());
    msg.extend_from_slice(&arg);
    connection.send(&msg)?;
    Ok(new_id)
}