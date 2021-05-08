use std::ops::Add;

use crate::prelude::*;

mod delta_ops;

/// stores the state of the emulation along with enough history to 
/// view any recent state so that correct state per cycle can be observed
/// even without modeling each instruction cycle-by-cycle (read, modify, store)
pub struct State {
    // oldest_state: Snapshot,
    /// Recent changes to the emulator state. This vec is assumed to be sorted
    deltas: Vec<Delta>,
    newest_state: Snapshot,
}

impl State {
    pub fn get_snapshot(&self, cycle: u64) -> Snapshot {
        self.deltas.iter().filter(|delta|delta.cycle > cycle).fold(self.newest_state.clone(), |acc, delta| delta.apply(acc))
    }
    pub fn get_thin_snapshot(&self, cycle: u64) -> StateView {
        unimplemented!()
    }
    pub fn prune_before(&mut self, cycle: u64) {
        self.deltas.retain(|delta| delta.cycle >= cycle);
    }
}


pub struct StateView<'a> {
    state: &'a State,
    cycle: u64,
}

/// the whole state of the emulator
#[derive(Clone)]
pub struct Snapshot {
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    sp: u8,
    pc: u16,
    memory: Box<[u8;0x1_0000]>,
}


impl AddressableMemory for Snapshot {
    fn bounds(&self) -> (u16, u16) {
        (0, 0xffff)
    }
    fn set(&mut self, address: u16, value: u8) {
        // safety: safe because the slice we are indexing into is u16::MAX sized,
        // and we have a mutable reference to it.
        unsafe { *self.memory.get_unchecked_mut(address as usize) = value;}
    }
    fn get(&self, address: u16) -> u8 {
        // safety: safe because the slice we are indexing into is u16::MAX sized,
        // and we have a reference to it.
        unsafe {*self.memory.get_unchecked(address as usize)}
    }
}

impl RegisterAccess for Snapshot {
    fn get_a(&self) -> u8 {
        self.a
    }
    fn get_x(&self) -> u8 {
        self.x
    }
    fn get_y(&self) -> u8 {
        self.y
    }
    fn get_p(&self) -> u8 {
        self.p
    }
    fn get_sp(&self) -> u8 {
        self.sp
    }
    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn set_a(&mut self, value: u8) -> u8 {
        let old = self.a;
        self.a = value;
        old
    }
    fn set_x(&mut self, value: u8) -> u8 {
        let old = self.x;
        self.x = value;
        old
    }
    fn set_y(&mut self, value: u8) -> u8 {
        let old = self.y;
        self.y = value;
        old
    }
    fn set_p(&mut self, value: u8) -> u8 {
        let old = self.p;
        self.p = value;
        old
    }
    fn set_sp(&mut self, value: u8) -> u8 {
        let old = self.sp;
        self.sp = value;
        old
    }
    fn set_pc(&mut self, value: u16) -> u16 {
        let old = self.pc;
        self.pc = value;
        old
    }
}


#[test]
fn snapshot_size() {
    let s = std::mem::size_of::<Snapshot>();
    // let rw = std::mem::size_of::<RegisterWrite>();
    // let gw = std::mem::size_of::<GlobalEvent>();
    // let d = std::mem::size_of::<DeltaEvent>();
    // assert_eq!(mw, 4 );
    // assert_eq!(rw, 6 );
    // assert_eq!(gw, 1 );
    assert_eq!(s, 7+8+1 );
}
/// An update to the game state, along with the cycle in which it should occur
pub struct Delta {
    cycle: u64,
    event:DeltaEvent,
}

