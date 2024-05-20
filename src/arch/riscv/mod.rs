mod uart;

use sbi_rt::{NoReason, Shutdown, system_reset, SystemFailure};
use crate::arch::Arch;
use crate::arch::riscv::uart::RiscVUart;

pub struct RiscV;

impl Arch for RiscV {
    type Uart = RiscVUart;

    fn shutdown(failure: bool) -> ! {
        if !failure {
            system_reset(Shutdown, NoReason);
        } else {
            system_reset(Shutdown, SystemFailure);
        }
        unreachable!()
    }
}