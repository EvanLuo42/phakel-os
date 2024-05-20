use core::fmt::Write;

pub mod riscv;

pub trait Arch {
    type Uart: Uart;

    fn shutdown(failure: bool) -> !;
}

pub trait Uart: Write {
    fn new() -> Self;
    fn init(&mut self);
    fn put(&mut self, c: u8);
    fn get(&mut self) -> Option<u8>;
}