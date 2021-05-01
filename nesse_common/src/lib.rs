pub struct CompactOpcode {
    pub addressing: u8, // enum representing the addressing mode
    //status: u8, // flags represent if this opcode reads or writes to each status flag
    pub byte_cycles: u8, // upper bit indicates if page affects cycle count (0 or 1). next 3 highest bits indicate cycle cost (1,2,3,4). lowest 2 bits indicate bytes consumed by instruction. (1,2, or 3).
    pub family: u8,      // which family of instructions does this belong to
                         // maybe we can use this to branch on to different functions with the above as parameters
}

impl CompactOpcode {
    /// extract all values out of opcde bytecycles etc
    pub fn open(self) -> (u8, u8, u8) {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct NesMetaOpcode {
    pub name: String,
    pub description: String,
    pub status: StatusFlags,
}

#[derive(Debug)]
pub struct NesOpcode {
    pub meta: NesMetaOpcode,
    pub addressing: AddressingMode,
    pub opcode: u8,
    pub bytes: u8,
    pub cycles: CyclesCost,
}

impl NesOpcode {
    pub fn compact(self) -> CompactOpcode {
        let byte_cycles = {
            let upper_bits = match self.cycles {
                CyclesCost::Always(n) => n & 7 << 4,
                CyclesCost::PageDependant(n) => (n & 7 << 4) | 128,
            };
            let lower_bits = self.bytes & 0b11;
            lower_bits | upper_bits
        };
        let addressing = self.addressing.to_u8();
        let family = 0; // todo?
        CompactOpcode {
            addressing,
            byte_cycles,
            family,
        }
    }
}

#[derive(Debug)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

impl AddressingMode {
    pub fn from_str(str: &str) -> AddressingMode {
        use AddressingMode::*;
        let st = str.to_string();
        let stripped: String = st.split_whitespace().collect();
        match stripped.as_str() {
            "Implied" => Implicit,
            "Accumulator" => Accumulator,
            "Immediate" => Immediate,
            "ZeroPage" => ZeroPage,
            "ZeroPage,X" => ZeroPageX,
            "ZeroPage,Y" => ZeroPageY,
            "Relative" => Relative,
            "Absolute" => Absolute,
            "Absolute,X" => AbsoluteX,
            "Absolute,Y" => AbsoluteY,
            "Indirect" => Indirect,
            "(Indirect,X)" => IndexedIndirect,
            "(Indirect),Y" => IndirectIndexed,
            _ => panic!("not found {}", str),
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            Implicit => 0,
            Accumulator => 1,
            Immediate => 2,
            ZeroPage => 3,
            ZeroPageX => 4,
            ZeroPageY => 5,
            Relative => 6,
            Absolute => 7,
            AbsoluteX => 8,
            AbsoluteY => 9,
            Indirect => 10,
            IndexedIndirect => 11,
            IndirectIndexed => 12,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatusOption {
    Conditional,
    NotAffected,
}
#[derive(Debug, Clone)]
pub enum CyclesCost {
    Always(u8),
    PageDependant(u8),
}

impl CyclesCost {
    pub fn to_u8(&self) -> u8 {
        match self {
            CyclesCost::Always(n) => *n,
            CyclesCost::PageDependant(n) => (*n as i8).wrapping_neg() as u8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusFlags {
    pub carry: StatusOption,
    pub zero: StatusOption,
    pub interupt_disable: StatusOption,
    pub decimal: StatusOption,
    pub break_command: StatusOption,
    pub overflow: StatusOption,
    pub negative: StatusOption,
}
