use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct FmtConfig {
    tab_size: u8,
    column_limit: u8,
    declaration_spacing: u8
}

fn main() {
    println!("Hello, world!");
}
