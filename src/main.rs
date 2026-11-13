#![no_main] //For Not linking the RUST Library as this is a os so we need to make it rid of any dependency
#![no_std] //disable all RUST level entry points

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle this function  meaning don't use this name anywhere
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by default
    loop{} 
}
//This function is called on panic

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


