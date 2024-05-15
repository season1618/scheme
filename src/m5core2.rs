pub mod uart;
pub use uart::read_line;

use esp32_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::IO,
    peripherals::{Peripherals, UART0},
    prelude::*,
    uart::{TxRxPins, Uart}
};

pub fn m5core2_new<'a>() -> Uart<'a, UART0> {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let _delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let txrxpins = TxRxPins::new_tx_rx(
        io.pins.gpio1.into_push_pull_output(),
        io.pins.gpio3.into_floating_input(),
    );
    
    let uart = Uart::new_with_config(
        peripherals.UART0,
        None,
        Some(txrxpins),
        &clocks,
        &mut system.peripheral_clock_control,
    );

    uart
}