use crate::prelude::*;
pub struct Spy(&'static [(u16, &'static str)]);

impl Spy {
    pub fn new(inner: &'static [(u16, &'static str)]) -> Spy {
        Spy(inner)
    }
}

impl NesPeripheral for Spy {
    fn tick(&mut self, nes: &mut Nes) {
        let regs = nes.dump_registers();
        let pc = regs.get_pc();
        let mut line = String::new();
        if let Some((_, label)) = self.0.iter().find(|(address, _)| *address == pc) {
            line.push_str(&format!("{}:\n", label));
            // len += label.len() + 2;
            // println!("LABEL {} at {:x}", label, pc);
        }

        line.push_str(&format!("{:04X}  ", pc));
        let opcode = nes.peek_pc();
        let opcode_stats = &OPCODE_JUMPTABLE[opcode as usize];
        let bytes = opcode_stats.bytes as u16;
        let mode = opcode_stats.addressing;
        let mut dissassembled: [u8; 3] = [opcode, 0, 0];
        for byte in 1u16..bytes {
            let digit = nes.get(byte + pc);
            dissassembled[byte as usize] = digit;
        }
        for i in 0..3 {
            if i < bytes {
                line.push_str(&format!("{:02X} ", dissassembled[i as usize]));
            } else {
                line.push_str("   ");
            }
        }
        let (_, name) = opcode_names[opcode as usize];
        if name.len() == 3 {
            line.push_str(&format!(" {} ", name));
        } else {
            // the extra opcodes have a * in front of the name
            line.push_str(&format!("{} ", name));
        }

        let mut len = 4;
        match mode {
            0 => {
                // Implicit
                // line.push_str("imp ");
                len = 0;
            }
            1 => {
                // Accumulator
                line.push_str("A");
                len = 1;
            }
            2 => {
                // Immediate
                let value = nes.get(pc + 1);
                line.push_str(&format!("#${:02X}", value));
                len = 4;
            }
            3 => {
                // ZeroPage
                let address = nes.get(pc + 1);
                let value = nes.get(address as u16);
                line.push_str(&format!("${:02X} = {:02X}", address, value));
                len = 8;
            }
            4 => {
                // ZeroPageX
                // immediate value, resulant total, value at that address
                // "$33,X @ 33 = AA"
                let offset = nes.get(pc + 1);
                let total = offset.wrapping_add(regs.x);
                let value = nes.get(total as u16);
                line.push_str(&format!(
                    "${:02X},X @ {:02X} = {:02X}",
                    offset, total, value
                ));
                len = 15;
            }
            5 => {
                // ZeroPageY
                // line.push_str("zpy ");
                let offset = nes.get(pc + 1);
                let total = offset.wrapping_add(regs.y);
                let value = nes.get(total as u16);
                line.push_str(&format!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    offset, total, value
                ));
                len = 15;
            }
            6 => {
                // Relative
                let value = nes.get(pc + 1) as i8; // adding one since pc hasn't been incremented past opcode at this point
                let destination = ((regs.pc as i32 + 2).wrapping_add(value as i32) & 0xffff) as u16;
                line.push_str(&format!("${:04X}", destination));
                len = 5;
            }
            7 => {
                // Absolute
                // todo: this is a real mess. find a better way to discriminate between different opcodes that load and store memory values
                let value = nes.get_short(pc + 1); // adding one since pc hasn't been incremented past opcode at this point
                let dereferenced = nes.get(value);
                if opcode == 0x32 || opcode == 0x4C || opcode == 0x20 {
                    // jsr doesn't print value at address
                    line.push_str(&format!("${:04X}", value));
                    len = 5;
                } else {
                    line.push_str(&format!("${:04X} = {:02X}", value, dereferenced));
                    len = 10;
                }
            }
            8 => {
                // AbsoluteX
                // $0633,X @ 0633 = AA
                let value = nes.get_short(pc + 1); // adding one since pc hasn't been incremented past opcode at this point
                let total = value.wrapping_add(nes.cpu.registers.x as u16);
                let dereferenced = nes.get(total);
                line.push_str(&format!(
                    "${:04X},X @ {:04X} = {:02X}",
                    value, total, dereferenced
                ));
                len = 19;
            }
            9 => {
                // AbsoluteY
                let value = nes.get_short(pc + 1); // adding one since pc hasn't been incremented past opcode at this point
                let total = value.wrapping_add(nes.cpu.registers.y as u16);
                let dereferenced = nes.get(total);
                use OpGroup::*;
                // todo: fix this mess
                match opcode_group(opcode) {
                    Control => {
                        if opcode & 0xf0 == 0xa0 {
                            line.push_str(&format!(
                                "${:04X},Y @ {:04X} = {:02X}",
                                value, total, dereferenced
                            ));
                            len = 19;
                        } else {
                            if opcode & 0x0f == 0x0c
                                && (opcode & 0xf0 == 0x80
                                    || opcode & 0xf0 == 0x20
                                    || opcode & 0xf0 == 0xe0
                                    || opcode & 0xf0 == 0xc0)
                            {
                                line.push_str(&format!(
                                    "${:04X},Y @ {:04X} = {:02X}",
                                    value, total, dereferenced
                                ));
                                len = 19;
                            } else {
                                line.push_str(&format!("${:04X},Y", value));
                                len = 14;
                            }
                        }
                    }
                    Alu => {
                        line.push_str(&format!(
                            "${:04X},Y @ {:04X} = {:02X}",
                            value, total, dereferenced
                        ));
                        len = 19;
                    }
                    Rmw => {
                        line.push_str(&format!(
                            "${:04X},Y @ {:04X} = {:02X}",
                            value, total, dereferenced
                        ));
                        len = 19;
                    }
                    Unoff => {
                        line.push_str(&format!(
                            "${:04X},Y @ {:04X} = {:02X}",
                            value, total, dereferenced
                        ));
                        len = 19;
                    }
                }
            }
            10 => {
                // Indirect

                let address = nes.get_short(nes.cpu.registers.pc + 1);

                let address_lo = (address & 0xff) as u8;
                let address_hi = address & 0xff00;
                let lo = nes.get(address) as u16;
                let hi_address = address_lo.wrapping_add(1) as u16 | address_hi;
                let hi = nes.get(hi_address) as u16;
                let deref = (hi << 8) | lo;

                line.push_str(&format!("(${:04X}) = {:04X}", address, deref));
                len = 14;
            }
            11 => {
                // IndexedIndirect (indirect x)
                // "($80,X) @ 80 = 0200 = 5A"
                let table = nes.get(pc + 1);
                let base = table.wrapping_add(regs.x);
                // let value = nes.get(address as u16);
                let lo = nes.get(base as u16) as u16;
                let hi = nes.get(base.wrapping_add(1) as u16) as u16;
                let address = hi << 8 | lo;
                let value = nes.get(address);
                line.push_str(&format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    table, base, address, value
                ));
                len = 24;
            }
            12 => {
                // IndirectIndexed (indirect y)
                // immediate, short value in zp, total plus y, value at that address
                // "($33),Y = 0400 @ 0400 = 7F"
                // let address = nes.get(pc+1);
                // let value = nes.get(address);
                let immediate = nes.get(pc + 1);
                let lo = nes.get(immediate as u16) as u16;
                let hi = nes.get(immediate.wrapping_add(1) as u16) as u16; // wraps around zero page
                let short = hi << 8 | lo;
                let address = short.wrapping_add(regs.y as u16);
                let value = nes.get(address);

                line.push_str(&format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    immediate, short, address, value
                ));
                len = 26;
            }
            _ => {
                unimplemented!()
            }
        }
        // if let Some((_, label)) = LABEL_LIST.iter().find(|(address, _)| *address == pc) {
        //     line.push_str(&format!("({})", label));
        //     len += label.len() + 2;
        //     // println!("LABEL {} at {:x}", label, pc);
        // }
        if len > 28 {
            // panic!("what?");
        } else {
            let spaces = 28 - len;
            for _ in 0..spaces {
                line.push(' ');
            }
        }
        line.push_str(&format!("A:{:02X} ", regs.a));
        line.push_str(&format!("X:{:02X} ", regs.x));
        line.push_str(&format!("Y:{:02X} ", regs.y));
        line.push_str(&format!("P:{:02X} ", regs.p));
        line.push_str(&format!("SP:{:02X} ", regs.sp));

        let ppu_cycle = 10_055;
        {
            let ppu_thousands = ppu_cycle / 1000;
            let ppu_hundreds = ppu_cycle % 1000;
            line.push_str("PPU:");
            line.push_str(&format!("{:>3},{:>3} ", ppu_thousands, ppu_hundreds));
        }
        let cpu_cycle = nes.cpu.cycles;
        line.push_str(&format!("CYC:{}", cpu_cycle));

        let stack = nes.dump_stack();
        // let pc = regs.get_pc();
        // print!("{:2x} ## {:?} ", next_opcode, regs);

        println!("{}", line);
        // println!("{}", stack);
    }
}

enum OpGroup {
    Control,
    Alu,
    Rmw,
    Unoff,
}

fn opcode_group(opcode: u8) -> OpGroup {
    // println!("decode op: {:08b}", opcode);
    let low = opcode & 0b11;
    match (low & 0b10 == 0b10, low & 0b01 == 0b01) {
        (false, false) => OpGroup::Control,
        (false, true) => OpGroup::Alu,
        (true, false) => OpGroup::Rmw,
        (true, true) => OpGroup::Unoff,
    }
}
