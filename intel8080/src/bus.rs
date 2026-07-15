use std::any::Any;

pub trait Bus: Any {
    fn read8(&mut self, addr: u16) -> u8;
    fn write8(&mut self, addr: u16, value: u8);

    fn input(&mut self, port: u8) -> u8;
    fn output(&mut self, port: u8, value: u8);
}