impl Delta {
    pub fn new(cycle: u64, event: DeltaEvent) -> Delta {
        Delta { cycle, event}
    }
    pub fn apply(&self, mut snapshot: Snapshot) -> Snapshot {
        use DeltaEvent::*;
        match &self.event {
            WriteMem(MemoryWrite{address, old_value, status}) => {
                snapshot.set(*address, *old_value);
                snapshot.set_p(*status);
            }
            WriteRegister(RegisterWrite::PC(old, status)) => {
                snapshot.set_pc(*old);
                snapshot.set_p(*status);
            }
            WriteRegister(RegisterWrite::Byte(which,RegisterHistory{old, status})) => {
                use ByteRegister::*;
                match which {
                    A => snapshot.set_a(*old),
                    X => snapshot.set_x(*old),
                    Y => snapshot.set_y(*old),
                    SP => snapshot.set_sp(*old),
                    P => snapshot.set_p(*old),
                };
                snapshot.set_p(*status);
            }
            GlobalStateUpdate(ge) => {
                unimplemented!()
            }
        }
        snapshot
    }
}

/// a single update event to the game state
pub enum DeltaEvent {
    WriteMem(MemoryWrite),
    WriteRegister(RegisterWrite),
    GlobalStateUpdate(GlobalEvent),
}

impl DeltaEvent {
    pub fn write_mem(address: u16, old_value: u8, status: u8) -> DeltaEvent {
        DeltaEvent::WriteMem(MemoryWrite{address, old_value, status})
    }
    pub fn write_x(old: u8, status: u8) -> DeltaEvent {
        DeltaEvent::WriteRegister(RegisterWrite::Byte(ByteRegister::X, RegisterHistory{old, status}))
    }
    pub fn write_a(old: u8, status: u8) -> DeltaEvent {
        DeltaEvent::WriteRegister(RegisterWrite::Byte(ByteRegister::A, RegisterHistory{old, status}))
    }
    pub fn write_y(old: u8, status: u8) -> DeltaEvent {
        DeltaEvent::WriteRegister(RegisterWrite::Byte(ByteRegister::Y, RegisterHistory{old, status}))
    }
}

#[test]
fn delta_size() {
    let mw = std::mem::size_of::<MemoryWrite>();
    let rw = std::mem::size_of::<RegisterWrite>();
    let gw = std::mem::size_of::<GlobalEvent>();
    let de = std::mem::size_of::<DeltaEvent>();
    let d = std::mem::size_of::<Delta>();
    assert_eq!(mw, 4 );
    assert_eq!(rw, 4 );
    assert_eq!(gw, 1 );
    assert_eq!(de, 6 );
    assert_eq!(d, 16 );
}

pub struct MemoryWrite {
    address: u16,
    old_value: u8,
    status: u8,
}

pub enum RegisterWrite {
    Byte(ByteRegister, RegisterHistory),
    // X(RegisterHistory),
    // Y(RegisterHistory),
    // P(RegisterHistory),
    PC(u16,u8),
    // SP(RegisterHistory),
}

enum ByteRegister {
    A,X,Y,P,SP,
}

pub struct RegisterHistory {
    old: u8,
    status: u8,
}

pub enum GlobalEvent {
    Reset,
    Stop,
}

pub trait Event {
    fn apply(&self, snapshot:&mut Snapshot);
    fn undo(&self, snapshot:&mut Snapshot);
}


pub mod registers {
    use super::RegisterAccess;
    pub trait Register<A> {
        fn get(&self, access: &A) -> u8;
        fn set(&mut self, access: &mut A, value: u8) -> u8;
    }
    
    pub struct X;
    pub struct Y;
    pub struct A;

    impl<A: RegisterAccess> Register<A> for X {
        fn get(&self, access: &A) -> u8 {
            access.get_x()
        }
        fn set(&mut self, access: &mut A, value: u8) -> u8 {
            let old = access.get_x();
            access.set_x(value);
            old
        }
    }
    impl<A: RegisterAccess> Register<A> for Y {
        fn get(&self, access: &A) -> u8 {
            access.get_y()
        }
        fn set(&mut self, access: &mut A, value: u8) -> u8 {
            let old = access.get_y();
            access.set_y(value);
            old
        }
    }
    impl<A: RegisterAccess> Register<A> for A {
        fn get(&self, access: &A) -> u8 {
            access.get_a()
        }
        fn set(&mut self, access: &mut A, value: u8) -> u8 {
            let old = access.get_a();
            access.set_a(value);
            old
        }
    }
}
