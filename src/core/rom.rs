use std::fs;

fn read_rom(filename: &str) {
    let rom_data = fs::read(filename).expect("Unable to read file contents");
    
}