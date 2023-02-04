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

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
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
    /// decode the syntax found on obelisk.me.uk/6502
    pub fn from_reference_material(str: &str) -> AddressingMode {
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
    /// decode the syntax found on the nesdev wiki
    pub fn from_reference_short_version(str: &str) -> AddressingMode {
        let string: String = str.split_ascii_whitespace().collect();
        use AddressingMode::*;
        match string.as_str() {
            "#i" => Immediate,
            "(d,X)" => IndexedIndirect,
            "d" => ZeroPage,
            "a" => Absolute,
            "(d),Y" => IndirectIndexed,
            "d,Y" => ZeroPageY,
            "a,Y" => AbsoluteY,
            "a,X" => AbsoluteX,
            "d,X" => ZeroPageX,
            "" => Implicit,
            _ => unimplemented!(),
        }
    }
    pub const fn to_u8(&self) -> u8 {
        use AddressingMode::*;
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
    pub const fn from_u8(num: u8) -> AddressingMode {
        use AddressingMode::*;
        match num {
            0 => Implicit,
            1 => Accumulator,
            2 => Immediate,
            3 => ZeroPage,
            4 => ZeroPageX,
            5 => ZeroPageY,
            6 => Relative,
            7 => Absolute,
            8 => AbsoluteX,
            9 => AbsoluteY,
            10 => Indirect,
            11 => IndexedIndirect,
            12 => IndirectIndexed,
            _ => Implicit,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum StatusOption {
    #[default]
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
            // CyclesCost::PageDependant(n) => (*n as i8).wrapping_neg() as u8, // todo: do something here
            CyclesCost::PageDependant(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StatusFlags {
    pub carry: StatusOption,
    pub zero: StatusOption,
    pub interupt_disable: StatusOption,
    pub decimal: StatusOption,
    pub break_command: StatusOption,
    pub overflow: StatusOption,
    pub negative: StatusOption,
}

impl StatusFlags {
    pub fn new() -> StatusFlags {
        Default::default()
    }
}
