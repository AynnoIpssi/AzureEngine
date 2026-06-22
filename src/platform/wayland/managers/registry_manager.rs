use crate::platform::wayland::models::connection::WaylandConnection;
use crate::platform::wayland::models::registry::WaylandRegistry;
use crate::platform::wayland::models::registry::WaylandGlobal;

pub fn find_global(registry: &WaylandRegistry, interface: &str) -> Option<u32> {
    for global in registry.get_globals() {
        if global.name() == interface {
            return Some(global.id());
        }
    }
    None
}
pub fn get_registry(connection: &mut WaylandConnection) -> Result<WaylandRegistry, String> {
    let mut msg = Vec::new();
    msg.extend_from_slice(&1u32.to_le_bytes());         // object_id
    msg.extend_from_slice(&((12u32 << 16) | 1u32).to_le_bytes()); // taille+opcode
    msg.extend_from_slice(&2u32.to_le_bytes());         // id du registry
    connection.send(&msg)?;

    let mut wl_display = Vec::new();
    wl_display.extend_from_slice(&1u32.to_le_bytes()); //object_id
    wl_display.extend_from_slice(&((12u32 << 16) | 0u32).to_le_bytes()); //tail + opcode
    wl_display.extend_from_slice(&3u32.to_le_bytes()); //id display
    connection.send(&wl_display)?;

    let mut registry = WaylandRegistry::new();

    loop{
        let mut header = [0u8; 8];
        connection.receive(&mut header)?;

        let object_id = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let size_opcode = u32::from_le_bytes(header[4..8].try_into().unwrap());
        let size = (size_opcode >> 16) as u16;
        let opcode = (size_opcode & 0xFFFF) as u16;

        let args_size = (size - 8) as usize;
        let mut args = vec![0u8; args_size];
        connection.receive(&mut args)?;

        match (object_id, opcode) {
            (2, 0) => {
                let global_id = u32::from_le_bytes(args[0..4].try_into().unwrap());
                let name_len = u32::from_le_bytes(args[4..8].try_into().unwrap()) as usize;
                let name = String::from_utf8_lossy(&args[8..8 + name_len - 1]).to_string();
                let name_padded = (name_len + 3) & !3;
                let version_offset = 8 + name_padded;
                let version = u32::from_le_bytes(args[version_offset..version_offset + 4].try_into().unwrap());
                let global = WaylandGlobal::new(&name, version, global_id);
                registry.add_global(global);
            }
            (3, 0) => break,
            _ => {}
        }
    }


    Ok(registry)
}

