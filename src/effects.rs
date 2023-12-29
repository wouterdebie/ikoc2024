#![allow(dead_code)]
use std::time::Duration;

use crate::utils::screen_center;
use crate::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};
use anyhow::{Error, Result};
use embedded_graphics::mono_font::ascii::FONT_5X7;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, LineHeight, Text, TextStyleBuilder};

pub fn blink<D>(
    display: &mut Display,
    d: &mut D,
    times: u32,
    duration: Duration,
) -> Result<(), Error>
where
    D: Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    for _ in 0..times {
        d.draw(display).unwrap();
        display.flush().unwrap();
        std::thread::sleep(duration);
        display.clear(BinaryColor::Off).unwrap();
        display.flush().unwrap();
        std::thread::sleep(duration);
    }
    Ok(())
}

pub fn type_text(display: &mut Display, s: &str) -> Result<(), Error> {
    let character_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(110))
        .baseline(Baseline::Top)
        .build();

    let mut out = "".to_string();
    let mut lines = 0;
    for (i, c) in s.chars().enumerate() {
        // End of the line
        if i > 0 && i % 25 == 0 {
            out.push('\n');
            lines += 1;
            // Move up when there more than 3 lines and every consecutive line
            if lines > 2 {
                let mut split = out.splitn(2, '\n');
                split.next();
                out = split.next().unwrap().to_string();
                // clear display
                display.clear(BinaryColor::Off).unwrap();
            }
        }
        out.push(c);
        let text = Text::with_text_style(&out, Point::new(0, 0), character_style, text_style);

        text.draw(display).unwrap();
        display.flush().unwrap();
    }

    Ok(())
}

fn scroll<D>(display: &mut Display, d: &mut D, from: Point, to: Point) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    // Calculate the distance between from and to
    let distance_x = (from.x - to.x).abs();
    let distance_y = (from.y - to.y).abs();
    let distance = ((distance_x.pow(2) + distance_y.pow(2)) as f32).sqrt() as u32;

    // Calculate the step size for x and y
    let by = Point::new(get_step(from.x, to.x), get_step(from.y, to.y));

    // First translate the d to 0,0 and translate it to `from`
    d.translate_mut(Point::new(
        -d.bounding_box().top_left.x + from.x,
        -d.bounding_box().top_left.y + from.y,
    ));

    // Create a bounding box one pixel larger than the d to clear the previous d.
    // We use this to make sure that the previous d is cleared, instead of completely
    // clearing the display between every frame.
    let mut bb = Rectangle::new(
        d.bounding_box().top_left - Point::new(1, 1),
        d.bounding_box().size + Size::new(2, 2),
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
        // Translate the bounding box
        bb.translate_mut(by);
        bb.draw(display).unwrap();

        // Translate the d
        d.translate_mut(by);
        d.draw(display).unwrap();

        // Write the display buffer to the display
        display.flush().unwrap();
    }
    Ok(())
}

fn get_step(from: i32, to: i32) -> i32 {
    match from.cmp(&to) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

pub fn left<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    left_in(display, d)?;
    left_out(display, d)
}

pub fn left_in<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    let center = screen_center(d);
    scroll(
        display,
        d,
        Point::new(SCREEN_WIDTH as i32, center.y),
        center,
    )
}

pub fn left_out<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    scroll(
        display,
        d,
        d.bounding_box().top_left,
        Point::new(-(d.bounding_box().size.width as i32), screen_center(d).y),
    )
}

pub fn right<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    right_in(display, d)?;
    right_out(display, d)
}

pub fn right_in<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    let center = screen_center(d);
    scroll(
        display,
        d,
        Point::new(-(d.bounding_box().size.width as i32), center.y),
        center,
    )
}

pub fn right_out<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    scroll(
        display,
        d,
        d.bounding_box().top_left,
        Point::new(SCREEN_WIDTH as i32, screen_center(d).y),
    )
}

pub fn up<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    up_in(display, d)?;
    up_out(display, d)
}

pub fn up_in<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    let center = screen_center(d);

    scroll(
        display,
        d,
        Point::new(center.x, SCREEN_HEIGHT as i32),
        center,
    )
}

pub fn up_out<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    scroll(
        display,
        d,
        d.bounding_box().top_left,
        Point::new(screen_center(d).x, -(d.bounding_box().size.height as i32)),
    )
}

pub fn down<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    down_in(display, d)?;
    down_out(display, d)
}

pub fn down_in<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    let center = screen_center(d);
    scroll(
        display,
        d,
        Point::new(center.x, -(d.bounding_box().size.height as i32)),
        center,
    )
}

pub fn down_out<D>(display: &mut Display, d: &mut D) -> Result<(), Error>
where
    D: Dimensions + Transform + Drawable<Color = embedded_graphics::pixelcolor::BinaryColor>,
{
    scroll(
        display,
        d,
        d.bounding_box().top_left,
        Point::new(screen_center(d).x, SCREEN_HEIGHT as i32),
    )
}
