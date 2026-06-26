use azure_core::rules::window_provider::AzureWindowProvider;
use azure_core::rules::window_event::WindowEvent;
use crate::platform::wayland::models::connection::WaylandConnection;
pub struct Window{
    surface_id: u32,
    buffer_id: u32,
    pool_id: u32,
    xdg_toplevel_id: u32,
    xdg_wm_id: u32,
    xdg_surface_id: u32,
    width: i32,
    height: i32,
    ptr: *mut u8,
    connection: WaylandConnection,
}

impl Window {
    pub fn surface_id(&self) -> u32 { self.surface_id }
    pub fn buffer_id(&self) -> u32 { self.buffer_id }
    pub fn xdg_toplevel_id(&self) -> u32 { self.xdg_toplevel_id }
    pub fn xdg_wm_id(&self) -> u32 { self.xdg_wm_id }
    pub fn xdg_surface_id(&self) -> u32 { self.xdg_surface_id }
    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn ptr(&self) -> *mut u8 { self.ptr }

    pub fn connection_mut(&mut self) -> &mut WaylandConnection {
        &mut self.connection
    }

    pub fn new(surface_id: u32, buffer_id: u32, pool_id: u32, xdg_toplevel_id: u32, xdg_wm_id: u32, xdg_surface_id: u32, width: i32, height: i32, ptr: *mut u8, connection: WaylandConnection) -> Window{
        Window{
            surface_id,
            buffer_id,
            pool_id,
            xdg_toplevel_id,
            xdg_wm_id,
            xdg_surface_id,
            width,
            height,
            ptr,
            connection,
        }
    }
}

impl AzureWindowProvider for Window {

    fn width(&self) -> i32 {self.width}
    fn height(&self) -> i32 {self.height}
    fn render(&mut self, pixels: &[u8]) -> Result<(), String> {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.ptr, (self.width * self.height * 4) as usize)
        };
        buffer.copy_from_slice(pixels);
        Ok(())
    }
    fn poll_event(&mut self) -> Option<WindowEvent> {
        let mut header = [0u8; 8];
        self.connection.set_nonblocking(true);
        match self.connection.receive(&mut header) {
            Err(_) => {
                self.connection.set_nonblocking(false);
                return None;
            }
            Ok(_) => {}
        }

        let object_id = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let size_opcode = u32::from_le_bytes(header[4..8].try_into().unwrap());
        let size = (size_opcode >> 16) as u16;
        let opcode = (size_opcode & 0xFFFF) as u16;
        let args_size = (size - 8) as usize;
        let mut args = vec![0u8; args_size];
        self.connection.receive(&mut args).ok();

        self.connection.set_nonblocking(false).ok();

        let toplevel_id = self.xdg_toplevel_id;
        let xdg_surface_id = self.xdg_surface_id;

        if object_id == toplevel_id && opcode == 1 {
            return Some(WindowEvent::WindowClose);
        }

        if object_id == toplevel_id && opcode == 0 {
            let new_width = i32::from_le_bytes(args[0..4].try_into().unwrap_or([0;4]));
            let new_height = i32::from_le_bytes(args[4..8].try_into().unwrap_or([0;4]));
            return Some(WindowEvent::WindowResize(new_width, new_height));
        }
        None
    }
}