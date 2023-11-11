use crate::instruction::*;
use crate::launch;
use crate::ppu;
use crate::register;
use crate::timer::*;
pub struct CPU{
    pub registers:register::Registers,
    program_counter:u16,
    stack_pointer:u16,
    pub bus:MemoryBus,
    pub is_halted:bool,
    ei:u8,
    di:u8,
    last_pc:u16,
    pub jpad_interrupt: bool,
}
pub struct MemoryBus{
    rom: Vec<u8>,
    wram:[u8;0x2000],
    hram:[u8;0x80],
    pub ppu:ppu::PPU,
    ime:bool,
    ie:u8,
    if_reg:u8,
    pub timer:Timer,
    pub joypad: u8,
}

impl MemoryBus {
  fn read_rom(&self, address: u16) -> u8 {
    self.rom[address as usize]
  }
  
  fn wram_read(&self,address: u16)->u8{
    self.wram[(address & 0x1FFF) as usize]
  }

  fn wram_write(&mut self,address: u16,val:u8){
    self.wram[(address & 0x1FFF) as usize] = val;
  }

  fn hram_read(&self,address: u16)->u8{
    self.hram[(address & 0x007F) as usize]
  }

  fn hram_write(&mut self,address: u16,val:u8) {
    self.hram[(address & 0x007F) as usize] = val;
  }

  pub fn bus_read(&self,address:u16)->u8{
    match address{
      0x0000..=0x7FFF |  0xA000..=0xBFFF => self.read_rom(address), //ROM
      0x8000..=0x9FFF => self.ppu.vram_read(address), //VRAM
      0xC000..=0xDFFF=>self.wram_read(address),//WRAM
      0xE000..=0xFDFF=>0,//ECHO RAM
      0xFE00..=0xFE9F=>self.ppu.oam_read(address),//OAM
      0xFEA0..=0xFEFF=>0,//Not usable
      0xFF00 => self.joypad, //Joypad
      0xFF01..=0xFF02 => panic!("Serial transfer Link Cable"),
      0xFF04..=0xFF07 =>self.timer.timer_read(address), //Timer
      0xFF0F =>self.if_reg, //IF interrupt flags
      0xFF40..=0xFF4B => self.lcd_read(address),
      0xFF4C..=0xFF7F => panic!(),
      0xFF80..=0xFFFE=>self.hram_read(address),//HRAM
      0xFFFF =>self.ie,//IE interrupt enable
      _ =>0
    }
    
  }

  fn bus_write(&mut self,address:u16,val:u8){
    match address{
      0x0000..=0x7FFF => (), //ROM
      0x8000..=0x9FFF => self.ppu.vram_write(address,val), //VRAM
      0xA000..=0xBFFF =>(), //External RAM
      0xC000..=0xDFFF=>self.wram_write(address,val),//WRAM
      0xE000..=0xFDFF=>(),//ECHO RAM
      0xFE00..=0xFE9F=>self.ppu.oam_write(address,val),//OAM
      0xFEA0..=0xFEFF=>(),//Not usable
      0xFF00 => self.joypad = val, //Joypad
      0xFF01..=0xFF02 => (),
      0xFF04..=0xFF07 =>self.timer.timer_write(address, val), //Timer
      0xFF0F =>self.if_reg = val, //IF interrupt flags
      0xFF40..=0xFF4B => self.lcd_write(address,val),
      0xFF4C..=0xFF7F => (),
      0xFF80..=0xFFFE=>self.hram_write(address,val),//HRAM
      0xFFFF =>self.ie = val,//IE interrupt enable
      _ =>println!("\x1b[93mWrite to {}\x1b[0m",address)
    }
    
  }

  pub fn lcd_read(&self,address:u16)->u8{
    match address{  
        0xFF40 => self.ppu.lcdc,
        0xFF41 => self.ppu.lcds,
        0xFF42 => self.ppu.scy,
        0xFF43 => self.ppu.scx,
        0xFF44 => self.ppu.ly,
        0xFF45 => self.ppu.lyc,
        0xFF46 => {
            panic!("Can't read DMA")
        },
        0xFF47 => self.ppu.bg_palette,
        0xFF48 => self.ppu.obp0,
        0xFF49 => self.ppu.obp1,
        0xFF4A => self.ppu.wy,
        0xFF4B => self.ppu.wx,
        _ =>panic!()
    }
  }

  pub fn lcd_write(&mut self,address:u16,val:u8){
      match address{
          0xFF40 => self.ppu.lcdc = val,
          0xFF41 => self.ppu.lcds = val,
          0xFF42 => self.ppu.scy = val,
          0xFF43 => self.ppu.scx = val,
          0xFF44 => (),
          0xFF45 => self.ppu.lyc = val,
          0xFF46 => self.dma_transfer(val),
          0xFF47 => self.ppu.bg_palette = val,
          0xFF48 => self.ppu.obp0 = val,
          0xFF49 => self.ppu.obp1 = val,
          0xFF4A => self.ppu.wy = val,
          0xFF4B => self.ppu.wx = val,
          _ =>panic!()
      }
  }

  fn dma_transfer(&mut self,start:u8){
    let address = start as u16 * 0x100;
    for i in 0..0xA0u16{
        let val = self.bus_read(address+i);
        self.ppu.oam_write(i, val);
    }
  }
}

pub enum EmulatorError {
  /*
  InvalidOpcode,
  MemoryReadError,
  MemoryWriteError,
  StackOverflow,
  StackUnderflow,
  InvalidAddress,
  InterruptHandlingError,
  */
  // Add more error variants as needed
}

impl CPU {
  pub fn new() -> CPU{
    let flags = register::FlagsRegister  {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: true,
    };
    let regs = register::Registers{
      a:0x01,
      b:0x00,
      c:0x13,
      d:0x00,
      e:0xD8,
      f:flags,
      h:0x01,
      l:0x4D,
    };
    let mem_bus = MemoryBus {
      rom:launch::launch("./tetris.gb", 64),
      wram:[0u8;0x2000],
      hram:[0u8;0x80],
      ppu:ppu::PPU::new(),
      ime: false,
      ie: 0,
      if_reg: 0,
      timer: Timer::new(),
      joypad: 0b00111111,
    };
    CPU {
      registers: regs,
      program_counter: 0x0100,
      stack_pointer: 0xFFFE,
      is_halted: false,
      bus: mem_bus,
      ei:0,
      di:0,
      last_pc:0,
      jpad_interrupt: false,
      
    }
  }

  fn read_next_byte(&self) -> u8 {
    self.bus.bus_read(self.program_counter.wrapping_add(1))
  }

  fn read_next_word(&self) -> u16{
    let tmp_pc=self.program_counter.wrapping_add(1);

    self.bus.bus_read(tmp_pc) as u16 | ((self.bus.bus_read(tmp_pc.wrapping_add(1)) as u16) << 8)
  }

