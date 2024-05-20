#[cfg(target_arch = "riscv64")]
extern "C" {
    pub fn skernel();
    pub fn ekernel();
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss();
    pub fn ebss();
}