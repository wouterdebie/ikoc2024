#![allow(dead_code)]
use anyhow::{Error, Result};
use embedded_graphics::image::ImageRaw;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_5X7;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::{image::Image, prelude::*};
use crate::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};

fn scroll(
    display: &mut Display,
    text: &str,
    from: Point,
    to: Point,
) -> Result<(), Error> {
    display.clear(BinaryColor::Off).unwrap();

    let mut text = Text::new(
        text,
        Point::new(0, 0),
        MonoTextStyle::new(&FONT_5X7, BinaryColor::On),
    );

    let distance_x = (from.x - to.x).abs();
    let distance_y = (from.y - to.y).abs();

    let distance = ((distance_x.pow(2) + distance_y.pow(2)) as f32).sqrt() as u32;

    let step_x = match from.x.cmp(&to.x) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    };

    let step_y = match from.y.cmp(&to.y) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    };

    let mut bb = Rectangle::new(
        text.bounding_box().top_left - Point::new(1, 1),
        text.bounding_box().size + Size::new(2, 2),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .stroke_color(BinaryColor::Off)
            .stroke_width(1)
            .build(),
    );

    // Scroll from "from" to "to"
    for _ in 0..distance {
        // draw bounding box one pixel larger than the image to clear the previous image
        bb.translate_mut(Point::new(step_x, step_y));
        bb.draw(display).unwrap();

        text.translate_mut(Point::new(step_x, step_y));
        text.draw(display).unwrap();
        display.flush().unwrap();
    }
    Ok(())
}


pub fn scroll_left(display: &mut Display, raw: &ImageRaw<BinaryColor>) -> Result<(), Error> {
    let y = ((SCREEN_HEIGHT / 2) - (raw.size().height / 2)) as i32;

    scroll(
        display,
        raw,
        Point::new(SCREEN_WIDTH as i32, y),
        Point::new(-(raw.size().width as i32), y),
    )
}

pub fn scroll_right(display: &mut Display, raw: &ImageRaw<BinaryColor>) -> Result<(), Error> {
    let y = ((SCREEN_HEIGHT / 2) - (raw.size().height / 2)) as i32;

    scroll(
        display,
        raw,
        Point::new(-(raw.size().width as i32), y),
        Point::new(SCREEN_WIDTH as i32, y),
    )
}

pub fn scroll_up(display: &mut Display, raw: &ImageRaw<BinaryColor>) -> Result<(), Error> {
    let x = ((SCREEN_WIDTH / 2) - (raw.size().width / 2)) as i32;

    scroll(
        display,
        raw,
        Point::new(x, SCREEN_HEIGHT as i32),
        Point::new(x, -(raw.size().height as i32)),
    )
}

pub fn scroll_down(display: &mut Display, raw: &ImageRaw<BinaryColor>) -> Result<(), Error> {
    let x = ((SCREEN_WIDTH / 2) - (raw.size().width / 2)) as i32;

    scroll(
        display,
        raw,
        Point::new(x, -(raw.size().height as i32)),
        Point::new(x, SCREEN_HEIGHT as i32),
    )
}

fn draw(display: &mut Display, raw: &ImageRaw<BinaryColor>) -> Result<(), Error> {
    display.clear(BinaryColor::Off).unwrap();
    Image::new(raw, Point::new(0, 0)).draw(display).unwrap();
    display.flush().unwrap();
    Ok(())
}
