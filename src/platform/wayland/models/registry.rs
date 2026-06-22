pub struct WaylandGlobal {
    name: String,
    version: u32,
    id: u32,
}

impl WaylandGlobal {

    pub fn name(&self) -> &str { &self.name }
    pub fn version(&self) -> u32 { self.version }
    pub fn id(&self) -> u32 { self.id }

    pub fn new(name: &str, version: u32, id: u32) -> WaylandGlobal {
        WaylandGlobal {
            name: name.to_string(),
            version,
            id,
        }
    }
}

pub struct WaylandRegistry {
    globals: Vec<WaylandGlobal>,
}

impl WaylandRegistry {
    pub fn new() -> WaylandRegistry {
        WaylandRegistry{
            globals: Vec::new(),
        }
    }

    pub fn add_global(&mut self, global: WaylandGlobal) {
        self.globals.push(global);
    }

    pub fn get_globals(&self) -> &[WaylandGlobal] {
        &self.globals
    }
}