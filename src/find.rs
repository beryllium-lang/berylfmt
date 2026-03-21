pub fn find_config() -> Option<String> {
    let mut curr_path = std::env::current_dir().ok()?;
    
    loop {
        let try_config = curr_path.join(".berylfmt");
        
        if try_config.exists() {
            if try_config.is_file() {
                return try_config.to_str().map(|s| s.to_string());
            } else {
                return None;
            }
        }
        
        match curr_path.parent() {
            Some(parent) if parent != curr_path => {
                curr_path = parent.to_path_buf();
            }
            _ => break,
        }
    }
    
    None
}
