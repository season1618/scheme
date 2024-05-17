use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb666,
    prelude::*,
    text::Text,
};
use display_interface::DisplayError;

use crate::m5core2::M5Core2;

static STYLE: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, Rgb666::BLACK);

impl<'a> M5Core2<'a> {
    pub fn draw(&mut self, text: &str) -> Result<Point, DisplayError> {
        self.line += 1;
        Text::new(text, Point::new(0, 20 * self.line as i32), STYLE).draw(&mut self.lcd)
    }
}
