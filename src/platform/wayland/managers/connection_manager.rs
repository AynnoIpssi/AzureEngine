use crate::platform::wayland::models::connection::WaylandConnection;
use std::os::unix::net::UnixStream;

pub fn connect() -> Result<WaylandConnection, String> {


    //Find environement variable.
    let display = std::env::var("WAYLAND_DISPLAY")
        .map_err(|_| "WAYLAND_DISPLAY not set".to_string())?;

    //Find Location environement variable.
    let runtime = std::env::var("XDG_RUNTIME_DIR")
    .map_err(|_| "XDG_RUNTIME_DIR not set".to_string())?;

    //Create Path
    let path = format!("{}/{}", runtime, display);

    //oppen connection with UnixStream
    let stream = UnixStream::connect(&path)
        .map_err(|e| format!("Failed to connect to Wayland socket: {}", e))?;

    //Return Connection.
    Ok(WaylandConnection::new(stream))
}