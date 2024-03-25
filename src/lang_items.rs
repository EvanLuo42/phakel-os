use core::panic::PanicInfo;
use crate::println;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    if let Some(location) = _panic_info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            _panic_info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", _panic_info.message().unwrap());
    }
    shutdown(true)
}
