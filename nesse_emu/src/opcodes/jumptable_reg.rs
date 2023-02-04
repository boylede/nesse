use crate::opcodes::*;
use crate::Nes;
pub type OpcodeFn = fn(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8;

pub struct Opcode {
    pub exec: OpcodeFn,
    pub addressing: u8,
    pub cycles: u8,
    pub bytes: u8,
}

#[test]
pub fn check_jumptable_entry_size() {
    let entry_size = std::mem::size_of::<Opcode>();
    assert!(entry_size == std::mem::align_of::<usize>() * 2);
    assert!(std::mem::size_of_val(&OPCODE_JUMPTABLE) == entry_size * 256);
}
impl Opcode {
    #[inline(always)]
    pub fn run(&self, nes: &mut Nes) -> u8 {
        (self.exec)(nes, self.addressing, self.cycles, self.bytes)
    }
}
pub const OPCODE_JUMPTABLE: [Opcode; 256] = [
    Opcode {
        exec: brk,
        addressing: 0u8,
        cycles: 7u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: asl,
        addressing: 3u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: php,
        addressing: 0u8,
        cycles: 3u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: asl,
        addressing: 1u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: asl,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bpl,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ora,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: asl,
        addressing: 4u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: clc,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ora,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: asl,
        addressing: 8u8,
        cycles: 7u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: jsr,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: and,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bit,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: and,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: rol,
        addressing: 3u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: plp,
        addressing: 0u8,
        cycles: 4u8,
        bytes: 1u8,
    },
    Opcode {
        exec: and,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: rol,
        addressing: 1u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bit,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: and,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: rol,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bmi,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: and,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: and,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: rol,
        addressing: 4u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sec,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: and,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: and,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: rol,
        addressing: 8u8,
        cycles: 7u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: rti,
        addressing: 0u8,
        cycles: 6u8,
        bytes: 1u8,
    },
    Opcode {
        exec: eor,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: eor,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lsr,
        addressing: 3u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: pha,
        addressing: 0u8,
        cycles: 3u8,
        bytes: 1u8,
    },
    Opcode {
        exec: eor,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lsr,
        addressing: 1u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: jmp,
        addressing: 7u8,
        cycles: 3u8,
        bytes: 3u8,
    },
    Opcode {
        exec: eor,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: lsr,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bvc,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: eor,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: eor,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lsr,
        addressing: 4u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cli,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: eor,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: eor,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: lsr,
        addressing: 8u8,
        cycles: 7u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: rts,
        addressing: 0u8,
        cycles: 6u8,
        bytes: 1u8,
    },
    Opcode {
        exec: adc,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: adc,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ror,
        addressing: 3u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: pla,
        addressing: 0u8,
        cycles: 4u8,
        bytes: 1u8,
    },
    Opcode {
        exec: adc,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ror,
        addressing: 1u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: jmp,
        addressing: 10u8,
        cycles: 5u8,
        bytes: 3u8,
    },
    Opcode {
        exec: adc,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: ror,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bvs,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: adc,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: adc,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ror,
        addressing: 4u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sei,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: adc,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: adc,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: ror,
        addressing: 8u8,
        cycles: 7u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sta,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sty,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: sta,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: stx,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: dey,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: txa,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sty,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: sta,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: stx,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bcc,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: sta,
        addressing: 12u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sty,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: sta,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: stx,
        addressing: 5u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: tya,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sta,
        addressing: 9u8,
        cycles: 5u8,
        bytes: 3u8,
    },
    Opcode {
        exec: txs,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sta,
        addressing: 8u8,
        cycles: 5u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ldy,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lda,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ldx,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ldy,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lda,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ldx,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: tay,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: lda,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: tax,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ldy,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: lda,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: ldx,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bcs,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lda,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ldy,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: lda,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: ldx,
        addressing: 5u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: clv,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: lda,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: tsx,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: ldy,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: lda,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: ldx,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cpy,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: cmp,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cpy,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: cmp,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: dec,
        addressing: 3u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: iny,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cmp,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: dex,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cpy,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: cmp,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: dec,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: bne,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: cmp,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cmp,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: dec,
        addressing: 4u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cld,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cmp,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cmp,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: dec,
        addressing: 8u8,
        cycles: 7u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cpx,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: sbc,
        addressing: 11u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cpx,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: sbc,
        addressing: 3u8,
        cycles: 3u8,
        bytes: 2u8,
    },
    Opcode {
        exec: inc,
        addressing: 3u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: inx,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sbc,
        addressing: 2u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: nop,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: cpx,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: sbc,
        addressing: 7u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: inc,
        addressing: 7u8,
        cycles: 6u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: beq,
        addressing: 6u8,
        cycles: 2u8,
        bytes: 2u8,
    },
    Opcode {
        exec: sbc,
        addressing: 12u8,
        cycles: 5u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sbc,
        addressing: 4u8,
        cycles: 4u8,
        bytes: 2u8,
    },
    Opcode {
        exec: inc,
        addressing: 4u8,
        cycles: 6u8,
        bytes: 2u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sed,
        addressing: 0u8,
        cycles: 2u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sbc,
        addressing: 9u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
    Opcode {
        exec: sbc,
        addressing: 8u8,
        cycles: 4u8,
        bytes: 3u8,
    },
    Opcode {
        exec: inc,
        addressing: 8u8,
        cycles: 7u8,
        bytes: 3u8,
    },
    Opcode {
        exec: placeholder,
        addressing: 0u8,
        cycles: 0u8,
        bytes: 1u8,
    },
];
pub fn placeholder(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
    println!("opcode not implemented.");
    0
}