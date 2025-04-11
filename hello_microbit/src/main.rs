#![deny(unsafe_code)]
#![no_main]
#![no_std]

// `entry` is the entry point of the program, `main` is not assumed because of `no_main`
use cortex_m_rt::entry;

// the `microbit` crate provides the API for the micro:bit
// if this is omitted, the compile will fail
use microbit as _;

// `rtt_target` is used for RTT logging
use rtt_target::{rtt_init_print,rprintln};

// `panic_rtt_target` is used for panic messages
use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    // initialize RTT logging
    rtt_init_print!();

    loop {
        // print a message to the RTT log
        rprintln!("Hello from micro:bit!");
    }
}