use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb666,
    prelude::*,
    text::Text,
};

static STYLE: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_6X10, Rgb666::BLACK);

pub fn draw<D, E>(lcd: &mut D, text: &str) -> Result<Point, E> 
where D: DrawTarget<Color = Rgb666, Error = E> {
    Text::new(text, Point::new(0, 10), STYLE).draw(lcd)
}