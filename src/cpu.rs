use crate::instruction::*;
use crate::register;
use crate::timer;



struct CPU{
    registers:register::Registers,
    program_counter:u16,
    stack_pointer:u16,
    clock:timer::TimerContext,
    bus:MemoryBus,
    is_halted:bool,
    ime:bool,
    ie:u8,
    if_reg:u8,
}
struct MemoryBus{
    mem: [u8;0xFFFF]
}

impl MemoryBus {
  fn read_byte(&self, address: u16) -> u8 {
    self.mem[address as usize]
  }
  fn write_byte(&self, address : u16, byte:u8){

  }
}

enum EmulatorError {
  InvalidOpcode,
  MemoryReadError,
  MemoryWriteError,
  StackOverflow,
  StackUnderflow,
  InvalidAddress,
  InterruptHandlingError,
  // Add more error variants as needed
}

impl CPU {
  fn read_next_byte(&self) -> u8 {
    self.bus.read_byte(self.program_counter).wrapping_add(1)
  }

  fn read_next_word(&self) -> u16{
    self.program_counter.wrapping_add(1)
  }

  fn step(&mut self) {
    let mut instruction_byte = self.bus.read_byte(self.program_counter);
    
    let prefixed = instruction_byte == 0xCB;
    if prefixed {
      instruction_byte = self.bus.read_byte(self.program_counter + 1);
      self.clock.timer_tick(4);
    }

    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte,prefixed) {
      self.execute(instruction)
    } else {
      panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
    };
    self.program_counter = next_pc;
  }


  fn execute(&mut self, instruction: Instruction) ->u16{
      match instruction {
        Instruction::ADD(target) => {
          match target {
            ArithmeticTarget::B => {
              let value = self.registers.b;
              let new_value = self.add(value);
              self.registers.a = new_value;
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::C => {
              let value = self.registers.c;
              let new_value = self.add(value);
              self.registers.a = new_value;
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D => {
              let value = self.registers.d;
              let new_value = self.add(value);
              self.registers.a = new_value;
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::E => {
              let value = self.registers.e;
              let new_value = self.add(value);
              self.registers.a = new_value;
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::H => {
              let value = self.registers.c;
              let new_value = self.add(value);
              self.registers.a = new_value;
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::L => {
              let value = self.registers.l;
              let new_value = self.add(value);
              self.registers.a = new_value;
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::HL => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            }
            ArithmeticTarget::A => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::SP => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            _ => {self.program_counter}
          }
        },
        Instruction::SUB(target) => {
          match target {
            ArithmeticTarget::A => {
              let value = self.registers.a;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::B => {
              let value = self.registers.b;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::C => {
              let value = self.registers.c;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D => {
              let value = self.registers.d;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::E => {
              let value = self.registers.e;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::H => {
              let value = self.registers.h;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::L => {
              let value = self.registers.l;
              let (new_value, did_overflow) = self.sub(value);
              self.registers.a = new_value;
              self.update_flags(new_value, did_overflow);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::HL => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(8);
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
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::B => {
              let value = self.registers.b;
              self.and(value);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::C => {
              let value = self.registers.c;
              self.and(value);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D => {
              let value = self.registers.d;
              self.and(value);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::E => {
              let value = self.registers.e;
              self.and(value);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::H => {
              let value = self.registers.h;
              self.and(value);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::L => {
              let value = self.registers.l;
              self.and(value);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::HL => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            }
            _=>{self.program_counter}
          }
        },    
        Instruction::SBC(target) => {
          match target {
            ArithmeticTarget::A => {
              self.sbc(&mut self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::B => {
              self.sbc(&mut self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::C => {
              self.sbc(&mut self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::D => {
              self.sbc(&mut self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::E => {
              self.sbc(&mut self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::H => {
              self.sbc(&mut self.registers.h);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::L => {
              self.sbc(&mut self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);
                self.sbc_hl(value);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            _=>{self.program_counter.wrapping_add(1)}
          }
        },
        Instruction::OR(target) => {
          match target {
            ArithmeticTarget::A => {
              self.or(&mut self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::B => {
              self.or(&mut self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::C => {
              self.or(&mut self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::D => {
              self.or(&mut self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::E => {
              self.or(&mut self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::H => {
              self.or(&mut self.registers.h);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::L => {
              self.or(&mut self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);
                self.or_hl(value);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(2)
            },
            _=>{self.program_counter.wrapping_add(1)}
          } 
        },    
        Instruction::XOR(target) => {
          match target {
            ArithmeticTarget::A => {
              self.xor(&mut self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::B => {
              self.xor(&mut self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::C => {
              self.xor(&mut self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::D => {
              self.xor(&mut self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::E => {
              self.xor(&mut self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::H => {
              self.xor(&mut self.registers.h);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::L => {
              self.xor(&mut self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);
                self.xor_hl(value);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            _=>{self.program_counter.wrapping_add(1)}
          }
        }, 
        Instruction::CP(target) => {
          match target {
            ArithmeticTarget::A => {
              self.cp(&self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::B => {
              self.cp(&self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::C => {
              self.cp(&self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::D => {
              self.cp(&self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::E => {
              self.cp(&self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::H => {
              self.cp(&self.registers.h);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::L => {
              self.cp(&self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let value = self.bus.read_byte(address);
                self.cp(&value);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(1)
            }
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            _=>{self.program_counter.wrapping_add(1)}
          }
        }, 
        Instruction::INC(target) => {
          match target {
            IncDecTarget::A => {
              self.inc(&mut self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::B => {
              self.inc(&mut self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::C => {
              self.inc(&mut self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::D => {
              self.inc(&mut self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::E => {
              self.inc(&mut self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::H => {
              self.inc(&mut self.registers.h);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::L => {
              self.inc(&mut self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let mut value = self.bus.read_byte(address);
                self.inc(&mut value);
                // Write the modified value back to memory
                self.bus.write_byte(address, value);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(1)
            }
            IncDecTarget::BC =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            IncDecTarget::DE =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            IncDecTarget::SP =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            _=>{self.program_counter.wrapping_add(1)}
          }
        },
        Instruction::DEC(target) => {
          match target {
            IncDecTarget::A => {
              self.dec(&mut self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::B => {
              self.dec(&mut self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::C => {
              self.dec(&mut self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::D => {
              self.dec(&mut self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::E => {
              self.dec(&mut self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::H => {
              self.dec(&mut self.registers.h);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::L => {
              self.dec(&mut self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            IncDecTarget::HL => {
                // Read the value from memory at the address pointed to by HL
                let address = self.registers.get_hl();
                let mut value = self.bus.read_byte(address);
                self.dec(&mut value);
                // Write the modified value back to memory
                self.bus.write_byte(address, value);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(1)
            }
            IncDecTarget::BC =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            IncDecTarget::DE =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            IncDecTarget::SP =>{
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(1)
            }
            _ =>{self.program_counter.wrapping_add(1)}
          }
        }, 
        Instruction::CCF() => {
          self.ccf();
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        },
        Instruction::SCF() => {
          self.scf();
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        },
        Instruction::RRA() => {
          self.rra();
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        },
        Instruction::RLA() => {
          self.rla();
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        },
        Instruction::RRCA() => {
          self.rrca();
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        },
        Instruction::RLCA() => {
          self.rlca();
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        }, 
        Instruction::ADC(target) => {
          match target {
            ArithmeticTarget::A => {
              self.adc(self.registers.a);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::B => {
              self.adc(self.registers.b);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::C => {
              self.adc(self.registers.c);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::D => {
              self.adc(self.registers.d);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::E => {
              self.adc(self.registers.e);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::H => {
              self.adc(self.registers.h);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::L => {
              self.adc(self.registers.l);
              self.clock.timer_tick(4);
              self.program_counter.wrapping_add(1)
            },
            ArithmeticTarget::HL => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },
            ArithmeticTarget::D8 => {
              //TODO
              self.clock.timer_tick(8);
              self.program_counter.wrapping_add(2)
            },

            _ =>{self.program_counter.wrapping_add(1)}
          }
        },
        Instruction::CPL() => {
          // Perform the complement operation on register A
          self.registers.a = !self.registers.a;

          // Update flags
          self.registers.f.subtract = true;
          self.registers.f.half_carry = true;
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        },
        Instruction::BIT(bit, target) => {
          match target {
              PrefixTarget::A => {
                self.bit(bit, self.registers.a);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                self.bit(bit, self.registers.b);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                self.bit(bit, self.registers.c);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                self.bit(bit, self.registers.d);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                self.bit(bit, self.registers.e);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                self.bit(bit, self.registers.h);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                self.bit(bit, self.registers.l);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                  // Read the value from memory at the address pointed to by HL
                  let address = self.registers.get_hl();
                  let value = self.bus.read_byte(address);
                  self.bit(bit, value);
                  self.clock.timer_tick(16);
                  self.program_counter.wrapping_add(2)
              },
              _=>{self.program_counter.wrapping_add(1)}
          }
        }, 
        Instruction::RES(bit, target) => {
          match target {
              PrefixTarget::A => {
                self.res(bit, &mut self.registers.a);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                self.res(bit, &mut self.registers.b);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                self.res(bit, &mut self.registers.c);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                self.res(bit, &mut self.registers.d);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                self.res(bit, &mut self.registers.e);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                self.res(bit, &mut self.registers.h);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                self.res(bit, &mut self.registers.l);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                  // Read the value from memory at the address pointed to by HL
                  let address = self.registers.get_hl();
                  let mut value = self.bus.read_byte(address);
                  self.res(bit, &mut value);
                  // Write the modified value back to memory
                  self.bus.write_byte(address, value);
                  self.clock.timer_tick(16);
                  self.program_counter.wrapping_add(2)
              }
          }
        },
        Instruction::SET(bit, target) => {
          match target {
              PrefixTarget::A => {
                self.set(bit, &mut self.registers.a);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::B => {
                self.set(bit, &mut self.registers.b);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::C => {
                self.set(bit, &mut self.registers.c);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::D => {
                self.set(bit, &mut self.registers.d);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::E => {
                self.set(bit, &mut self.registers.e);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::H => {
                self.set(bit, &mut self.registers.h);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::L => {
                self.set(bit, &mut self.registers.l);
                self.clock.timer_tick(8);
                self.program_counter.wrapping_add(2)
              },
              PrefixTarget::HL => {
                  // Read the value from memory at the address pointed to by HL
                  let address = self.registers.get_hl();
                  let mut value = self.bus.read_byte(address);
                  self.set(bit, &mut value);
                  // Write the modified value back to memory
                  self.bus.write_byte(address, value);
                  self.clock.timer_tick(16);
                  self.program_counter.wrapping_add(2)
              }
          }
        },
        Instruction::SRL(target) => {
            match target {
                PrefixTarget::A => {
                  self.srl(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.srl(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.srl(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.srl(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.srl(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.srl(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.srl(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.srl(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::RR(target) => {
            match target {
                PrefixTarget::A => {
                  self.rr(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.rr(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.rr(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.rr(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.rr(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.rr(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.rr(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.rr(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::RL(target) => {
            match target {
                PrefixTarget::A => {
                  self.rl(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.rl(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.rl(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.rl(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.rl(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.rl(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.rl(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.rl(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(16);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::RRC(target) => {
            match target {
                PrefixTarget::A => {
                  self.rrc(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.rrc(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.rrc(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.rrc(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.rrc(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.rrc(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.rrc(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.rrc(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(8);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::RLC(target) => {
            match target {
                PrefixTarget::A => {
                  self.rlc(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.rlc(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.rlc(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.rlc(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.rlc(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.rlc(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.rlc(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.rlc(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(16);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::SRA(target) => {
            match target {
                PrefixTarget::A => {
                  self.sra(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.sra(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.sra(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.sra(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.sra(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.sra(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.sra(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.sra(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(16);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::SLA(target) => {
            match target {
                PrefixTarget::A => {
                  self.sla(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.sla(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.sla(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.sla(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.sla(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.sla(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.sla(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.sla(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(16);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        Instruction::SWAP(target) => {
            match target {
                PrefixTarget::A => {
                  self.swap(&mut self.registers.a);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::B => {
                  self.swap(&mut self.registers.b);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::C => {
                  self.swap(&mut self.registers.c);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::D => {
                  self.swap(&mut self.registers.d);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::E => {
                  self.swap(&mut self.registers.e);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::H => {
                  self.swap(&mut self.registers.h);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::L => {
                  self.swap(&mut self.registers.l);
                  self.clock.timer_tick(8);
                  self.program_counter.wrapping_add(2)
                },
                PrefixTarget::HL => {
                    // Read the value from memory at the address pointed to by HL
                    let address = self.registers.get_hl();
                    let mut value = self.bus.read_byte(address);
                    self.swap(&mut value);
                    // Write the modified value back to memory
                    self.bus.write_byte(address, value);
                    self.clock.timer_tick(16);
                    self.program_counter.wrapping_add(2)
                }
            }
        },
        //TODO rest of instr
        Instruction::JP(test,target) => {
          let jump_condition = match test {
              JumpTest::NotZero => !self.registers.f.zero,
              JumpTest::NotCarry => !self.registers.f.carry,
              JumpTest::Zero => self.registers.f.zero,
              JumpTest::Carry => self.registers.f.carry,
              JumpTest::Always => true
          };
          if jump_condition {
            self.clock.timer_tick(16);
          }else {
            self.clock.timer_tick(12);
          }
          match target{
            JumpTarget::A16 => {
              self.jump(jump_condition,);
              self.program_counter.wrapping_add(3)
            },
            JumpTarget::HL =>{
              self.jump(jump_condition,);
              self.program_counter.wrapping_add(1)
            },
          }  
        }
        Instruction::LD(load_type) => {
          match load_type {
            LoadType::Byte(target, source) => {
              let source_value = match source {
                LoadByteSource::BC =>{
                  match target{
                    LoadByteTarget::A=>,
                    LoadByteTarget::D16=>,
                    _=>{}
                  }
                },
                LoadByteSource::DE =>{
                  match target{
                    LoadByteTarget::A=>,
                    LoadByteTarget::D16=>,
                    _=>{}
                  }
                },
                LoadByteSource::HLI =>{
                  match target{
                    LoadByteTarget::A=>,
                    LoadByteTarget::D16=>,
                    _=>{}
                  }
                },
                LoadByteSource::HLD =>{
                  match target{
                    LoadByteTarget::A=>,
                    LoadByteTarget::D16=>,
                    _=>{}
                  }
                },
                LoadByteSource::A => {
                  match target{
                    LoadByteTarget::BC =>,
                    LoadByteTarget::DE =>,
                    LoadByteTarget::HLI =>,
                    LoadByteTarget::HLD =>,
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::D8 =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                },
                LoadByteSource::B => {
                  match target{
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                },
                LoadByteSource::C => {
                  match target{
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                },
                LoadByteSource::D => {
                  match target{
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                },
                LoadByteSource::E => {
                  match target{
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                },
                LoadByteSource::H=> {
                  match target{
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                },
                LoadByteSource::L => {
                  match target{
                    LoadByteTarget::B =>,
                    LoadByteTarget::C =>,
                    LoadByteTarget::D =>,
                    LoadByteTarget::E =>,
                    LoadByteTarget::H =>,
                    LoadByteTarget::L =>,
                    LoadByteTarget::HL =>,
                    LoadByteTarget::A =>,
                    _ => {}
                  }
                }  
                LoadByteSource::D8 => self.read_next_byte(),
                LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                _ => { panic!("TODO: implement other sources") }
              };
              match target {
                LoadByteTarget::A => self.registers.a = source_value,
                LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                _ => { panic!("TODO: implement other targets") }
              };
              match source {
                LoadByteSource::D8  => self.program_counter.wrapping_add(2),
                _                   => self.program_counter.wrapping_add(1),
              }
            }
            _ => { panic!("TODO: implement other load types") }
          }
        }
        Instruction::PUSH(target) => {
            let value = match target {
              StackTarget::BC => self.registers.get_bc(),
              StackTarget::DE => self.registers.get_de(),
              StackTarget::HL => self.registers.get_hl(),
              StackTarget::AF => self.registers.get_af(),
              _ => { panic!("Err: ") }
            };
            self.push(value);
            self.clock.timer_tick(16);
            self.program_counter.wrapping_add(1)
        }
        Instruction::POP(target) => {
            let result = self.pop();
            match target {
                StackTarget::BC => self.registers.set_bc(result),
                StackTarget::DE => self.registers.set_de(result),
                StackTarget::HL => self.registers.set_hl(result),
                StackTarget::AF => self.registers.set_af(result),
                _ => { panic!("Err:") }
            };
            self.clock.timer_tick(16);
            self.program_counter.wrapping_add(1)
        }
        Instruction::CALL(test) => {
            let jump_condition = match test {
                JumpTest::NotZero => !self.registers.f.zero,
                _ => { panic!("TODO: support more conditions") }
            };
            if jump_condition {
              self.clock.timer_tick(24);
            }else{
              self.clock.timer_tick(12);
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
              _ => { panic!("Err: ") }
            };
            if jump_condition {
              self.clock.timer_tick(20);
            }else {
              self.clock.timer_tick(8); 
            }
            self.return_(jump_condition)
        }
        Instruction::NOP() => {
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        } 
        Instruction::HALT() => {
          self.is_halted = true;
          self.clock.timer_tick(4);
          self.program_counter.wrapping_add(1)
        }
        Instruction::RETI() => {
          self.return_(true);
          self.ime = true;
          self.clock.timer_tick(16);
          self.program_counter.overflowing_add(1);

        }
        Instruction::EI() => {
          self.ime = true;
          self.clock.timer_tick(4);
          self.program_counter.overflowing_add(1);
        }
        Instruction:DI() => {
          self.ime = false;
          self.clock.timer_tick(4);
          self.program_counter.overflowing_add(1);
        }
        Instruction::PREFIX() => {
          //Should never be read
        }

    }
  fn push(&mut self, val:u16){
    self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    self.bus.write_byte(self.stack_pointer, ((val & 0xFF00) >> 8) as u8);
    self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    self.bus.write_byte(self.stack_pointer, (val & 0xFF) as u8);
        
  }

  fn pop(&mut self) -> u16 {
    let lsb = self.bus.read_byte(self.stack_pointer) as u16;
    self.stack_pointer = self.stack_pointer.wrapping_add(1);
    
    let msb = self.bus.read_byte(self.stack_pointer) as u16;
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

 
  fn sub(&mut self, value: u8) -> (u8, bool) {
    let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
    (new_value, did_overflow)
  }

  fn and(&mut self, value: u8) {
    self.registers.a &= value;
    self.update_flags(self.registers.a);
  }

  fn sbc(&mut self, value: &mut u8) {
    let carry = if self.registers.f.carry { 1 } else { 0 };
    let (result, did_overflow) = self.registers.a.overflowing_sub(*value);
    let (result, did_overflow2) = result.overflowing_sub(carry);
    
    self.registers.a = result;
    self.registers.f.zero = self.registers.a == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) + carry > 0x0F;
    self.registers.f.carry = did_overflow || did_overflow2;
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

  fn or(&mut self, value: &mut u8) {
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

  fn xor(&mut self, value: &mut u8) {
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

  fn inc(&mut self, value: &mut u8) {
    *value = value.wrapping_add(1);
    self.registers.f.zero = *value == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = (*value & 0x0F) == 0;
    // Carry flag remains unchanged
  }

  fn dec(&mut self, value: &mut u8) {
    *value = value.wrapping_sub(1);
    self.registers.f.zero = *value == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (*value & 0x0F) == 0x0F;
    // Carry flag remains unchanged
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
    let sum_without_carry = self.registers.a + value;

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

  fn res(&mut self, bit: u8, value: &mut u8) {
    // Reset (clear) the specified bit in the value
    let mask = !(1 << bit);
    *value &= mask;
  }

  fn set(&mut self, bit: u8, value: &mut u8) {
    // Set (make 1) the specified bit in the value
    let mask = 1 << bit;
    *value |= mask;
  }

  fn srl(&mut self, value: &mut u8) {
      // Perform a logical right shift on the value
      let carry = *value & 0x01;
      *value >>= 1;
      
      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn rr(&mut self, value: &mut u8) {
      // Calculate the carry bit before the rotation
      let carry = self.registers.f.carry as u8;

      // Perform a right rotation on the value with the carry bit
      *value = (*value >> 1) | (carry << 7);

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = (*value & 0x01) != 0;
  }

  fn rl(&mut self, value: &mut u8) {
      // Calculate the carry bit before the rotation
      let carry = ((*value & 0x80) != 0) as u8;

      // Perform a left rotation on the value with the carry bit
      *value = (*value << 1) | self.registers.f.carry as u8;

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn rrc(&mut self, value: &mut u8) {
      // Calculate the carry bit before the rotation
      let carry = *value & 0x01;

      // Perform a right rotation through the carry bit on the value
      *value = (*value >> 1) | (carry << 7);

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn rlc(&mut self, value: &mut u8) {
      // Calculate the carry bit before the rotation
      let carry = (*value & 0x80) >> 7;

      // Perform a left rotation through the carry bit on the value
      *value = (*value << 1) | carry;

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn sra(&mut self, value: &mut u8) {
      // Calculate the carry bit before the shift
      let carry = *value & 0x01;

      // Perform an arithmetic right shift on the value
      *value = (*value >> 1) | (*value & 0x80);

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn sla(&mut self, value: &mut u8) {
      // Calculate the carry bit before the shift
      let carry = (*value & 0x80) >> 7;

      // Perform an arithmetic left shift on the value
      *value <<= 1;

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = carry != 0;
  }

  fn swap(&mut self, value: &mut u8) {
      // Perform the swap operation by exchanging the upper and lower nibbles
      *value = (*value << 4) | (*value >> 4);

      // Update flags
      self.registers.f.zero = *value == 0;
      self.registers.f.subtract = false;
      self.registers.f.half_carry = false;
      self.registers.f.carry = false; // Carry flag is always reset
  }


  fn jump(&self, should_jump: bool) -> u16 {
    if should_jump {
      let least_significant_byte = self.bus.read_byte(self.program_counter + 1) as u16;
      let most_significant_byte = self.bus.read_byte(self.program_counter + 2) as u16;
        (most_significant_byte << 8) | least_significant_byte
      } else {
        self.program_counter.wrapping_add(3)
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

    // For all instructions
  fn update_flags(&mut self, result: u8, did_overflow: bool) {
    // Update flag register as needed.
    // For example, setting or clearing the carry and overflow flags.
    // The specific flag logic depends on your emulation requirements.
    // Here's a simple example of setting or clearing the carry flag:
    if did_overflow {
        self.registers.f |= 0b0001; // Set the carry flag
    } else {
        self.registers.f &= !0b0001; // Clear the carry flag
    }
  }

  fn handle_interrupts(&mut self) -> Result<(), EmulatorError> {
    if self.ime {
        let interrupt_flags = self.ie & self.if_reg;
        if interrupt_flags != 0 {
            self.ime = false; // Disable further interrupts

            if interrupt_flags & 0b00001 != 0 {
                self.handle_interrupt(0x0040)?; // V-Blank interrupt
            } else if interrupt_flags & 0b00010 != 0 {
                self.handle_interrupt(0x0048)?; // LCD STAT interrupt
            } else if interrupt_flags & 0b00100 != 0 {
                self.handle_interrupt(0x0050)?; // Timer interrupt
            } else if interrupt_flags & 0b01000 != 0 {
                self.handle_interrupt(0x0058)?; // Serial interrupt
            } else if interrupt_flags & 0b10000 != 0 {
                self.handle_interrupt(0x0060)?; // Joypad interrupt
            }
        }
    }
    Ok(())
}

fn handle_interrupt(&mut self, addr: u16) -> Result<(), EmulatorError> {
    // Push the return address onto the stack
    let pc = self.program_counter;
    self.push_stack(pc)?;

    // Disable further interrupts while servicing the current one
    self.ime = false;

    // Jump to the interrupt handler
    self.program_counter = addr;

    Ok(())
  }
}
