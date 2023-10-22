use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn launch() -> [u8;0xFFFF] {
// Open the file for reading

    let path = Path::new("snake.gb");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    
    let mut rom: Vec<u8> = Vec::new();
    //buffer = buffer[100..].to_vec();
    for (_index,byte) in buffer.iter().enumerate() {
        let hex_value = format!("{:04X}", byte);
        if hex_value.len() == 2 {
            rom.push(0x00); // Add a leading zero for 1-byte values
        }
        rom.extend_from_slice(&hex_value.as_bytes());
    }
    rom.drain(0..100);
//in case you cant to check the first value of the rom
   /* if let Some(first_value) = rom.get(0) {
        let hex_value = format!("{:02X}", first_value);
        println!("The first value in the Vec<u8> as hexadecimal is: 0x{}", hex_value);
    } else {
        println!("The Vec<u8> is empty.");
    }
*/  rom.try_into()
    .unwrap_or_else(|rom: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 0xFFFF, rom.len()))

}
