use std::array::from_mut;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

struct RomHeader{
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: [u8;16],
    pub new_license_code: u16,
    pub sgb_flag: u8,
    pub rom_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub license_code: u8,
    pub version: u8,
    pub checksum: u8,
    pub global_checksum: u16
}

struct Cartridge{
    pub filename: String,
    pub rom_size: u64,
    pub rom_data: Vec<u8>,
    pub rom_header: RomHeader
}

static HEADER:RomHeader = RomHeader{
    entry:[0u8;4], 
    logo:[0u8; 0x30], 
    title: [0u8;16],
    new_license_code:0u16 ,
    sgb_flag:0u8 ,
    rom_type:0u8 ,
    rom_size: 0u8,
    ram_size:0u8 ,
    dest_code:0u8 ,
    license_code:0u8 ,
    version:0u8,
    checksum: 0u8,
    global_checksum:0u16
};

static CARTRIDGE:Cartridge = Cartridge{
    filename:"".to_string(),
    rom_data:Vec::new(),
    rom_size:0,
    rom_header:HEADER
};

static ROM_TYPES: Vec<&str> = vec![
    "ROM ONLY",
    "MBC1",
    "MBC1+RAM",
    "MBC1+RAM+BATTERY",
    "MBC2",
    "MBC2+BATTERY",
    "ROM+RAM",
    "ROM+RAM+BATTERY",
    "MMM01",
    "MMM01+RAM",
    "MMM01+RAM+BATTERY",
    "MBC3+TIMER+BATTERY",
    "MBC3+TIMER+RAM+BATTERY",
    "MBC3",
    "MBC3+RAM",
    "MBC3+RAM+BATTERY",
    "MBC5",
    "MBC5+RAM",
    "MBC5+RAM+BATTERY",
    "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM",
    "MBC5+RUMBLE+RAM+BATTERY",
    "MBC6",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
    "POCKET CAMERA",
    "BANDAI TAMA5",
    "HuC3",
    "HuC1+RAM+BATTERY"
];

static LICENSE_CODE:Vec<&str> = vec![
    "None",
    "Nintendo R&D1",
    "Capcom",
    "Electronic Arts",
    "Hudson Soft",
    "b-ai",
    "kss",
    "pow",
    "PCM Complete",
    "san-x",
    "Kemco Japan",
    "seta",
    "Viacom",
    "Nintendo",
    "Bandai",
    "Ocean/Acclaim",
    "Konami",
    "Hector",
    "Taito",
    "Hudson",
    "Banpresto",
    "UbiSoft",
    "Atlus",
    "Malibu",
    "angel",
    "Bullet-Proof",
    "irem",
    "Absolute",
    "Acclaim",
    "Activision",
    "American sammy",
    "Konami",
    "Hi tech entertainment",
    "LJN",
    "Matchbox",
    "Mattel",
    "Milton Bradley",
    "Titus",
    "Virgin",
    "LucasArts",
    "Ocean",
    "Electronic Arts",
    "Infogrames",
    "Interplay",
    "Broderbund",
    "sculptured",
    "sci",
    "THQ",
    "Accolade",
    "misawa",
    "lozc",
    "Tokuma Shoten Intermedia",
    "Tsukuda Original",
    "Chunsoft",
    "Video system",
    "Ocean/Acclaim",
    "Varie",
    "Yonezawa/s’pal",
    "Kaneko",
    "Pack in soft",
    "Bottom Up",
    "Konami (Yu-Gi-Oh!)"
];

fn rom_type_name(cart: &Cartridge)->&'static str{
    let rom_type = cart.rom_header.rom_type;
    if  rom_type <= 0x22 {
        return ROM_TYPES[usize::from(rom_type)];
    }
    return "UNKNOWN";
}

fn rom_license_name(cart: &Cartridge)->&'static str{
    if  cart.rom_header.new_license_code <= 0xA4 {
        return LICENSE_CODE[usize::from(cart.rom_header.license_code)];
    }
    return "UNKNOWN";
}

pub fn load_rom(cart: String)-> std::io::Result<()>{
    let path = Path::new(&cart);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let size = file.metadata()?.len();
    file.rewind()?;

    file.seek(SeekFrom::Start(101))?;
    file.read_exact(&mut HEADER.entry);
    file.read_exact(&mut HEADER.logo);
    file.read_exact(&mut HEADER.title);
    let mut new_lic_code = [0u8;2];
    file.read_exact(&mut new_lic_code);
    HEADER.new_license_code =((new_lic_code[0] as u16) << 8) | new_lic_code[1] as u16;
    file.read_exact(from_mut(&mut HEADER.sgb_flag));
    file.read_exact(from_mut(&mut HEADER.rom_type));
    file.read_exact(from_mut(&mut HEADER.rom_size));
    file.read_exact(from_mut(&mut HEADER.ram_size));
    file.read_exact(from_mut(&mut HEADER.dest_code));
    file.read_exact(from_mut(&mut HEADER.license_code));
    file.read_exact(from_mut(&mut HEADER.version));
    file.read_exact(from_mut(&mut HEADER.checksum));
    let mut global_check = [0u8;2];
    file.read_exact(&mut global_check);
    HEADER.global_checksum =((global_check[0] as u16) << 8) | global_check[1] as u16;

    
    CARTRIDGE.filename = cart;
    CARTRIDGE.rom_size= size;
    CARTRIDGE.rom_data= data;
   
    let mut check:u8 = 0;
    for address in 0x0134..0x014c + 1{
        check = check - CARTRIDGE.rom_data[address] - 1;
    }
    Ok(())
}

pub fn rom_read( address:u16)->u8{
    return CARTRIDGE.rom_data[usize::from(address)];
}

pub fn rom_write( address:u16,val:u8){

}