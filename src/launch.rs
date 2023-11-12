use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn launch(file_path: &str, max_size_kb: usize) -> Vec<u8> {
    // Construct the full file path
    let path = Path::new(file_path);
    let display = path.display();
    
    // Open the file for reading
    let mut file = File::open(path).unwrap_or_else(|_| panic!("Couldn't open {}: {}", display, "Couldn't open the file"));
    
    // Read the file content into a Vec<u8>
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|_| panic!("Couldn't read {}: {}", display, "Couldn't read the file"));

    // Check if the ROM size exceeds the maximum allowed size
    let rom_size = buffer.len();
    if rom_size > max_size_kb * 1024 {
        panic!(
            "ROM size exceeds the maximum allowed size ({} KB)",
            max_size_kb
        );
    }

    // Return the ROM data as a Vec<u8>
    buffer
}





