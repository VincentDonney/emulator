pub enum Instruction {
    // ALU Instructions
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

    // 16-Bit ALU Instructions
    ADDHL(ArithmeticTarget),

    // INC and DEC Instructions
    INC(IncDecTarget),
    DEC(IncDecTarget),

    // Rotate and Shift Instructions
    RLC(PrefixTarget),
    RRC(PrefixTarget),
    RL(PrefixTarget),
    RR(PrefixTarget),
    SLA(PrefixTarget),
    SRA(PrefixTarget),
    SWAP(PrefixTarget),
    SRL(PrefixTarget),

    // Jump Instructions
    JP(JumpTest, JumpTarget),
    JR(JumpTest),
    CALL(JumpTest),
    RET(JumpTest),

    // Load Instructions
    LD(LoadType),
    LDH(LoadType),

    // Stack Instructions
    PUSH(StackTarget),
    POP(StackTarget),

    // Miscellaneous Instructions
    NOP(),
    BIT(u8, PrefixTarget),
    RES(u8, PrefixTarget),
    SET(u8, PrefixTarget),
    RLCA(),
    RRCA(),
    RLA(),
    RRA(),
    DAA(),
    CPL(),
    SCF(),
    CCF(),
    RETI(),

    // Halt and Stop Instructions
    HALT(),
    STOP(),

    // Interrupt Instructions
    DI(),
    EI(),

    // Prefix CB Instruction
    PREFIX(),

    // Restart (RST) Instructions
    RST(RestartTarget),
}
  
pub enum RestartTarget {
    H00, H08, H10, H18, H20, H28, H30, H38 
}
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, D8, DE, BC, SP
}

pub enum JumpTarget{
    A16, HL
}
pub enum IncDecTarget{
    A,B,C,D,E,H,L, BC, DE, SP, HL
}

pub enum PrefixTarget {
    A,B,C,D,E,H,L,HL
}

pub enum StackTarget{
    BC, DE, HL, AF
}

pub enum LoadByteTarget{
    A, B, C, D, E, H, L, HLI, HLD, BC, A16, DE, HL, SP, FF00C, A8
}

pub enum LoadByteSource{
    A, B, C, D, E, H, L, D8, HLI, HLD, BC, SP, DE, D16, HL, FF00C, A8, A16
}

pub enum LoadType{
    Byte(LoadByteTarget,LoadByteSource)
}

