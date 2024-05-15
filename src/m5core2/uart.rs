use esp32_hal::{
    prelude::*,
    uart::{Instance, Uart},
};
use core::str::from_utf8;

pub fn read_line<'a, T>(uart: &mut Uart<T>, buf: &'a mut [u8]) -> &'a str
    where T: Instance {
    for idx in 0..buf.len() {
        let _ = read_u8(uart);
        match read_u8(uart) {
            b'\r' => {
                uart.write(b'\n').unwrap();
                return from_utf8(&buf[..idx]).unwrap();
            },
            c => {
                uart.write(c).unwrap();
                buf[idx] = c;
            },
        }
    }
    from_utf8(buf).unwrap()
}

fn read_u8<T>(uart: &mut Uart<T>) -> u8
    where T: Instance {
    loop {
        match uart.read() {
            Ok(c) => break c,
            Err(_) => {},
        }
    }
}
