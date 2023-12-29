use anyhow::Result;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::mono_font::{ascii::FONT_5X7, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Baseline, LineHeight, Text, TextStyleBuilder};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::i2c::I2cDriver;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{i2c, peripherals::Peripherals};
use ssd1306::mode::{BufferedGraphicsMode, DisplayConfig};
use ssd1306::prelude::I2CInterface;
use ssd1306::{rotation::DisplayRotation, size::DisplaySize128x32, Ssd1306};
use utils::{screen_center, set_thread_spawn_configuration};

mod effects;
mod utils;
// mod text;

const SCREEN_WIDTH: u32 = 128;
const SCREEN_HEIGHT: u32 = 32;

type Display = Ssd1306<
    I2CInterface<I2cDriver<'static>>,
    DisplaySize128x32,
    BufferedGraphicsMode<DisplaySize128x32>,
>;

struct Poetry {
    text: String,
    src: u32,
}

struct RecvStats {
    received: u32,
}

struct SendStats {
    sent: u32,
}

enum Msg {
    Poetry(Poetry),
    RecvStats(RecvStats),
    SendStats(SendStats),
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let di = ssd1306::I2CDisplayInterface::new(
        i2c::I2cDriver::new(
            peripherals.i2c0,
            pins.gpio21,
            pins.gpio22,
            &i2c::I2cConfig::new().baudrate(1000.kHz().into()),
        )
        .unwrap(),
    );

    let (tx, rx) = std::sync::mpsc::channel::<Msg>();

    // Spawn display thread on core 1
    set_thread_spawn_configuration("display-thread\0", 8196, 5, Some(Core::Core1))?;
    let display_thread = std::thread::Builder::new()
        .stack_size(8196)
        .spawn(move || {
            let mut display = Ssd1306::new(di, DisplaySize128x32, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode();

            display
                .init()
                .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))
                .unwrap();

            // Logo
            display.clear(BinaryColor::Off).unwrap();
            let logotype: ImageRaw<BinaryColor> =
                ImageRaw::new(include_bytes!("../images/logotype.raw"), 128);

            let mut image = Image::new(&logotype, Point::new(0, 0));

            effects::up(&mut display, &mut image).unwrap();

            display.clear(BinaryColor::Off).unwrap();
            let character_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .line_height(LineHeight::Percent(150))
                .baseline(Baseline::Top)
                .build();

            let mut boot_text = Text::with_text_style(
                "Vogon Poetry Transceiver\nVersion: 1.0\nBooting..",
                Point::new(0, 0),
                character_style,
                text_style,
            );

            // Since the alignment is center, the bounding box is moved to the left,
            // so we move it to 0,0 and then translate it to the calculated center
            boot_text.translate_mut(screen_center(&boot_text) - boot_text.bounding_box().top_left);

            effects::blink(
                &mut display,
                &mut boot_text,
                3,
                std::time::Duration::from_millis(400),
            )
            .unwrap();

            boot_text.draw(&mut display).unwrap();
            display.flush().unwrap();

            log::info!("Booted");
            // Check for new messages
            let mut last_poetry = None;

            loop {
                log::info!("Waiting for message");
                let msg = rx.recv().unwrap();

                match msg {
                    Msg::Poetry(msg) => {
                        log::info!("Received message: {}", msg.text);
                        last_poetry = Some(msg);
                    }
                    Msg::RecvStats(stats) => {
                        log::info!("Received stats: {}", stats.received);
                    }
                    Msg::SendStats(stats) => {
                        log::info!("Sent stats: {}", stats.sent);
                    }
                }

                if let Some(ref poetry) = last_poetry {
                    // let mut text = Text::with_text_style(
                    //     &poetry.text,
                    //     Point::new(0, 0),
                    //     character_style,
                    //     text_style,
                    // );

                    // // Since the alignment is center, the bounding box is moved to the left,
                    // // so we move it to 0,0 and then translate it to the calculated center
                    // text.translate_mut(screen_center(&text) - text.bounding_box().top_left);

                    display.clear(BinaryColor::Off).unwrap();
                    effects::type_text(&mut display, &poetry.text).unwrap();
                }
                display.flush().unwrap();
            }
        })?;

    // TODO: Setup ESP-NOW

    // Spawn receiving thread that passes messages to the display thread
    set_thread_spawn_configuration("recv-thread\0", 8196, 15, None)?;
    let recv_thread = std::thread::Builder::new()
        .stack_size(8196)
        .spawn(move || {
            // In a loop create a hello world message that has an increased number
            // and send it to the display thread then wait for 1 second
            let mut i = 0;
            loop {
                let msg = Poetry {
                    text: "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF".to_string(),
                    src: 0,
                };
                tx.send(Msg::Poetry(msg)).unwrap();
                log::info!("Sent message");
                i += 1;
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        })?;

    display_thread.join().unwrap();
    recv_thread.join().unwrap();
    Ok(())
}