  pub fn step(&mut self) {
    
    let mut instruction_byte = self.bus.bus_read(self.program_counter);
    
    let prefixed = instruction_byte == 0xCB;
    if prefixed {
      instruction_byte = self.bus.bus_read(self.program_counter + 1);
      self.bus.timer.timer_tick(4);
    }

    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte,prefixed) {
      self.execute(instruction)
    } else {
      panic!("Unkown instruction found for: {}", instruction_byte);
    };
    self.program_counter = next_pc;
  }


  fn execute(&mut self, instruction: Instruction) ->u16{
    self.update_ime();
    self.last_pc = self.program_counter;
    let instruction_name = instruction_name(&instruction);
    println!("Executing {} PC = {:#06x}, a: {} b : {} c :{} d : {} e: {} h: {} l: {} lcdc :{:08b} ly: {}, interrupt: {}, SP: {}", instruction_name,self.program_counter,self.registers.a
  , self.registers.b,self.registers.c,self.registers.d,self.registers.e,self.registers.h,self.registers.l,self.bus.ppu.lcdc,self.bus.ppu.ly,self.bus.if_reg,self.stack_pointer);
    
    match instruction { 
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::HL => {
            let address = self.registers.get_hl();
            let value = self.bus.bus_read(address);
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 => {
            let immediate_value = self.read_next_byte();
            let new_value = self.add(immediate_value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          }
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::SP => {
            self.stack_pointer = self.add_sp();
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          _ => {panic!()}
        }
      },
      Instruction::ADDHL(target) =>{
        match target {
          ArithmeticTarget::BC =>{
            let bc = self.registers.get_bc();
            let add = self.addhl(bc);
            self.registers.set_hl(add);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::DE =>{
            let de = self.registers.get_de();
            let add =self.addhl(de);
            self.registers.set_hl(add);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::HL =>{
            let hl = self.registers.get_hl();
            let add = self.addhl(hl);
            self.registers.set_hl(add);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::SP =>{
            let sp =self.stack_pointer;
            let add = self.addhl(sp);
            self.registers.set_hl(add);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          },
          _=>{panic!("")}
        }
      },
      Instruction::SUB(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::HL => {
            let address = self.registers.get_hl();
            let value = self.bus.bus_read(address);
            let (new_value, _did_overflow) = self.sub(value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 => {
            let immediate_value = self.read_next_byte();
            let (new_value, _did_overflow) = self.sub(immediate_value);
            self.registers.a = new_value;
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          }
          _=>{self.program_counter}    
        }
      },
      Instruction::AND(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            self.and(value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::HL => {
            let address = self.registers.get_hl();
            let value = self.bus.bus_read(address);
            self.and_hl(value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 => {
            let immediate_value = self.read_next_byte();
            self.and(immediate_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          }
          _=>{self.program_counter}
        }
      },    
      Instruction::SBC(target) => {
        match target {
          ArithmeticTarget::A => {
            let a =self.registers.a;
            self.sbc(&a);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::B => {
            let b = self.registers.b;
            self.sbc(&b);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::C => {
            let c = self.registers.c;
            self.sbc(&c);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::D => {
            let d = self.registers.d;
            self.sbc(&d);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::E => {
            let e = self.registers.e;
            self.sbc(&e);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::H => {
            let h = self.registers.h;
            self.sbc(&h);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::L => {
            let l = self.registers.l;
            self.sbc(&l);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::HL => {
              // Read the value from memory at the address pointed to by HL
              let address = self.registers.get_hl();
              let value = self.bus.bus_read(address);
              self.sbc_hl(value);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 =>{
            let mut immediate_value = self.read_next_byte();
            self.sbc(&mut immediate_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          },
          _=>{self.program_counter.wrapping_add(1)}
        }
      },
      Instruction::OR(target) => {
        match target {
          ArithmeticTarget::A => {
            let a =self.registers.a;
            self.or(&a);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::B => {
            let b = self.registers.b;
            self.or(&b);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::C => {
            let c = self.registers.c;
            self.or(&c);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::D => {
            let d = self.registers.d;
            self.or(&d);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::E => {
            let e = self.registers.e;
            self.or(&e);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::H => {
            let h = self.registers.h;
            self.or(&h);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::L => {
            let l =self.registers.l;
            self.or(&l);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::HL => {
              // Read the value from memory at the address pointed to by HL
              let address = self.registers.get_hl();
              let value = self.bus.bus_read(address);
              self.or_hl(value);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 => {
            let mut immediate_value = self.read_next_byte();
            self.or(&mut immediate_value);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(2)
          },
          _=>{self.program_counter.wrapping_add(1)}
        } 
      },    
      Instruction::XOR(target) => {
        match target {
          ArithmeticTarget::A => {
            let a = self.registers.a;
            self.xor(&a);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::B => {
            let b = self.registers.b;
            self.xor(&b);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::C => {
            let c =self.registers.c;
            self.xor(&c);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::D => {
            let d = self.registers.d;
            self.xor(&d);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::E => {
            let e =self.registers.e;
            self.xor(&e);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::H => {
            let h = self.registers.h;
            self.xor(&h);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::L => {
            let l =self.registers.l;
            self.xor(&l);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::HL => {
            // Read the value from memory at the address pointed to by HL
            let address = self.registers.get_hl();
            let value = self.bus.bus_read(address);
            self.xor_hl(value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 => {
            let mut immediate_value = self.read_next_byte();
            self.xor(&mut immediate_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          },
          _=>{self.program_counter.wrapping_add(1)}
        }
      }, 
      Instruction::CP(target) => {
        match target {
          ArithmeticTarget::A => {
            let a = self.registers.a;
            self.cp(&a);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::B => {
            let b = self.registers.b;
            self.cp(&b);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::C => {
            let c = self.registers.c;
            self.cp(&c);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::D => {
            let d = self.registers.d;
            self.cp(&d);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::E => {
            let e = self.registers.e;
            self.cp(&e);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::H => {
            let h = self.registers.h;
            self.cp(&h);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::L => {
            let l = self.registers.l;
            self.cp(&l);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::HL => {
            // Read the value from memory at the address pointed to by HL
            let address = self.registers.get_hl();
            let value = self.bus.bus_read(address);
            self.cp(&value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          ArithmeticTarget::D8 => {
            let mut immediate_value = self.read_next_byte();
            self.cp(&mut immediate_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          },
          _=>{self.program_counter.wrapping_add(1)}
        }
      }, 
      Instruction::INC(target) => {
        match target {
          IncDecTarget::A => {
            let a  = self.registers.a.wrapping_add(1); 
            self.inc(&a);
            self.registers.a = a;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::B => {
            let b  = self.registers.b.wrapping_add(1); 
            self.inc(&b);
            self.registers.b = b;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::C => {
            let c  = self.registers.c.wrapping_add(1); 
            self.inc(&c);
            self.registers.c = c;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::D => {
            let d  = self.registers.d.wrapping_add(1); 
            self.inc(&d);
            self.registers.d = d;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::E => {
            let e  = self.registers.e.wrapping_add(1); 
            self.inc(&e);
            self.registers.e = e;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::H => {
            let h  = self.registers.h.wrapping_add(1); 
            self.inc(&h);
            self.registers.h = h;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::L => {
            let l  = self.registers.l.wrapping_add(1); 
            self.inc(&l);
            self.registers.l = l;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::HLP => {
            // Read the value from memory at the address pointed to by HL
            let address = self.registers.get_hl();
            let mut value = self.bus.bus_read(address);
            self.inc(&mut value);
            // Write the modified value back to memory
            self.bus.bus_write(address, value);
            self.bus.timer.timer_tick(12);
            self.program_counter.wrapping_add(1)
          }
          IncDecTarget::HL => {
            self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::BC =>{
            let value = self.registers.get_bc();
            let new_value = value.wrapping_add(1);
            self.registers.set_bc(new_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          IncDecTarget::DE =>{
            let value = self.registers.get_de();
            let new_value = value.wrapping_add(1);
            self.registers.set_de(new_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          IncDecTarget::SP =>{
            self.stack_pointer = self.stack_pointer.wrapping_add(1);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
        }
      },
      Instruction::DEC(target) => {
        match target {
          IncDecTarget::A => {
            let a  = self.registers.a.wrapping_sub(1); 
            self.dec(&a);
            self.registers.a = a;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::B => {
            let b  = self.registers.b.wrapping_sub(1);  
            self.dec(&b);
            self.registers.b = b;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::C => {
            let c  = self.registers.c.wrapping_sub(1); 
            self.dec(&c);
            self.registers.c = c;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::D => {
            let d  = self.registers.d.wrapping_sub(1); 
            self.dec(&d);
            self.registers.d = d;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::E => {
            let e  = self.registers.e.wrapping_sub(1); 
            self.dec(&e);
            self.registers.e = e;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::H => {
            let h  = self.registers.h.wrapping_sub(1); 
            self.dec(&h);
            self.registers.h = h;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::L => {
            let l  = self.registers.l.wrapping_sub(1); 
            self.dec(&l);
            self.registers.l = l;
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::HLP => {
            // Read the value from memory at the address pointed to by HL
            let address = self.registers.get_hl();
            let mut value = self.bus.bus_read(address);
            self.dec(&mut value);
            // Write the modified value back to memory
            self.bus.bus_write(address, value);
            self.bus.timer.timer_tick(12);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::HL => {
            self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          },
          IncDecTarget::BC =>{
            let value = self.registers.get_bc();
            let new_value = value.wrapping_sub(1);
            self.registers.set_bc(new_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          IncDecTarget::DE =>{
            let value = self.registers.get_de();
            let new_value = value.wrapping_sub(1);
            self.registers.set_de(new_value);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
          IncDecTarget::SP =>{
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(1)
          }
        }
      }, 
      Instruction::CCF() => {
        self.ccf();
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      },
      Instruction::SCF() => {
        self.scf();
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      },
      Instruction::RRA() => {
        self.rra();
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      },
      Instruction::RLA() => {
        self.rla();
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      },
      Instruction::RRCA() => {
        self.rrca();
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      },
      Instruction::RLCA() => {
        self.rlca();
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      }, 
      Instruction::ADC(target) => {
        match target {
          ArithmeticTarget::A => {
            self.adc(self.registers.a);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::B => {
            self.adc(self.registers.b);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::C => {
            self.adc(self.registers.c);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::D => {
            self.adc(self.registers.d);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::E => {
            self.adc(self.registers.e);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::H => {
            self.adc(self.registers.h);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::L => {
            self.adc(self.registers.l);
            self.bus.timer.timer_tick(4);
            self.program_counter.wrapping_add(1)
          },
          ArithmeticTarget::HL => {
            self.adc(self.bus.bus_read(self.registers.get_hl()));
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          },
          ArithmeticTarget::D8 => {
            let immediate = self.read_next_byte();
            self.adc(immediate);
            self.bus.timer.timer_tick(8);
            self.program_counter.wrapping_add(2)
          },
          _ =>panic!()
        }
      },
      Instruction::CPL() => {
        // Perform the complement operation on register A
        self.registers.a = !self.registers.a;

        // Update flags
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      },
      Instruction::BIT(bit, target) => {
        match target {
            PrefixTarget::A => {
              self.bit(bit, self.registers.a);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::B => {
              self.bit(bit, self.registers.b);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::C => {
              self.bit(bit, self.registers.c);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::D => {
              self.bit(bit, self.registers.d);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::E => {
              self.bit(bit, self.registers.e);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::H => {
              self.bit(bit, self.registers.h);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::L => {
              self.bit(bit, self.registers.l);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::HL => {
              // Read the value from memory at the address pointed to by HL
              let address = self.registers.get_hl();
              let value = self.bus.bus_read(address);
              self.bit(bit, value);
              self.bus.timer.timer_tick(16);
              self.program_counter.wrapping_add(2)
            },
        }
      }, 
      Instruction::RES(bit, target) => {
        match target {
            PrefixTarget::A => {
              let a = self.registers.a;
              self.registers.a = self.res(bit,a);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::B => {
              let b = self.registers.b;
              self.registers.b = self.res(bit,b);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::C => {
              let c = self.registers.c;
              self.registers.c = self.res(bit,c);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::D => {
              let d = self.registers.d;
              self.registers.d = self.res(bit,d);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::E => {
              let e = self.registers.e;
              self.registers.e = self.res(bit,e);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::H => {
              let h = self.registers.h;
              self.registers.h = self.res(bit,h);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::L => {
              let l = self.registers.l;
              self.registers.l = self.res(bit,l);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::HL => {
              // Read the value from memory at the address pointed to by HL
              let address = self.registers.get_hl();
              let value = self.bus.bus_read(address);
              let res_val = self.res(bit, value);
              // Write the modified value back to memory
              self.bus.bus_write(address, res_val);
              self.bus.timer.timer_tick(16);
              self.program_counter.wrapping_add(2)
            }
        }
      },
      Instruction::SET(bit, target) => {
        match target {
            PrefixTarget::A => {
              let a = self.registers.a;
              self.registers.a = self.set(bit,a);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::B => {
              let b = self.registers.b;
              self.registers.b = self.set(bit,b);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::C => {
              let c = self.registers.c;
              self.registers.c = self.set(bit,c);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::D => {
              let d = self.registers.d;
              self.registers.d = self.set(bit,d);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::E => {
              let e = self.registers.e;
              self.registers.e = self.set(bit,e);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::H => {
              let h = self.registers.h;
              self.registers.h = self.set(bit,h);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::L => {
              let l = self.registers.l;
              self.registers.l = self.set(bit,l);
              self.bus.timer.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            PrefixTarget::HL => {
              // Read the value from memory at the address pointed to by HL
              let address = self.registers.get_hl();
              let value = self.bus.bus_read(address);
              let set_val = self.set(bit, value);
              // Write the modified value back to memory
              self.bus.bus_write(address, set_val);
              self.bus.timer.timer_tick(16);
              self.program_counter.wrapping_add(2)
            }
        }
      },
      Instruction::SRL(target) => {
          match target {
              PrefixTarget::A => {
                let a  = self.registers.a;
                self.srl(&a);
                self.registers.a >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b  = self.registers.b;
                self.srl(&b);
                self.registers.b >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c  = self.registers.c;
                self.srl(&c);
                self.registers.c >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d  = self.registers.d;
                self.srl(&d);
                self.registers.d >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e  = self.registers.e;
                self.srl(&e);
                self.registers.e >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h = self.registers.h;
                self.srl(&h);
                self.registers.h >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l  = self.registers.l;
                self.srl(&l);
                self.registers.l >>= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let mut value = self.bus.bus_read(address);
                self.srl(&value);
                value >>= 1;
                // Write the modified value back to memory
                self.bus.bus_write(address, value);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::RR(target) => {
          match target {
              PrefixTarget::A => {
                let a  = self.registers.a;
                self.registers.a =self.rr(a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b  = self.registers.b;
                self.registers.b =self.rr(b);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c  = self.registers.c;
                self.registers.c =self.rr(c);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d  = self.registers.d;
                self.registers.d =self.rr(d);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e = self.registers.e;
                self.registers.e =self.rr(e);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h  = self.registers.h;
                self.registers.h =self.rr(h);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l  = self.registers.l;
                self.registers.l =self.rr(l);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.bus_read(address);
                let rr_val = self.rr(value);
                // Write the modified value back to memory
                self.bus.bus_write(address, rr_val);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::RL(target) => {
          match target {
              PrefixTarget::A => {
                let a = self.registers.a;
                self.registers.a = self.rl(a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b = self.registers.b;
                self.registers.b = self.rl(b);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c = self.registers.c;
                self.registers.c = self.rl(c);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d = self.registers.d;
                self.registers.d = self.rl(d);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e = self.registers.e;
                self.registers.e = self.rl(e);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h = self.registers.h;
                self.registers.h = self.rl(h);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l = self.registers.l;
                self.registers.l = self.rl(l);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.bus_read(address);
                let rl_val = self.rl(value);
                // Write the modified value back to memory
                self.bus.bus_write(address, rl_val);
                self.bus.timer.timer_tick(16);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::RRC(target) => {
          match target {
              PrefixTarget::A => {
                let a =self.registers.a;
                self.registers.a = self.rrc(a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b =self.registers.b;
                self.registers.b = self.rrc(b);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c =self.registers.c;
                self.registers.c = self.rrc(c);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d =self.registers.d;
                self.registers.d = self.rrc(d);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e =self.registers.e;
                self.registers.e = self.rrc(e);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h =self.registers.h;
                self.registers.h = self.rrc(h);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l =self.registers.l;
                self.registers.l = self.rrc(l);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.bus_read(address);
                let rrc_val = self.rrc(value);
                // Write the modified value back to memory
                self.bus.bus_write(address, rrc_val);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::RLC(target) => {
          match target {
              PrefixTarget::A => {
                let a =self.registers.a;
                self.registers.a = self.rlc(a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b =self.registers.b;
                self.registers.b = self.rlc(b);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c =self.registers.c;
                self.registers.c = self.rlc(c);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d =self.registers.d;
                self.registers.d = self.rlc(d);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e =self.registers.e;
                self.registers.e = self.rlc(e);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h =self.registers.h;
                self.registers.h = self.rlc(h);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l =self.registers.l;
                self.registers.l = self.rlc(l);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.bus_read(address);
                let rlc_val = self.rlc(value);
                
                // Write the modified value back to memory
                self.bus.bus_write(address, rlc_val);
                self.bus.timer.timer_tick(16);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::SRA(target) => {
          match target {
              PrefixTarget::A => {
                let a = self.registers.a;
                self.registers.a =self.sra(a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b = self.registers.b;
                self.registers.b =self.sra(b);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c = self.registers.c;
                self.registers.c =self.sra(c);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d = self.registers.d;
                self.registers.d =self.sra(d);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e = self.registers.e;
                self.registers.e =self.sra(e);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h = self.registers.h;
                self.registers.h =self.sra(h);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l = self.registers.l;
                self.registers.l =self.sra(l);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.bus_read(address);
                let sra_val = self.sra(value);
                // Write the modified value back to memory
                self.bus.bus_write(address, sra_val);
                self.bus.timer.timer_tick(16);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::SLA(target) => {
          match target {
              PrefixTarget::A => {
                let a = self.registers.a;
                self.sla(&a);
                self.registers.a <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b = self.registers.b;
                self.sla(&b);
                self.registers.b <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c = self.registers.c;
                self.sla(&c);
                self.registers.c <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d = self.registers.d;
                self.sla(&d);
                self.registers.d <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e = self.registers.e;
                self.sla(&e);
                self.registers.e <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h = self.registers.h;
                self.sla(&h);
                self.registers.h <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l = self.registers.l;
                self.sla(&l);
                self.registers.l <<= 1;
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let mut value = self.bus.bus_read(address);
                self.sla(&value);
                value <<=1;
                // Write the modified value back to memory
                self.bus.bus_write(address, value );
                self.bus.timer.timer_tick(16);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::SWAP(target) => {
          match target {
              PrefixTarget::A => {
                let a = self.registers.a;
                self.registers.a = self.swap(a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                let b = self.registers.b;
                self.registers.b = self.swap(b);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                let c = self.registers.c;
                self.registers.c = self.swap(c);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                let d = self.registers.d;
                self.registers.d = self.swap(d);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                let e = self.registers.e;
                self.registers.e = self.swap(e);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                let h = self.registers.h;
                self.registers.h = self.swap(h);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                let l = self.registers.l;
                self.registers.l = self.swap(l);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.bus_read(address);
                let swap_val =self.swap(value);
                // Write the modified value back to memory
                self.bus.bus_write(address, swap_val);
                self.bus.timer.timer_tick(16);
                self.program_counter.wrapping_add(2)
              }
          }
      },
      Instruction::JP(test,target) => {
        let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true
        };
        if jump_condition {
          self.bus.timer.timer_tick(16);
        }else {
          self.bus.timer.timer_tick(12);
        }
        match target{
          JumpTarget::A16 => {
            self.jump(jump_condition)
          },
          JumpTarget::HL =>{
            self.jump_hl(jump_condition)
          },
        }  
      }
      Instruction::LD(load_type) => {
        match load_type {
          LoadType::Byte(target, source) => {
            match target {
              LoadByteTarget::BC =>{
                match source{
                  LoadByteSource::A=>{
                    self.bus.bus_write(self.registers.get_bc(), self.registers.a);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D16=>{
                    self.registers.set_bc(self.read_next_word());
                    self.bus.timer.timer_tick(12);
                    self.program_counter.wrapping_add(3)
                  },
                  _=>{panic!{"Err:"}}
                }
              },
              LoadByteTarget::DE =>{
                match source{
                  LoadByteSource::A=>{
                    self.bus.bus_write(self.registers.get_de(), self.registers.a);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D16=>{
                    self.registers.set_de(self.read_next_word());
                    self.bus.timer.timer_tick(12);
                    self.program_counter.wrapping_add(3)
                  },
                  _=>{panic!{"Err:"}}
                }
              },
              LoadByteTarget::HL =>{
                match source{
                  LoadByteSource::A=>{
                    self.bus.bus_write(self.registers.get_hl(), self.registers.a);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::B => {
                    self.bus.bus_write(self.registers.get_hl(), self.registers.b);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C => {
                    self.bus.bus_write(self.registers.get_hl(), self.registers.c);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D => {
                    self.bus.bus_write(self.registers.get_hl(), self.registers.d);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E => {
                    self.bus.bus_write(self.registers.get_hl(), self.registers.e);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H => {
                    self.bus.bus_write(self.registers.get_hl(), self.registers.h);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L => {
                    self.bus.bus_write(self.registers.get_hl(), self.registers.l);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },                 
                  LoadByteSource::D16=>{
                    let next_word = self.read_next_word();
                    self.registers.set_hl(next_word);
                    self.bus.timer.timer_tick(12);
                    self.program_counter.wrapping_add(3)
                  },
                  LoadByteSource::SP=>{
                    let r8 = self.read_next_byte() as i8 as i32;
                    let sp = self.stack_pointer as i32;
                    self.registers.set_hl(sp.wrapping_add(r8) as u16);
                    self.registers.f.zero = false;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry =(sp as u16 & 0x0F) + (r8 as u16 & 0x0F) > 0x0F;
                    self.registers.f.carry = (sp as u16 & 0xFF) + (r8 as u16 & 0xFF) > 0xFF;
                    self.bus.timer.timer_tick(12);
                    self.program_counter.wrapping_add(2)
                  }
                  LoadByteSource::D8 => {
                    self.bus.bus_write(self.registers.get_hl(), self.read_next_byte());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _=>{panic!{"Err:"}}
                }
              },
              LoadByteTarget::SP => {
                match source{
                  LoadByteSource::D16=>{
                    self.stack_pointer = self.read_next_word();
                    self.bus.timer.timer_tick(12);
                    self.program_counter.wrapping_add(3)
                  },
                  LoadByteSource::HL=>{
                    self.stack_pointer = self.registers.get_hl();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  _=>{panic!()}
                }
              },
              LoadByteTarget::HLI => {
                self.bus.bus_write(self.registers.get_hl(), self.registers.a);
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(1)
              },
              LoadByteTarget::HLD =>{
                self.bus.bus_write(self.registers.get_hl(), self.registers.a);
                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(1)
              },
              LoadByteTarget::A => {
                match source{
                  LoadByteSource::BC =>{
                    self.registers.a = self.bus.bus_read(self.registers.get_bc());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::DE =>{
                    self.registers.a = self.bus.bus_read(self.registers.get_de());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HLI =>{
                    self.registers.a = self.bus.bus_read(self.registers.get_hl());
                    self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HLD =>{
                    self.registers.a = self.bus.bus_read(self.registers.get_hl());
                    self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::B =>{
                    self.registers.a = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.a = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.a = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.a = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.a = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.a = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.a = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.a = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  LoadByteSource::A =>{
                    self.registers.a = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A8 =>{
                    self.registers.a = self.bus.bus_read(0xFF00 | self.read_next_byte() as u16);
                    self.bus.timer.timer_tick(12);
                    self.program_counter.wrapping_add(2)
                  },
                  LoadByteSource::A16 =>{
                    self.registers.a = self.bus.bus_read(self.read_next_word());
                    self.bus.timer.timer_tick(16);
                    self.program_counter.wrapping_add(3)
                  },
                  LoadByteSource::FF00C =>{
                    self.registers.a = self.bus.bus_read(0xFF00 | self.registers.c as u16);
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  _ => {self.program_counter.wrapping_add(1)}
                }
              },
              LoadByteTarget::B => {
                match source{
                  LoadByteSource::B =>{
                    self.registers.b = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.b = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.b = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.b = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.b = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.b = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.b = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A =>{
                    self.registers.b = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.b = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _ => {self.program_counter.wrapping_add(1)}
                }
              },
              LoadByteTarget::C => {
                match source{
                  LoadByteSource::B =>{
                    self.registers.c = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.c = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.c = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.c = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.c = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.c = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.c = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A =>{
                    self.registers.c = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.c = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _ => {self.program_counter.wrapping_add(1)}
                }
              },
              LoadByteTarget::D => {
                match source{
                  LoadByteSource::B =>{
                    self.registers.d = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.d = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.d = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.d = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.d = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.d = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.d = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A =>{
                    self.registers.d = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.d = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _ => {self.program_counter.wrapping_add(1)}
                }
              },
              LoadByteTarget::E => {
                match source{
                  LoadByteSource::B =>{
                    self.registers.e = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.e = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.e = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.e = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.e = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.e = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.e = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A =>{
                    self.registers.e = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.e = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _ => {panic!()}
                }
              },
              LoadByteTarget::H=> {
                match source{
                  LoadByteSource::B =>{
                    self.registers.h = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.h = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.h = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.h = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.h = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.h = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.h = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A =>{
                    self.registers.h = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.h = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _ => {panic!()}
                }
              },
              LoadByteTarget::L => {
                match source{
                  LoadByteSource::B =>{
                    self.registers.l = self.registers.b;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::C =>{
                    self.registers.l = self.registers.c;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D =>{
                    self.registers.l = self.registers.d;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::E =>{
                    self.registers.l = self.registers.e;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::H =>{
                    self.registers.l = self.registers.h;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::L =>{
                    self.registers.l = self.registers.l;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::HL =>{
                    self.registers.l = self.bus.bus_read(self.registers.get_hl());
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::A =>{
                    self.registers.l = self.registers.a;
                    self.bus.timer.timer_tick(4);
                    self.program_counter.wrapping_add(1)
                  },
                  LoadByteSource::D8 =>{
                    self.registers.l = self.read_next_byte();
                    self.bus.timer.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                  },
                  _ => {panic!()}
                }
              },
              LoadByteTarget::A16 =>{
                match source{
                  LoadByteSource::A =>{
                    self.bus.bus_write(self.read_next_byte() as u16, self.registers.a);
                    self.bus.timer.timer_tick(16);
                    self.program_counter.wrapping_add(3)
                  },
                  LoadByteSource::SP =>{
                    self.bus.bus_write(self.read_next_byte() as u16, self.stack_pointer as u8);
                    self.bus.timer.timer_tick(20);
                    self.program_counter.wrapping_add(3)
                  },
                  _=>{panic!()}
                }
              },
              LoadByteTarget::A8 =>{
                self.bus.bus_write(0xFF00 | self.read_next_byte() as u16, self.registers.a);
                self.bus.timer.timer_tick(12);
                self.program_counter.wrapping_add(2)
              },
              LoadByteTarget::FF00C => {
                self.bus.bus_write(0xFF00 | self.registers.c as u16, self.registers.a);
                self.bus.timer.timer_tick(8);
                self.program_counter.wrapping_add(1)
              },
            }  
          }
        }
      },
      Instruction::PUSH(target) => {
          let value = match target {
            StackTarget::BC => self.registers.get_bc(),
            StackTarget::DE => self.registers.get_de(),
            StackTarget::HL => self.registers.get_hl(),
            StackTarget::AF => self.registers.get_af(),
          };
          self.push(value);
          self.bus.timer.timer_tick(16);
          self.program_counter.wrapping_add(1)
      }
      Instruction::POP(target) => {
          let result = self.pop();
          match target {
              StackTarget::BC => self.registers.set_bc(result),
              StackTarget::DE => self.registers.set_de(result),
              StackTarget::HL => self.registers.set_hl(result),
              StackTarget::AF => self.registers.set_af(result),
          };
          self.bus.timer.timer_tick(16);
          self.program_counter.wrapping_add(1)
      }
      Instruction::CALL(test) => {
          let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true,
          };
          if jump_condition {
            self.bus.timer.timer_tick(24);
          }else{
            self.bus.timer.timer_tick(12);
          }
          self.call(jump_condition)
      }
      Instruction::RET(test) => {
          let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true,
          };
          if jump_condition {
            self.bus.timer.timer_tick(20);
          }else {
            self.bus.timer.timer_tick(8); 
          }
          self.return_(jump_condition)
      }
      Instruction::JR(test) => {
        let jump_condition = match test {
          JumpTest::NotZero => !self.registers.f.zero,
          JumpTest::NotCarry => !self.registers.f.carry,
          JumpTest::Zero => self.registers.f.zero,
          JumpTest::Carry => self.registers.f.carry,
          JumpTest::Always => true,
        };
        if jump_condition {
          self.bus.timer.timer_tick(12)
        }else{
          self.bus.timer.timer_tick(8);
        }
        
        self.jr(jump_condition)
      }
      Instruction::STOP() => {
        self.is_halted = true;
        //Halt display until button pressed
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(2)
      }
      Instruction::NOP() => {
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      } 
      Instruction::HALT() => {
        self.is_halted = true;
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      }
      Instruction::RETI() => {
        let ret = self.return_(true);
        self.bus.ime = true;
        self.bus.timer.timer_tick(16);
        ret
      }
      Instruction::EI() => {
        self.ei += 3;
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      }
      Instruction::DI() => {
        self.di += 3;
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      }
      Instruction::PREFIX() => {
        self.bus.timer.timer_tick(4);
        self.program_counter.wrapping_add(1)
      }
      Instruction::RST(restart) => {
        match restart{
          RestartTarget::H00 =>{
            self.push(self.program_counter);
            self.program_counter = 0x0000;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
          RestartTarget::H08 => {
            self.push(self.program_counter);
            self.program_counter = 0x0008;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
          RestartTarget::H10 => {
            self.push(self.program_counter);
            self.program_counter = 0x0010;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
          RestartTarget::H18 => {
            self.push(self.program_counter);
            self.program_counter = 0x0018;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
          RestartTarget::H20 =>{
            self.push(self.program_counter);
            self.program_counter = 0x0020;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
          RestartTarget::H28 => {
            self.push(self.program_counter);
            self.program_counter = 0x0028;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
          RestartTarget::H30 => {
            self.push(self.program_counter);
            self.program_counter = 0x0030;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          }
          RestartTarget::H38 => {
            self.push(self.program_counter);
            self.program_counter = 0x0038;
            self.bus.timer.timer_tick(16);
            self.program_counter.wrapping_add(1)
          },
        }
      }
      Instruction::DAA() => {
        self.daa()
      },
    }
  }
  fn push(&mut self, val:u16){  
    self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    self.bus.bus_write(self.stack_pointer, ((val & 0xFF00) >> 8) as u8);
    self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    self.bus.bus_write(self.stack_pointer, (val & 0xFF) as u8);    
  }

  fn pop(&mut self)->u16{
    let lsb = self.bus.bus_read(self.stack_pointer) as u16;
    self.stack_pointer = self.stack_pointer.wrapping_add(1);
    let msb = self.bus.bus_read(self.stack_pointer) as u16;
    self.stack_pointer = self.stack_pointer.wrapping_add(1);
    (msb << 8) | lsb
  }

  fn add(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
    new_value
  }

  fn addhl(&mut self, value: u16) -> u16 {
    let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = (self.registers.get_hl() & 0xF) + (new_value & 0xF) > 0xF;
    new_value
  }

  fn add_sp(&mut self) -> u16{
    let r8 = self.read_next_byte() as i8;
    let (new_value, did_overflow) = (self.stack_pointer as i16).overflowing_add(r8 as i16);
    self.registers.f.zero = false;
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = (self.stack_pointer & 0xF) + (new_value as u16 & 0xF) > 0xF;
    new_value as u16
  }
 
  fn sub(&mut self, value: u8) -> (u8, bool) {
    let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);
    self.registers.f.carry = did_overflow;
    (new_value, did_overflow)
  }

  fn and(&mut self, value: u8) {
    self.registers.a &= value;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = true;
    self.registers.f.carry = false;
  }

  fn and_hl(&mut self, value: u8) {
    self.registers.a &= value;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = true;
    self.registers.f.carry = false;
  }

  fn sbc(&mut self, value: &u8) {
    let a = self.registers.a;
    let carry = if self.registers.f.carry { 1 } else { 0 };
    let (result, did_overflow) = a.overflowing_sub(*value);
    let (result, did_overflow2) = result.overflowing_sub(carry);
    self.registers.a = result;  
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (a & 0x0F) < (*value & 0x0F) + carry;
    self.registers.f.carry = did_overflow || did_overflow2
  }

  fn sbc_hl(&mut self, value: u8) {
    let carry = if self.registers.f.carry { 1 } else { 0 };
    let hl_value = self.registers.get_hl();
    let (result, did_overflow) = hl_value.overflowing_sub(value as u16);
    let (result, did_overflow2) = result.overflowing_sub(carry as u16);

    self.registers.set_hl(result);
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (hl_value & 0x0FFF) < (value as u16 & 0x0FFF) + (carry as u16);
    self.registers.f.carry = did_overflow || did_overflow2;
  }

  fn or(&mut self, value: &u8) {
    self.registers.a |= *value;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = false;
  }

  fn or_hl(&mut self, value: u8) {
    self.registers.a |= value;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = false;
  }

  fn xor(&mut self, value: &u8) {
    self.registers.a ^= *value;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = false;
  }

  fn xor_hl(&mut self, value: u8) {
    self.registers.a ^= value;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = false;
  }
  
  fn cp(&mut self, value: &u8) {
    let result = self.registers.a.wrapping_sub(*value);
    self.registers.f.zero = result == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (self.registers.a & 0x0F) < (*value & 0x0F);
    self.registers.f.carry = self.registers.a < *value;
  }

  fn inc(&mut self, value: &u8) {
    self.registers.f.zero = *value == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = (*value & 0x0F) == 0;
    // Carry flag remains unchanged
  }

  fn dec(&mut self, value: &u8) {
    self.registers.f.zero = *value == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (*value & 0x0F) == 0x0F;
    // Carry flag remains unchanged
  }
  fn daa(&mut self)->u16{
    if !self.registers.f.half_carry {
      if self.registers.f.carry || self.registers.a > 0x99 {
        let _ = self.registers.a.wrapping_add(0x60);
        self.registers.f.carry = true;
      }
      if self.registers.f.half_carry || (self.registers.a & 0x0F) > 0x09{
        let _ = self.registers.a.wrapping_add(0x6);
      }
    }else{
      if self.registers.f.carry {
        let _ = self.registers.a.wrapping_sub(0x60);
      }
      if self.registers.f.half_carry {
        let _ = self.registers.a.wrapping_sub(0x6);
      }
    }
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.half_carry = false;
    self.bus.timer.timer_tick(4);
    self.program_counter.wrapping_add(1)
  }

  fn ccf(&mut self) {
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = !self.registers.f.carry;
  }

  fn scf(&mut self) {
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = true;
  }

  fn rra(&mut self) {
    let carry = self.registers.f.carry as u8;
    let old_carry = self.registers.a & 0x01;
    self.registers.a >>= 1;
    self.registers.a |= carry << 7;
    self.registers.f.zero = false; // Clear zero flag
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = old_carry != 0;
  }

  fn rla(&mut self) {
    let carry = self.registers.f.carry as u8;
    let old_carry = (self.registers.a >> 7) & 0x01;
    self.registers.a <<= 1;
    self.registers.a |= carry;
    self.registers.f.zero = false; // Clear zero flag
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = old_carry != 0;
  }

  fn rrca(&mut self) {
    let old_carry = self.registers.a & 0x01;
    self.registers.a >>= 1;
    self.registers.a |= old_carry << 7;
    self.registers.f.zero = false; // Clear zero flag
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = old_carry != 0;
  }

  fn rlca(&mut self) {
    let old_carry = self.registers.a & 0x01;
    self.registers.a >>= 1;
    self.registers.a |= (self.registers.f.carry as u8) << 7;
    self.registers.f.zero = false; // Clear zero flag
    self.registers.f.subtract = false;
    self.registers.f.half_carry = false;
    self.registers.f.carry = old_carry != 0;
  }

  fn adc(&mut self, value: u8) {
    let carry = if self.registers.f.carry { 1 } else { 0 };

    // Calculate the sum without carry
    let _sum_without_carry = self.registers.a + value;

    // Calculate the carry
    let carry_result = (self.registers.a as u16) + (value as u16) + (carry as u16);

    // Calculate the sum with carry (including overflow)
    let sum_with_carry = carry_result as u8;

    // Check for overflow (carry out of the 8-bit range)
    let did_overflow = carry_result > 0xFF;

    self.registers.f.zero = sum_with_carry == 0;
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = ((self.registers.a & 0x0F) + (value & 0x0F) + carry) > 0x0F;
    self.registers.a = sum_with_carry;
}

  fn bit(&mut self, bit: u8, value: u8) {
        // Test the specified bit in the value
        let test_bit = 1 << bit;
        let result = value & test_bit;

        // Update flags
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
  }

  fn res(&mut self, bit: u8, value: u8) -> u8{
    // Reset (clear) the specified bit in the value
    let mask = !(1 << bit);
    let mut val = value;
    val &= mask;
    val
  }

  fn set(&mut self, bit: u8, value: u8)-> u8 {
    // Set (make 1) the specified bit in the value
    let mask = 1 << bit;
    let mut val = value;
    val |= mask;
    val
  }

  fn srl(&mut self, value: &u8) {
      // Perform a logical right shift on the value
      let carry = *value & 0x01;
      
      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn rr(&mut self, value: u8) -> u8{
      // Calculate the carry bit before the rotation
      let carry = self.registers.f.carry as u8;

      // Perform a right rotation on the value with the carry bit
      let val = (value >> 1) | (carry << 7);

      // Update flags
      self.registers.f.zero = value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = (value & 0x01) != 0;
      val
  }

  fn rl(&mut self, value: u8) -> u8{
      // Calculate the carry bit before the rotation
      let carry = ((value & 0x80) != 0) as u8;

      // Perform a left rotation on the value with the carry bit
      let val = (value << 1) | self.registers.f.carry as u8;

      // Update flags
      self.registers.f.zero = value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
      val
  }

  fn rrc(&mut self, value: u8) -> u8 {
      // Calculate the carry bit before the rotation
      let carry = value & 0x01;

      // Perform a right rotation through the carry bit on the value
      let val = (value >> 1) | (carry << 7);

      // Update flags
      self.registers.f.zero = value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
      val
  }

  fn rlc(&mut self, value: u8) -> u8{
      // Calculate the carry bit before the rotation
      let carry = (value & 0x80) >> 7;

      // Perform a left rotation through the carry bit on the value
      let val = (value << 1) | carry;

      // Update flags
      self.registers.f.zero = value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
      val
  }

  fn sra(&mut self, value:u8)-> u8 {
      // Calculate the carry bit before the shift
      let carry = value & 0x01;

      // Perform an arithmetic right shift on the value
      let val = (value >> 1) | (value & 0x80);

      // Update flags
      self.registers.f.zero = value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
      val
  }

  fn sla(&mut self, value: &u8) {
      // Calculate the carry bit before the shift
      let carry = (value & 0x80) >> 7;

      // Perform an arithmetic left shift on the value
      

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn swap(&mut self, value:u8) -> u8{
      // Perform the swap operation by exchanging the upper and lower nibbles
      let val = (value << 4) | (value >> 4);

      // Update flags
      self.registers.f.zero = value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = false; // Carry flag is always reset
      val
  }


  fn jump(&mut self, should_jump: bool) -> u16 {
    if should_jump {
      let least_significant_byte = self.bus.bus_read(self.program_counter.wrapping_add(1)) as u16;
      let most_significant_byte = self.bus.bus_read(self.program_counter.wrapping_add(2)) as u16;
      println!("{}",(most_significant_byte << 8) | least_significant_byte);
        (most_significant_byte << 8) | least_significant_byte
    } else {
      self.program_counter.wrapping_add(3)
    }
  }

  fn jump_hl(&mut self, should_jump: bool ) -> u16 {
    if should_jump {
      let least_significant_byte = self.bus.bus_read(self.registers.get_hl() + 1) as u16;
      let most_significant_byte = self.bus.bus_read(self.registers.get_hl() + 2) as u16;
        (most_significant_byte << 8) | least_significant_byte
    } else {
      self.program_counter.wrapping_add(1)
    }
  }

  fn jr(&mut self, should_jump: bool) -> u16 {
    if should_jump {
      let r8 = self.read_next_byte() as i8;
      let new_pc = (self.program_counter as i16 + r8 as i16) as u16;
      new_pc.wrapping_add(2)
    } else {
      self.program_counter.wrapping_add(2)
    }
  }

  fn call(&mut self, should_jump: bool) -> u16 {
    let next_pc = self.program_counter.wrapping_add(3);
    if should_jump {
      self.push(next_pc);
      self.read_next_word()
      } else {
        next_pc
      }
  }

  fn return_(&mut self, should_jump: bool) -> u16 {
    if should_jump {
      self.pop()
    } else {
      self.program_counter.wrapping_add(1)
    }
  }
  
  pub fn interrupts(&mut self) -> Result<(), EmulatorError> {
    if self.bus.ppu.vblank_interrupt == 1 {
        self.bus.if_reg = self.bus.if_reg | (1 << 0);

    }
    if self.bus.ppu.stat_interrupt == 1 {
      self.bus.if_reg = self.bus.if_reg | (1 << 1);
    }

    if self.bus.timer.timer_interrupt == 1 {
      self.bus.if_reg = self.bus.if_reg | (1 << 2);
    }
    if self.jpad_interrupt{
      self.bus.if_reg = self.bus.if_reg | (1 << 4);
    }

    if self.bus.ime {
        let interrupt_flags = self.bus.ie & self.bus.if_reg;
        if interrupt_flags != 0 {
            self.bus.ime = false; // Disable further interrupts

            if interrupt_flags & 0b00001 != 0 {
              self.bus.ppu.vblank_interrupt = 0;
              self.bus.if_reg = self.bus.if_reg & !(1 << 0);
              self.handle_interrupt(0x0040)?; // V-Blank interrupt
            } else if interrupt_flags & 0b00010 != 0 {
              self.bus.ppu.stat_interrupt = 0;
              self.bus.if_reg = self.bus.if_reg & !(1 << 1);
              self.handle_interrupt(0x0048)?; // LCD STAT interrupt
            } else if interrupt_flags & 0b00100 != 0 {
              self.bus.timer.timer_interrupt = 0;
              self.bus.if_reg = self.bus.if_reg & !(1 << 2);
              self.handle_interrupt(0x0050)?; // Timer interrupt
            } else if interrupt_flags & 0b01000 != 0 {
              self.handle_interrupt(0x0058)?; // Serial interrupt
            } else if interrupt_flags & 0b10000 != 0 {
              self.jpad_interrupt = false;
              self.bus.if_reg = self.bus.if_reg & !(1 << 4);
              self.handle_interrupt(0x0060)?; // Joypad interrupt
            }
        }
    }
    Ok(())
  }

  fn handle_interrupt(&mut self, addr: u16) -> Result<(), EmulatorError> {
    //Do nothing during 2 cycles 
    self.bus.timer.timer_tick(2);
    // Push the return address onto the stack
    let pc = self.last_pc;
    self.push(pc);
    println!("{}",self.bus.bus_read(0xCFFD));
    self.bus.timer.timer_tick(2);
    
    // Jump to the interrupt handler
    self.program_counter = addr;
    Ok(())
  }

  
  fn update_ime(&mut self){
    //Test if there is a delayed DI instruction
    if self.di > 1 {
      self.di -= 1;
    }
    //If delay ended, update IME
    if self.di == 1 {
      self.bus.ime = false;
      self.di = 0;
    }
    //Test if there is a delayed EI instruction
    if self.ei > 1 {
      self.ei -= 1; 
    }
    //If delay ended, update IME
    if self.ei == 1 {
      self.bus.ime = true;
      self.ei = 0;
    }
  }
    
}

//string corresponding to the instruction used
fn instruction_name(instruction: &Instruction) -> String {
  match instruction {
      Instruction::ADD(target) => {
        let str = "ADD ".to_string();
        match target {
          ArithmeticTarget::A => str + "A,A",
          ArithmeticTarget::B => str + "A,B", 
          ArithmeticTarget::C => str + "A,C",
          ArithmeticTarget::D => str + "A,D", 
          ArithmeticTarget::E => str + "A,E", 
          ArithmeticTarget::H => str + "A,H", 
          ArithmeticTarget::L => str + "A,L", 
          ArithmeticTarget::HL => str + "A,(HL)", 
          ArithmeticTarget::D8 => str + "A,d8", 
          ArithmeticTarget::SP => str + " SP,r8",
          _ => "Err".to_string()
        }
      },
      Instruction::ADC(target) => {
        let str = "ADC ".to_string();
        match target {
          ArithmeticTarget::A => str + "A,A",
          ArithmeticTarget::B => str + "A,B", 
          ArithmeticTarget::C => str + "A,C",
          ArithmeticTarget::D => str + "A,D", 
          ArithmeticTarget::E => str + "A,E", 
          ArithmeticTarget::H => str + "A,H", 
          ArithmeticTarget::L => str + "A,L", 
          ArithmeticTarget::HL => str + "A,(HL)", 
          ArithmeticTarget::D8 => str + "A,d8", 
          _ => "Err".to_string()
        }
      },
      Instruction::SUB(target) => {
        let str = "SUB ".to_string();
        match target {
          ArithmeticTarget::A => str + "A",
          ArithmeticTarget::B => str + "B", 
          ArithmeticTarget::C => str + "C",
          ArithmeticTarget::D => str + "D", 
          ArithmeticTarget::E => str + "E", 
          ArithmeticTarget::H => str + "H", 
          ArithmeticTarget::L => str + "L", 
          ArithmeticTarget::HL => str + "(HL)", 
          ArithmeticTarget::D8 => str + "d8", 
          _ => "Err".to_string()
        }
      },
      Instruction::SBC(target) => {
        let str = "SBC ".to_string();
        match target {
          ArithmeticTarget::A => str + "A,A",
          ArithmeticTarget::B => str + "A,B", 
          ArithmeticTarget::C => str + "A,C",
          ArithmeticTarget::D => str + "A,D", 
          ArithmeticTarget::E => str + "A,E", 
          ArithmeticTarget::H => str + "A,H", 
          ArithmeticTarget::L => str + "A,L", 
          ArithmeticTarget::HL => str + "A,(HL)", 
          ArithmeticTarget::D8 => str + "A,d8",
          _ => "Err".to_string()
        }
      },
      Instruction::AND(target) => {
        let str = "AND ".to_string();
        match target {
          ArithmeticTarget::A => str + "A",
          ArithmeticTarget::B => str + "B", 
          ArithmeticTarget::C => str + "C",
          ArithmeticTarget::D => str + "D", 
          ArithmeticTarget::E => str + "E", 
          ArithmeticTarget::H => str + "H", 
          ArithmeticTarget::L => str + "L", 
          ArithmeticTarget::HL => str + "(HL)", 
          ArithmeticTarget::D8 => str + "d8",
          _ => "Err".to_string()
        }
      },
      Instruction::OR(target) => {
        let str = "OR ".to_string();
        match target {
          ArithmeticTarget::A => str + "A",
          ArithmeticTarget::B => str + "B", 
          ArithmeticTarget::C => str + "C",
          ArithmeticTarget::D => str + "D", 
          ArithmeticTarget::E => str + "E", 
          ArithmeticTarget::H => str + "H", 
          ArithmeticTarget::L => str + "L", 
          ArithmeticTarget::HL => str + "(HL)", 
          ArithmeticTarget::D8 => str + "d8",
          _ => "Err".to_string()
        }
      },
      Instruction::XOR(target) => {
        let str = "XOR ".to_string();
        match target {
          ArithmeticTarget::A => str + "A",
          ArithmeticTarget::B => str + "B", 
          ArithmeticTarget::C => str + "C",
          ArithmeticTarget::D => str + "D", 
          ArithmeticTarget::E => str + "E", 
          ArithmeticTarget::H => str + "H", 
          ArithmeticTarget::L => str + "L", 
          ArithmeticTarget::HL => str + "(HL)", 
          ArithmeticTarget::D8 => str + "d8",
          _ => "Err".to_string()
        }
      },
      Instruction::CP(target) => {
        let str = "CP ".to_string();
        match target {
          ArithmeticTarget::A => str + "A",
          ArithmeticTarget::B => str + "B", 
          ArithmeticTarget::C => str + "C",
          ArithmeticTarget::D => str + "D", 
          ArithmeticTarget::E => str + "E", 
          ArithmeticTarget::H => str + "H", 
          ArithmeticTarget::L => str + "L", 
          ArithmeticTarget::HL => str + "(HL)", 
          ArithmeticTarget::D8 => str + "d8",
          _ => "Err".to_string()
        }
      },
      Instruction::ADDHL(target) => {
        let str = "ADD ".to_string();
        match target {
          ArithmeticTarget::BC => str + "HL,BC",
          ArithmeticTarget::DE => str + "HL,DE", 
          ArithmeticTarget::HL => str + "HL,HL",
          ArithmeticTarget::SP => str + "HL,SP", 
          _ => "Err".to_string()
        }
      },
      Instruction::INC(target) => {
        let str = "INC ".to_string();
        match target {
          IncDecTarget::A => str + "A",
          IncDecTarget::B => str + "B",
          IncDecTarget::C => str + "C",
          IncDecTarget::D => str + "D",
          IncDecTarget::E => str + "E",
          IncDecTarget::H => str + "H",
          IncDecTarget::L => str + "L",
          IncDecTarget::HLP => str + "(HL)",
          IncDecTarget::BC => str + "BC",
          IncDecTarget::DE => str + "DE",
          IncDecTarget::HL => str + "HL",
          IncDecTarget::SP => str + "SP"
        }
      },
      Instruction::DEC(target) => {
        let str = "DEC ".to_string();
        match target {
          IncDecTarget::A => str + "A",
          IncDecTarget::B => str + "B",
          IncDecTarget::C => str + "C",
          IncDecTarget::D => str + "D",
          IncDecTarget::E => str + "E",
          IncDecTarget::H => str + "H",
          IncDecTarget::L => str + "L",
          IncDecTarget::HLP => str + "(HL)",
          IncDecTarget::BC => str + "BC",
          IncDecTarget::DE => str + "DE",
          IncDecTarget::HL => str + "HL",
          IncDecTarget::SP => str + "SP"
        }
      },
      Instruction::RLC(target) => {
        let str = "RLC ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::RRC(target) => {
        let str = "RRC ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::RL(target) => {
        let str = "RL ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::RR(target) => {
        let str = "RR ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::SLA(target) => {
        let str = "SLA ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::SRA(target) => {
        let str = "SRA ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::SWAP(target) => {
        let str = "SWAP ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::SRL(target) => {
        let str = "SRL ".to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::JP(test, target) => {
        let str = "JP ".to_string();
        match target {
          JumpTarget::A16 => {
            match test {
              JumpTest::Always => str + "a16",
              JumpTest::Carry => str + "C,a16",
              JumpTest::NotCarry => str + "NC,a16",
              JumpTest::NotZero => str + "NZ,a16",
              JumpTest::Zero => str + "Z,a16"
            }
          },
          JumpTarget::HL => str + "(HL)"
        }
      },
      Instruction::JR(test) => {
        let str = "JR ".to_string();
        match test {
          JumpTest::Always => str + "r8",
          JumpTest::Carry => str + "C,r8",
          JumpTest::NotCarry => str + "NC,r8",
          JumpTest::NotZero => str + "NZ,r8",
          JumpTest::Zero => str + "Z,r8"
        }
      },
      Instruction::CALL(test) => {
        let str = "CALL ".to_string();
        match test {
          JumpTest::Always => str + "a16",
          JumpTest::Carry => str + "C,a16",
          JumpTest::NotCarry => str + "NC,a16",
          JumpTest::NotZero => str + "NZ,a16",
          JumpTest::Zero => str + "Z,a16"
        }
      },
      Instruction::RET(test) => {
        let str = "RET ".to_string();
        match test {
          JumpTest::Always => str,
          JumpTest::Carry => str + "C",
          JumpTest::NotCarry => str + "NC",
          JumpTest::NotZero => str + "NZ",
          JumpTest::Zero => str + "Z"
        }
      },
      Instruction::LD(load_type) => {
        let mut str = "LD ".to_string();
        match load_type {
          LoadType::Byte(target, source) => {
            match target {
              LoadByteTarget::A =>{str += "A,";},
              LoadByteTarget::A16 => {str += "(a16),";},
              LoadByteTarget::A8 => {str = "LDH (a8),".to_string();},
              LoadByteTarget::B => {str += "B,";},
              LoadByteTarget::C => {str += "C,";},
              LoadByteTarget::D => {str += "D,";},
              LoadByteTarget::E => {str += "E,";},
              LoadByteTarget::H => {str += "H,";},
              LoadByteTarget::L => {str += "L,";},
              LoadByteTarget::HLI => {str += "(HL+),";},
              LoadByteTarget::HLD => {str += "(HL-),";},
              LoadByteTarget::BC => {str += "(BC),";},
              LoadByteTarget::DE => {str += "(DE),";},
              LoadByteTarget::HL => {str += "HL,";},
              LoadByteTarget::SP => {str += "SP,";},
              LoadByteTarget::FF00C => {str += "(C),";},
            }; 
            match source {
              LoadByteSource::A =>{str += "A";},
              LoadByteSource::A16 => {str += "(a16)";},
              LoadByteSource::A8 => {str = "LDH A,(a8)".to_string();},
              LoadByteSource::B => {str += "B";},
              LoadByteSource::C => {str += "C";},
              LoadByteSource::D => {str += "D";},
              LoadByteSource::E => {str += "E";},
              LoadByteSource::H => {str += "H";},
              LoadByteSource::L => {str += "L";},
              LoadByteSource::HLI => {str += "(HL+)";},
              LoadByteSource::HLD => {str += "(HL-)";},
              LoadByteSource::BC => {str += "(BC)";},
              LoadByteSource::DE => {str += "(DE)";},
              LoadByteSource::HL => {str += "(HL)";},
              LoadByteSource::SP => {str += "SP";},
              LoadByteSource::FF00C => {str += "(C)";},
              LoadByteSource::D8 => {str += "d8";},
              LoadByteSource::D16 => {str += "d16";},           
            };
          } 
        };
        str  
      },
      Instruction::PUSH(target) => {
        let str = "PUSH ".to_string();
        match target {
          StackTarget::AF => str + "AF",
          StackTarget::BC => str + "BC",
          StackTarget::DE => str + "DE",
          StackTarget::HL => str + "HL"
        }
      },
      Instruction::POP(target) => {
        let str = "POP ".to_string();
        match target {
          StackTarget::AF => str + "AF",
          StackTarget::BC => str + "BC",
          StackTarget::DE => str + "DE",
          StackTarget::HL => str + "HL"
        }
      },
      Instruction::NOP() => "NOP".to_string(),
      Instruction::BIT(bit, target) => {
        let str = "BIT ".to_string() + &bit.to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::RES(bit, target) =>  {
        let str = "RES ".to_string() + &bit.to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::SET(bit, target) =>  {
        let str = "SET ".to_string() + &bit.to_string();
        match target {
          PrefixTarget::A => str + "A",
          PrefixTarget::B => str + "B",
          PrefixTarget::C => str + "C",
          PrefixTarget::D => str + "D",
          PrefixTarget::E => str + "E",
          PrefixTarget::H => str + "H",
          PrefixTarget::L => str + "L",
          PrefixTarget::HL => str + "(HL)",
        }
      },
      Instruction::RLCA() => "RLCA".to_string(),
      Instruction::RRCA() => "RRCA".to_string(),
      Instruction::RLA() => "RLA".to_string(),
      Instruction::RRA() => "RRA".to_string(),
      Instruction::DAA() => "DAA".to_string(),
      Instruction::CPL() => "CPL".to_string(),
      Instruction::SCF() => "SCF".to_string(),
      Instruction::CCF() => "CCF".to_string(),
      Instruction::RETI() => "RETI".to_string(),
      Instruction::HALT() => "HALT".to_string(),
      Instruction::STOP() => "STOP".to_string(),
      Instruction::DI() => "DI".to_string(),
      Instruction::EI() => "EI".to_string(),
      Instruction::PREFIX() => "PREFIX".to_string(),
      Instruction::RST(_) => "RST".to_string(),
  }
}