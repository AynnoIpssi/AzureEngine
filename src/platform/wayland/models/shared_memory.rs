pub struct WaylandMemory{
    fd: i32,
    size: usize,
}

impl WaylandMemory{
    pub fn fd(&self) -> i32{self.fd}
    pub fn size(&self) -> usize{self.size}

    pub fn new(fd: i32, size: usize) -> WaylandMemory{
        WaylandMemory{
            fd,
            size
        }
    }
}
