pub mod uart;
pub mod imu;
pub mod lcd;
mod pmu;

pub use uart::read_line;
pub use lcd::write;
pub use imu::{accel, gyro, temp};

use esp32_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{AnyPin, Gpio15, IO, Output, PushPull},
    i2c::I2C,
    peripherals::{I2C0, Peripherals, SPI2, UART0},
    prelude::*,
    spi::{FullDuplexMode, Spi, SpiMode},
    uart::{TxRxPins, Uart}
};
use axp192::Axp192;
use display_interface_spi::SPIInterfaceNoCS;
use mipidsi::{
    Builder,
    Display,
    models::ILI9342CRgb666,
    options::{ColorInversion, ColorOrder},
};

use pmu::pmu_init;
use imu::imu_init;

pub struct M5Core2<'a> {
    pub uart: Uart<'a, UART0>,
    pub imu: &'a mut I2C<'a, I2C0>,
    pub lcd: Display<SPIInterfaceNoCS<Spi<'a, SPI2, FullDuplexMode>, Gpio15<Output<PushPull>>>, ILI9342CRgb666, AnyPin<Output<PushPull>>>,
}

impl<'a> M5Core2<'a> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let mut system = peripherals.DPORT.split();
        let mut clocks = ClockControl::max(system.clock_control).freeze();
        let mut delay = Delay::new(&clocks);
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

        let mut i2c = I2C::new(
            peripherals.I2C0,
            io.pins.gpio21,
            io.pins.gpio22,
            400u32.kHz(),
            &mut system.peripheral_clock_control,
            &clocks,
        );

        let i2c_ptr = &mut i2c as *mut I2C<_>;
        let mut imu = unsafe { &mut *i2c_ptr as &mut I2C<_> };
        imu_init(&mut imu, &mut delay).unwrap();

        let mut pmu = Axp192::new(i2c);
        pmu_init(&mut pmu, &mut delay).unwrap();

        let spi = Spi::new(
            peripherals.SPI2,
            io.pins.gpio18,
            io.pins.gpio23,
            io.pins.gpio38,
            io.pins.gpio5,
            400u32.kHz(),
            SpiMode::Mode0,
            &mut system.peripheral_clock_control,
            &mut clocks
        );
        let spi_iface = SPIInterfaceNoCS::new(spi, io.pins.gpio15.into_push_pull_output());
        let lcd = Builder::ili9342c_rgb666(spi_iface)
            .with_display_size(320, 240)
            .with_color_order(ColorOrder::Bgr)
            .with_invert_colors(ColorInversion::Inverted)
            .init(&mut delay, None::<AnyPin<Output<PushPull>>>)
            .unwrap();

        M5Core2 { uart, imu, lcd }
    }
}