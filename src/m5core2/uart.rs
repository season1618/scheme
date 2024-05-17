use esp32_hal::prelude::*;
use core::str::{Utf8Error, from_utf8};

use crate::m5core2::M5Core2;

impl<'a> M5Core2<'_> {
    pub fn read_line<'b>(&mut self, buf: &'b mut [u8]) -> Result<&'b str, Utf8Error> {
        for idx in 0..buf.len() {
            let _ = self.read_u8();
            let c = self.read_u8();
            if c == b'\r' {
                self.uart.write(b'\n').unwrap();
                return from_utf8(&buf[..idx]);
            } else {
                self.uart.write(c).unwrap();
                buf[idx] = c;
            }
        }
        from_utf8(buf)
    }

    fn read_u8(&mut self) -> u8 {
        loop {
            match self.uart.read() {
                Ok(c) => break c,
                Err(_) => {},
            }
        }
    }
}
