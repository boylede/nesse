pub trait Mapper {
    fn translate(&self, address: u16) -> u16 {
        address
    }
}


impl Mapper for () {}
