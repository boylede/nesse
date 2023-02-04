use super::*;
use nesse_common::AddressingMode;

fn translate_address<N: AddressableMemory + RegisterAccess>(
    snap: &mut N,
    mode: AddressingMode,
) -> u16 {
    use AddressingMode::*;
    match mode {
        Implicit => {
            panic!("Should not request a get with implicit address mode.");
        }
        Accumulator => {
            panic!("Should not request a get with accumulator address mode.");
        }
        Immediate => {
            let value = snap.get_pc();
            snap.increment_pc();
            value
        }
        ZeroPage => {
            let value = snap.get(snap.get_pc()) as u16;
            snap.increment_pc();
            value
        }
        ZeroPageX => {
            let base = snap.get(snap.get_pc());
            snap.increment_pc();
            // todo: is this behavior correct? will wrap around zero page
            base.wrapping_add(snap.get_x()) as u16
        }
        ZeroPageY => {
            let base = snap.get(snap.get_pc());
            snap.increment_pc();
            // todo: is this wrap intended before the cast to u16 or after
            base.wrapping_add(snap.get_y()) as u16
        }
        Relative => {
            let base = snap.get(snap.get_pc());
            snap.increment_pc();
            base as u16
        }
        Absolute => {
            let value = snap.get_pc();
            snap.increment_pc();
            snap.increment_pc(); // skips two bytes since pointers are a two byte value
            value
        }
        AbsoluteX => {
            unimplemented!()
        }
        AbsoluteY => {
            unimplemented!()
        }
        Indirect => {
            unimplemented!()
        }
        IndexedIndirect => {
            // The address of the table is taken from the instruction and the X register added to it (with zero page wrap around) to give the location of the least significant byte of the target address.
            let table = snap.get(snap.get_pc());
            snap.increment_pc();
            let base = table.wrapping_add(snap.get_x()) as u16;
            let lo = snap.get(base) as u16;
            let hi = snap.get(base.wrapping_add(1)) as u16;
            (hi << 8) | lo
        }
        IndirectIndexed => {
            let base = snap.get(snap.get_pc());
            snap.increment_pc();
            let lo = snap.get(base as u16);
            let hi = snap.get(base.wrapping_add(1) as u16);
            (hi as u16) << 8 | (lo as u16).wrapping_add(snap.get_y() as u16)
        }
    }
}

pub fn ldx(snap: &mut Snapshot, mode: AddressingMode, cycle: u64) -> Delta {
    let address = translate_address(snap, mode);
    let value = snap.get(address);
    let old_value = snap.get_x();
    let old_flags = snap.get_p();
    snap.set_x(value);
    snap.set_flags_from(value);
    use AddressingMode::*;
    let cycles = match mode {
        Immediate => 2,
        ZeroPage => 3,
        ZeroPageY => 4,
        Absolute => 4,
        AbsoluteY => 4, // todo: how to determine if page crossed?
        _ => 0,         // should not be reached
    };
    Delta::new(
        cycle.wrapping_add(cycles),
        DeltaEvent::write_x(old_value, old_flags),
    )
}

pub fn lda(snap: &mut Snapshot, mode: AddressingMode, cycle: u64) -> Delta {
    let address = translate_address(snap, mode);
    let value = snap.get(address);
    let old_value = snap.get_a();
    let old_flags = snap.get_p();
    snap.set_a(value);
    snap.set_flags_from(value);
    use AddressingMode::*;
    let cycles = match mode {
        Immediate => 2,
        ZeroPage => 3,
        ZeroPageX => 4,
        Absolute => 4,
        AbsoluteX => 4,
        AbsoluteY => 4, // todo: how to determine if page crossed?
        IndexedIndirect => 6,
        IndirectIndexed => 5,
        _ => 0, // should not be reached
    };
    Delta::new(
        cycle.wrapping_add(cycles),
        DeltaEvent::write_a(old_value, old_flags),
    )
}

pub fn ldy(snap: &mut Snapshot, mode: AddressingMode, cycle: u64) -> Delta {
    let address = translate_address(snap, mode);
    let value = snap.get(address);
    let old_value = snap.get_y();
    let old_flags = snap.get_p();
    snap.set_y(value);
    snap.set_flags_from(value);
    use AddressingMode::*;
    let cycles = match mode {
        Immediate => 2,
        ZeroPage => 3,
        ZeroPageX => 4,
        Absolute => 4,
        AbsoluteX => 4, // todo: page crossing
        _ => 0,         // should not be reached
    };
    Delta::new(
        cycle.wrapping_add(cycles),
        DeltaEvent::write_y(old_value, old_flags),
    )
}

pub fn sta(snap: &mut Snapshot, mode: AddressingMode, cycle: u64) -> Delta {
    let address = translate_address(snap, mode);
    let old_flags = snap.get_p();
    let old_value = snap.get(address);
    snap.set(address, snap.get_a());
    use AddressingMode::*;
    let cycles = match mode {
        ZeroPage => 3,
        ZeroPageX => 4,
        Absolute => 4,
        AbsoluteX => 5, // todo: page crossing
        AbsoluteY => 5,
        IndexedIndirect => 6,
        IndirectIndexed => 6,
        _ => 0, // should not be reached
    };
    Delta::new(
        cycle.wrapping_add(cycles),
        DeltaEvent::write_mem(address, old_value, old_flags),
    )
}
pub fn stx(snap: &mut Snapshot, mode: AddressingMode, cycle: u64) -> Delta {
    let address = translate_address(snap, mode);
    let old_flags = snap.get_p();
    let old_value = snap.get(address);
    snap.set(address, snap.get_x());
    use AddressingMode::*;
    let cycles = match mode {
        ZeroPage => 3,
        ZeroPageX => 4,
        Absolute => 4,
        _ => 0, // should not be reached
    };
    Delta::new(
        cycle.wrapping_add(cycles),
        DeltaEvent::write_mem(address, old_value, old_flags),
    )
}
pub fn sty(snap: &mut Snapshot, mode: AddressingMode, cycle: u64) -> Delta {
    let address = translate_address(snap, mode);
    let old_flags = snap.get_p();
    let old_value = snap.get(address);
    snap.set(address, snap.get_y());
    use AddressingMode::*;
    let cycles = match mode {
        ZeroPage => 3,
        ZeroPageX => 4,
        Absolute => 4,
        _ => 0, // should not be reached
    };
    Delta::new(
        cycle.wrapping_add(cycles),
        DeltaEvent::write_mem(address, old_value, old_flags),
    )
}
