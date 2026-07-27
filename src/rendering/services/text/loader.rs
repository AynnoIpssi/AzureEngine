pub fn load_font(path: &str) -> Result<Vec<u8>, String> {
    return std::fs::read(path).map_err(|e| e.to_string());
}