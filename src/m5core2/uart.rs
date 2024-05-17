use esp32_hal::prelude::*;
use core::str::{Utf8Error, from_utf8};

use crate::m5core2::M5Core2;

impl<'a> M5Core2<'_> {
    pub fn read_line<'b>(&mut self, buf: &'b mut [u8]) -> Result<&'b str, Utf8Error> {
        let mut idx = 0;
        while idx < buf.len() {
            let _ = self.read_u8(); // espflash monitor sends input twice.
            let c = self.read_u8();
            if c == b'\r' { // espflash monitor sends CR as new line.
                self.uart.write(b'\n').unwrap();
                return from_utf8(&buf[..idx]);
            } else if c == 0x08 { // backspace
                if idx > 0 {
                    self.uart.write(0x08).unwrap();
                    idx -= 1;
                }
            } else {
                self.uart.write(c).unwrap();
                buf[idx] = c;
                idx += 1;
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
