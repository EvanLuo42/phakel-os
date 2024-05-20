use core::fmt::Write;
use core::num::Wrapping;
use crate::arch::Uart;

/// receive holding register (for input bytes)
const RHR: usize = 0;
/// transmit holding register (for output bytes)
const THR: usize = 0;
/// interrupt enable register
const IER: usize = 1;
/// FIFO control register
const FCR: usize = 2;
/// interrupt status register
const ISR: usize = 2;
/// line control register
const LCR: usize = 3;
/// line status register
const LSR: usize = 5;

const IER_RX_ENABLE: usize = 1 << 0;
const IER_TX_ENABLE: usize = 1 << 1;
const FCR_FIFO_ENABLE: usize = 1 << 0;
const FCR_FIFO_CLEAR: usize = 3 << 1;
const LCR_EIGHT_BITS: usize = 3;
const LCR_BAUD_LATCH: usize = 1 << 7;
const LSR_RX_READY: usize = 1 << 0;
const LSR_TX_IDLE: usize = 1 << 5;

const UART_BASE_ADDR: usize = 0x10000000;
const UART_BUF_SIZE: usize = 32;

pub struct RiscVUart {
    buf: [u8; UART_BUF_SIZE],
    /// Write to next to buf[write_index % UART_BUF_SIZE]
    write_index: Wrapping<usize>,
    /// Read next from buf[read_index % UART_BUF_SIZE]
    read_index: Wrapping<usize>   
}

impl Write for RiscVUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Uart for RiscVUart {
    fn new() -> Self {
        Self {
            buf: [0u8; UART_BUF_SIZE],
            write_index: Wrapping(0),
            read_index: Wrapping(0)
        }
    }

    fn init(&mut self) {
        // Disable interrupts
        write_reg(UART_BASE_ADDR + IER, 0x00);

        // Special mode to set baud rate.
        write_reg(UART_BASE_ADDR + LCR, LCR_BAUD_LATCH as u8);

        // LSB for baud rate of 38.4K
        write_reg(UART_BASE_ADDR, 0x03);

        // MSB for baud rate of 38.4k
        write_reg(UART_BASE_ADDR + 1, 0x00);

        // Leave set-baud mode,
        // and set word length to 8 bits, no parity.
        write_reg(UART_BASE_ADDR + LCR, LCR_EIGHT_BITS as u8);

        // Reset and enable FIFOs.
        write_reg(UART_BASE_ADDR + FCR, FCR_FIFO_ENABLE as u8 | FCR_FIFO_CLEAR as u8);

        // Enable transmit and receive interrupts.
        write_reg(UART_BASE_ADDR + IER, IER_TX_ENABLE as u8 | IER_RX_ENABLE as u8);
    }

    fn put(&mut self, c: u8) {
        let ptr = UART_BASE_ADDR as *mut u8;
        // Write until previous data is flushed
        while unsafe { ptr.add(5).read_volatile() } & (1 << 5) == 0 {
        }
        unsafe {
            // Write data
            ptr.add(0).write_volatile(c);
        }
    }

    fn get(&mut self) -> Option<u8> {
        todo!()
    }
}

fn write_reg(addr: usize, val: u8) {
    unsafe {
        core::ptr::write(addr as *mut u8, val);
    }
}