pub enum JumpTest{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

impl Instruction{
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
          Instruction::from_byte_prefixed(byte)
        } else {
          Instruction::from_byte_not_prefixed(byte)
        }
    }
    
    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x06 => Some(Instruction::RLC(PrefixTarget::HL)),
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),

            0x08 => Some(Instruction::RRC(PrefixTarget::B)),
            0x09 => Some(Instruction::RRC(PrefixTarget::C)),
            0x0A => Some(Instruction::RRC(PrefixTarget::D)),
            0x0B => Some(Instruction::RRC(PrefixTarget::E)),
            0x0C => Some(Instruction::RRC(PrefixTarget::H)),
            0x0D => Some(Instruction::RRC(PrefixTarget::L)),
            0x0E => Some(Instruction::RRC(PrefixTarget::HL)),
            0x0F => Some(Instruction::RRC(PrefixTarget::A)),

            0x10 => Some(Instruction::RL(PrefixTarget::B)),
            0x11 => Some(Instruction::RL(PrefixTarget::C)),
            0x12 => Some(Instruction::RL(PrefixTarget::D)),
            0x13 => Some(Instruction::RL(PrefixTarget::E)),
            0x14 => Some(Instruction::RL(PrefixTarget::H)),
            0x15 => Some(Instruction::RL(PrefixTarget::L)),
            0x16 => Some(Instruction::RL(PrefixTarget::HL)),
            0x17 => Some(Instruction::RL(PrefixTarget::A)),

            0x18 => Some(Instruction::RR(PrefixTarget::B)),
            0x19 => Some(Instruction::RR(PrefixTarget::C)),
            0x1A => Some(Instruction::RR(PrefixTarget::D)),
            0x1B => Some(Instruction::RR(PrefixTarget::E)),
            0x1C => Some(Instruction::RR(PrefixTarget::H)),
            0x1D => Some(Instruction::RR(PrefixTarget::L)),
            0x1E => Some(Instruction::RR(PrefixTarget::HL)),
            0x1F => Some(Instruction::RR(PrefixTarget::A)),

            0x20 => Some(Instruction::SLA(PrefixTarget::B)),
            0x21 => Some(Instruction::SLA(PrefixTarget::C)),
            0x22 => Some(Instruction::SLA(PrefixTarget::D)),
            0x23 => Some(Instruction::SLA(PrefixTarget::E)),
            0x24 => Some(Instruction::SLA(PrefixTarget::H)),
            0x25 => Some(Instruction::SLA(PrefixTarget::L)),
            0x26 => Some(Instruction::SLA(PrefixTarget::HL)),
            0x27 => Some(Instruction::SLA(PrefixTarget::A)),

            0x28 => Some(Instruction::SRA(PrefixTarget::B)),
            0x29 => Some(Instruction::SRA(PrefixTarget::C)),
            0x2A => Some(Instruction::SRA(PrefixTarget::D)),
            0x2B => Some(Instruction::SRA(PrefixTarget::E)),
            0x2C => Some(Instruction::SRA(PrefixTarget::H)),
            0x2D => Some(Instruction::SRA(PrefixTarget::L)),
            0x2E => Some(Instruction::SRA(PrefixTarget::HL)),
            0x2F => Some(Instruction::SRA(PrefixTarget::A)),

            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x36 => Some(Instruction::SWAP(PrefixTarget::HL)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),

            0x38 => Some(Instruction::SRL(PrefixTarget::B)),
            0x39 => Some(Instruction::SRL(PrefixTarget::C)),
            0x3A => Some(Instruction::SRL(PrefixTarget::D)),
            0x3B => Some(Instruction::SRL(PrefixTarget::E)),
            0x3C => Some(Instruction::SRL(PrefixTarget::H)),
            0x3D => Some(Instruction::SRL(PrefixTarget::L)),
            0x3E => Some(Instruction::SRL(PrefixTarget::HL)),
            0x3F => Some(Instruction::SRL(PrefixTarget::A)),

            0x40 => Some(Instruction::BIT(0,PrefixTarget::B)),
            0x41 => Some(Instruction::BIT(0,PrefixTarget::C)),
            0x42 => Some(Instruction::BIT(0,PrefixTarget::D)),
            0x43 => Some(Instruction::BIT(0,PrefixTarget::E)),
            0x44 => Some(Instruction::BIT(0,PrefixTarget::H)),
            0x45 => Some(Instruction::BIT(0,PrefixTarget::L)),
            0x46 => Some(Instruction::BIT(0,PrefixTarget::HL)),
            0x47 => Some(Instruction::BIT(0,PrefixTarget::A)),

            0x48 => Some(Instruction::BIT(1,PrefixTarget::B)),
            0x49 => Some(Instruction::BIT(1,PrefixTarget::C)),
            0x4A => Some(Instruction::BIT(1,PrefixTarget::D)),
            0x4B => Some(Instruction::BIT(1,PrefixTarget::E)),
            0x4C => Some(Instruction::BIT(1,PrefixTarget::H)),
            0x4D => Some(Instruction::BIT(1,PrefixTarget::L)),
            0x4E => Some(Instruction::BIT(1,PrefixTarget::HL)),
            0x4F => Some(Instruction::BIT(1,PrefixTarget::A)),

            0x50 => Some(Instruction::BIT(2,PrefixTarget::B)),
            0x51 => Some(Instruction::BIT(2,PrefixTarget::C)),
            0x52 => Some(Instruction::BIT(2,PrefixTarget::D)),
            0x53 => Some(Instruction::BIT(2,PrefixTarget::E)),
            0x54 => Some(Instruction::BIT(2,PrefixTarget::H)),
            0x55 => Some(Instruction::BIT(2,PrefixTarget::L)),
            0x56 => Some(Instruction::BIT(2,PrefixTarget::HL)),
            0x57 => Some(Instruction::BIT(2,PrefixTarget::A)),

            0x58 => Some(Instruction::BIT(3,PrefixTarget::B)),
            0x59 => Some(Instruction::BIT(3,PrefixTarget::C)),
            0x5A => Some(Instruction::BIT(3,PrefixTarget::D)),
            0x5B => Some(Instruction::BIT(3,PrefixTarget::E)),
            0x5C => Some(Instruction::BIT(3,PrefixTarget::H)),
            0x5D => Some(Instruction::BIT(3,PrefixTarget::L)),
            0x5E => Some(Instruction::BIT(3,PrefixTarget::HL)),
            0x5F => Some(Instruction::BIT(3,PrefixTarget::A)),

            0x60 => Some(Instruction::BIT(4,PrefixTarget::B)),
            0x61 => Some(Instruction::BIT(4,PrefixTarget::C)),
            0x62 => Some(Instruction::BIT(4,PrefixTarget::D)),
            0x63 => Some(Instruction::BIT(4,PrefixTarget::E)),
            0x64 => Some(Instruction::BIT(4,PrefixTarget::H)),
            0x65 => Some(Instruction::BIT(4,PrefixTarget::L)),
            0x66 => Some(Instruction::BIT(4,PrefixTarget::HL)),
            0x67 => Some(Instruction::BIT(4,PrefixTarget::A)),

            0x68 => Some(Instruction::BIT(5,PrefixTarget::B)),
            0x69 => Some(Instruction::BIT(5,PrefixTarget::C)),
            0x6A => Some(Instruction::BIT(5,PrefixTarget::D)),
            0x6B => Some(Instruction::BIT(5,PrefixTarget::E)),
            0x6C => Some(Instruction::BIT(5,PrefixTarget::H)),
            0x6D => Some(Instruction::BIT(5,PrefixTarget::L)),
            0x6E => Some(Instruction::BIT(5,PrefixTarget::HL)),
            0x6F => Some(Instruction::BIT(5,PrefixTarget::A)),

            0x70 => Some(Instruction::BIT(6,PrefixTarget::B)),
            0x71 => Some(Instruction::BIT(6,PrefixTarget::C)),
            0x72 => Some(Instruction::BIT(6,PrefixTarget::D)),
            0x73 => Some(Instruction::BIT(6,PrefixTarget::E)),
            0x74 => Some(Instruction::BIT(6,PrefixTarget::H)),
            0x75 => Some(Instruction::BIT(6,PrefixTarget::L)),
            0x76 => Some(Instruction::BIT(6,PrefixTarget::HL)),
            0x77 => Some(Instruction::BIT(6,PrefixTarget::A)),

            0x78 => Some(Instruction::BIT(7,PrefixTarget::B)),
            0x79 => Some(Instruction::BIT(7,PrefixTarget::C)),
            0x7A => Some(Instruction::BIT(7,PrefixTarget::D)),
            0x7B => Some(Instruction::BIT(7,PrefixTarget::E)),
            0x7C => Some(Instruction::BIT(7,PrefixTarget::H)),
            0x7D => Some(Instruction::BIT(7,PrefixTarget::L)),
            0x7E => Some(Instruction::BIT(7,PrefixTarget::HL)),
            0x7F => Some(Instruction::BIT(7,PrefixTarget::A)),

            0x81 => Some(Instruction::RES(0,PrefixTarget::B)),
            0x81 => Some(Instruction::RES(0,PrefixTarget::C)),
            0x82 => Some(Instruction::RES(0,PrefixTarget::D)),
            0x83 => Some(Instruction::RES(0,PrefixTarget::E)),
            0x84 => Some(Instruction::RES(0,PrefixTarget::H)),
            0x85 => Some(Instruction::RES(0,PrefixTarget::L)),
            0x86 => Some(Instruction::RES(0,PrefixTarget::HL)),
            0x87 => Some(Instruction::RES(0,PrefixTarget::A)),

            0x88 => Some(Instruction::RES(1,PrefixTarget::B)),
            0x89 => Some(Instruction::RES(1,PrefixTarget::C)),
            0x8A => Some(Instruction::RES(1,PrefixTarget::D)),
            0x8B => Some(Instruction::RES(1,PrefixTarget::E)),
            0x8C => Some(Instruction::RES(1,PrefixTarget::H)),
            0x8D => Some(Instruction::RES(1,PrefixTarget::L)),
            0x8E => Some(Instruction::RES(1,PrefixTarget::HL)),
            0x8F => Some(Instruction::RES(1,PrefixTarget::A)),

            0x90 => Some(Instruction::RES(2,PrefixTarget::B)),
            0x91 => Some(Instruction::RES(2,PrefixTarget::C)),
            0x92 => Some(Instruction::RES(2,PrefixTarget::D)),
            0x93 => Some(Instruction::RES(2,PrefixTarget::E)),
            0x94 => Some(Instruction::RES(2,PrefixTarget::H)),
            0x95 => Some(Instruction::RES(2,PrefixTarget::L)),
            0x96 => Some(Instruction::RES(2,PrefixTarget::HL)),
            0x97 => Some(Instruction::RES(2,PrefixTarget::A)),

            0x98 => Some(Instruction::RES(3,PrefixTarget::B)),
            0x99 => Some(Instruction::RES(3,PrefixTarget::C)),
            0x9A => Some(Instruction::RES(3,PrefixTarget::D)),
            0x9B => Some(Instruction::RES(3,PrefixTarget::E)),
            0x9C => Some(Instruction::RES(3,PrefixTarget::H)),
            0x9D => Some(Instruction::RES(3,PrefixTarget::L)),
            0x9E => Some(Instruction::RES(3,PrefixTarget::HL)),
            0x9F => Some(Instruction::RES(3,PrefixTarget::A)),

            0xA0 => Some(Instruction::RES(4,PrefixTarget::B)),
            0xA1 => Some(Instruction::RES(4,PrefixTarget::C)),
            0xA2 => Some(Instruction::RES(4,PrefixTarget::D)),
            0xA3 => Some(Instruction::RES(4,PrefixTarget::E)),
            0xA4 => Some(Instruction::RES(4,PrefixTarget::H)),
            0xA5 => Some(Instruction::RES(4,PrefixTarget::L)),
            0xA6 => Some(Instruction::RES(4,PrefixTarget::HL)),
            0xA7 => Some(Instruction::RES(4,PrefixTarget::A)),

            0xA8 => Some(Instruction::RES(5,PrefixTarget::B)),
            0xA9 => Some(Instruction::RES(5,PrefixTarget::C)),
            0xAA => Some(Instruction::RES(5,PrefixTarget::D)),
            0xAB => Some(Instruction::RES(5,PrefixTarget::E)),
            0xAC => Some(Instruction::RES(5,PrefixTarget::H)),
            0xAD => Some(Instruction::RES(5,PrefixTarget::L)),
            0xAE => Some(Instruction::RES(5,PrefixTarget::HL)),
            0xAF => Some(Instruction::RES(5,PrefixTarget::A)),

            0xB0 => Some(Instruction::RES(6,PrefixTarget::B)),
            0xB1 => Some(Instruction::RES(6,PrefixTarget::C)),
            0xB2 => Some(Instruction::RES(6,PrefixTarget::D)),
            0xB3 => Some(Instruction::RES(6,PrefixTarget::E)),
            0xB4 => Some(Instruction::RES(6,PrefixTarget::H)),
            0xB5 => Some(Instruction::RES(6,PrefixTarget::L)),
            0xB6 => Some(Instruction::RES(6,PrefixTarget::HL)),
            0xB7 => Some(Instruction::RES(6,PrefixTarget::A)),

            0xB8 => Some(Instruction::RES(7,PrefixTarget::B)),
            0xB9 => Some(Instruction::RES(7,PrefixTarget::C)),
            0xBA => Some(Instruction::RES(7,PrefixTarget::D)),
            0xBB => Some(Instruction::RES(7,PrefixTarget::E)),
            0xBC => Some(Instruction::RES(7,PrefixTarget::H)),
            0xBD => Some(Instruction::RES(7,PrefixTarget::L)),
            0xBE => Some(Instruction::RES(7,PrefixTarget::HL)),
            0xBF => Some(Instruction::RES(7,PrefixTarget::A)),

            0xC0 => Some(Instruction::SET(0,PrefixTarget::B)),
            0xC1 => Some(Instruction::SET(0,PrefixTarget::C)),
            0xC2 => Some(Instruction::SET(0,PrefixTarget::D)),
            0xC3 => Some(Instruction::SET(0,PrefixTarget::E)),
            0xC4 => Some(Instruction::SET(0,PrefixTarget::H)),
            0xC5 => Some(Instruction::SET(0,PrefixTarget::L)),
            0xC6 => Some(Instruction::SET(0,PrefixTarget::HL)),
            0xC7 => Some(Instruction::SET(0,PrefixTarget::A)),

            0xC8 => Some(Instruction::SET(1,PrefixTarget::B)),
            0xC9 => Some(Instruction::SET(1,PrefixTarget::C)),
            0xCA => Some(Instruction::SET(1,PrefixTarget::D)),
            0xCB => Some(Instruction::SET(1,PrefixTarget::E)),
            0xCC => Some(Instruction::SET(1,PrefixTarget::H)),
            0xCD => Some(Instruction::SET(1,PrefixTarget::L)),
            0xCE => Some(Instruction::SET(1,PrefixTarget::HL)),
            0xCF => Some(Instruction::SET(1,PrefixTarget::A)),

            0xD0 => Some(Instruction::SET(2,PrefixTarget::B)),
            0xD1 => Some(Instruction::SET(2,PrefixTarget::C)),
            0xD2 => Some(Instruction::SET(2,PrefixTarget::D)),
            0xD3 => Some(Instruction::SET(2,PrefixTarget::E)),
            0xD4 => Some(Instruction::SET(2,PrefixTarget::H)),
            0xD5 => Some(Instruction::SET(2,PrefixTarget::L)),
            0xD6 => Some(Instruction::SET(2,PrefixTarget::HL)),
            0xD7 => Some(Instruction::SET(2,PrefixTarget::A)),

            0xD8 => Some(Instruction::SET(3,PrefixTarget::B)),
            0xD9 => Some(Instruction::SET(3,PrefixTarget::C)),
            0xDA => Some(Instruction::SET(3,PrefixTarget::D)),
            0xDB => Some(Instruction::SET(3,PrefixTarget::E)),
            0xDC => Some(Instruction::SET(3,PrefixTarget::H)),
            0xDD => Some(Instruction::SET(3,PrefixTarget::L)),
            0xDE => Some(Instruction::SET(3,PrefixTarget::HL)),
            0xDF => Some(Instruction::SET(3,PrefixTarget::A)),

            0xE0 => Some(Instruction::SET(4,PrefixTarget::B)),
            0xE1 => Some(Instruction::SET(4,PrefixTarget::C)),
            0xE2 => Some(Instruction::SET(4,PrefixTarget::D)),
            0xE3 => Some(Instruction::SET(4,PrefixTarget::E)),
            0xE4 => Some(Instruction::SET(4,PrefixTarget::H)),
            0xE5 => Some(Instruction::SET(4,PrefixTarget::L)),
            0xE6 => Some(Instruction::SET(4,PrefixTarget::HL)),
            0xE7 => Some(Instruction::SET(4,PrefixTarget::A)),

            0xE8 => Some(Instruction::SET(5,PrefixTarget::B)),
            0xE9 => Some(Instruction::SET(5,PrefixTarget::C)),
            0xEA => Some(Instruction::SET(5,PrefixTarget::D)),
            0xEB => Some(Instruction::SET(5,PrefixTarget::E)),
            0xEC => Some(Instruction::SET(5,PrefixTarget::H)),
            0xED => Some(Instruction::SET(5,PrefixTarget::L)),
            0xEE => Some(Instruction::SET(5,PrefixTarget::HL)),
            0xEF => Some(Instruction::SET(5,PrefixTarget::A)),

            0xF0 => Some(Instruction::SET(6,PrefixTarget::B)),
            0xF1 => Some(Instruction::SET(6,PrefixTarget::C)),
            0xF2 => Some(Instruction::SET(6,PrefixTarget::D)),
            0xF3 => Some(Instruction::SET(6,PrefixTarget::E)),
            0xF4 => Some(Instruction::SET(6,PrefixTarget::H)),
            0xF5 => Some(Instruction::SET(6,PrefixTarget::L)),
            0xF6 => Some(Instruction::SET(6,PrefixTarget::HL)),
            0xF7 => Some(Instruction::SET(6,PrefixTarget::A)),

            0xF8 => Some(Instruction::SET(7,PrefixTarget::B)),
            0xF9 => Some(Instruction::SET(7,PrefixTarget::C)),
            0xFA => Some(Instruction::SET(7,PrefixTarget::D)),
            0xFB => Some(Instruction::SET(7,PrefixTarget::E)),
            0xFC => Some(Instruction::SET(7,PrefixTarget::H)),
            0xFD => Some(Instruction::SET(7,PrefixTarget::L)),
            0xFE => Some(Instruction::SET(7,PrefixTarget::HL)),
            0xFF => Some(Instruction::SET(7,PrefixTarget::A)),

          _ => None
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP()),
            0x01 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::BC, LoadByteSource::D16))),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x05 => Some(Instruction::DEC(IncDecTarget::BC)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instruction::RLCA()),

            0x08 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A16, LoadByteSource::SP))),
            0x09 => Some(Instruction::ADDHL(ArithmeticTarget::BC)),
            0x0A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BC))),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x0F => Some(Instruction::RRCA()),

            0x10 => Some(Instruction::STOP()),
            0x11 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::DE, LoadByteSource::D16))),
            0x12 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::DE, LoadByteSource::A))),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instruction::RLA()),

            0x18 => Some(Instruction::JR(JumpTest::Always)),
            0x19 => Some(Instruction::ADDHL(ArithmeticTarget::DE)),
            0x1A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DE))),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instruction::RRA()),

            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x21 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D16))),
            0x22 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => Some(Instruction::DAA()),

            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x29 => Some(Instruction::ADDHL(ArithmeticTarget::HL)),
            0x2A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL()),

            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x31 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::SP, LoadByteSource::D16))),
            0x32 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLD, LoadByteSource::A))),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            0x34 => Some(Instruction::INC(IncDecTarget::HL)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF()),

            0x38 => Some(Instruction::JR(JumpTest::Carry)),
            0x39 => Some(Instruction::ADDHL(ArithmeticTarget::SP)),
            0x3A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLD))),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instruction::CCF()),

            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HL))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),

            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HL))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),

            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),

            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::L))),
            0x76 => Some(Instruction::HALT()),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::A))),

            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),

            0x81 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),

            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),

            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),

            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),

            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),

            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),

            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::HL)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),

            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::HL)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),

            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero,JumpTarget::A16)),
            0xC3 => Some(Instruction::JP(JumpTest::Always,JumpTarget::A16)),
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xC7 => Some(Instruction::RST(RestartTarget::H00)),

            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xCA => Some(Instruction::JP(JumpTest::Zero,JumpTarget::A16)),
            0xCB => Some(Instruction::PREFIX()),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xCD => Some(Instruction::CALL(JumpTest::Always)),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),
            0xCF => Some(Instruction::RST(RestartTarget::H08)),

            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry,JumpTarget::A16)),
            0xD3 => /* INVALID */ None,
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),
            0xD7 => Some(Instruction::RST(RestartTarget::H10)),

            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xD9 => Some(Instruction::RETI()),
            0xDA => Some(Instruction::JP(JumpTest::Carry,JumpTarget::A16)),
            0xDB => /* INVALID */ None,
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),
            0xDD => /* INVALID */ None,
            0xDE => Some(Instruction::SBC(ArithmeticTarget::D8)),
            0xDF => Some(Instruction::RST(RestartTarget::H18)),

            0xE0 => Some(Instruction::LDH(LoadType::Byte(LoadByteTarget::A8, LoadByteSource::A))),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::FF00C, LoadByteSource::A))),
            0xE3 => /* INVALID */ None,
            0xE4 => /* INVALID */ None,
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),
            0xE7 => Some(Instruction::RST(RestartTarget::H20)),

            0xE8 => Some(Instruction::ADD(ArithmeticTarget::SP)),
            0xE9 => Some(Instruction::JP(JumpTest::Always,JumpTarget::HL)),
            0xEA => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A16, LoadByteSource::A))),
            0xEB => /* INVALID */ None,
            0xEC => /* INVALID */ None,
            0xED => /* INVALID */ None,
            0xEE => Some(Instruction::XOR(ArithmeticTarget::D8)),
            0xEF => Some(Instruction::RST(RestartTarget::H28)),

            0xF0 => Some(Instruction::LDH(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A8))),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::FF00C))),
            0xF3 => Some(Instruction::DI()),
            0xF4 => /* INVALID */ None,
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),
            0xF7 => Some(Instruction::RST(RestartTarget::H30)),

            0xF8 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL, LoadByteSource::SP))),
            0xF9 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::SP, LoadByteSource::HL))),
            0xFA => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A16))),
            0xFB => Some(Instruction::EI()),
            0xFC => /* INVALID */ None,
            0xFD => /* INVALID */ None,
            0xFE => Some(Instruction::CP(ArithmeticTarget::D8)),
            0xFF => Some(Instruction::RST(RestartTarget::H38)),

          _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }

}