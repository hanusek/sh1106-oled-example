
use sh1106::{prelude::*, Builder};
use linux_embedded_hal::I2cdev;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use embedded_graphics::{
    primitives::{
        Circle, Line, PrimitiveStyle, Rectangle, Triangle,
    },
};

const SLAVE_ADDR: u16 = 0x3c;

fn main() 
{
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    i2c.set_slave_address(SLAVE_ADDR).unwrap();

    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    display.init().unwrap();
    display.flush().unwrap();
  
    let yoffset = 8;

    Triangle::new(Point::new(8, 16 + yoffset), Point::new(8 + 16, 16 + yoffset), Point::new(8 + 8, yoffset))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut display).unwrap();

    Rectangle::with_corners(Point::new(48, yoffset), Point::new(48 + 16, 16 + yoffset))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Circle::new(Point::new(88, yoffset), 16)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();
    
    let b = display.bounding_box();    
    let w : i32 = b.size.width as i32;
    let h : i32 = b.size.height as i32;
    println!("w: {} | h: {}", w, h);

    Line::new(
        Point::new(0, h/2),
        Point::new(w, h/2),
    )
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut display)
    .unwrap();

    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();

    Text::with_baseline("Hello world!", Point::new(0, h/2 + 5), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust", Point::new(0, h/2 + 5 + 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}
