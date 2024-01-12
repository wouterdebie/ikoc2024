use anyhow::Result;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::mono_font::{ascii::FONT_5X7, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Baseline, LineHeight, Text, TextStyleBuilder};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::sys::esp;
use esp_idf_svc::espnow::{EspNow, PeerInfo, BROADCAST};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::i2c::I2cDriver;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::{i2c, peripherals::Peripherals};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_sys as _;
use rand::Rng;
use ssd1306::mode::{BufferedGraphicsMode, DisplayConfig};
use ssd1306::prelude::I2CInterface;
use ssd1306::{rotation::DisplayRotation, size::DisplaySize128x32, Ssd1306};
use std::sync::{Arc, Mutex};
use utils::{screen_center, set_thread_spawn_configuration};

use crate::utils::mac_to_string;

mod effects;
mod utils;

const DEVICE_ID: &str = env!("DEVICE_ID");
const TOTAL_DEVICES: u8 = 42;
const SCREEN_WIDTH: u32 = 128;
const SCREEN_HEIGHT: u32 = 32;
const ESP_NOW_CHANNEL: u8 = 1;

const SEND_DELAY_RANGE: std::ops::Range<u64> = 5..20;

const POETRY: &[u8; 18827] = include_bytes!("../assets/poetry.txt");
const ASCII_CHEWIE: &[u8; 2806] = include_bytes!("../assets/chewie.txt");
type Display = Ssd1306<
    I2CInterface<I2cDriver<'static>>,
    DisplaySize128x32,
    BufferedGraphicsMode<DisplaySize128x32>,
>;

struct Poem {
    id: u8,
    src: u8,
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Split POETRY on empty lines and put them in an array
    let poems: Vec<String> = std::str::from_utf8(POETRY)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let poems_len = poems.len();

    log::info!("\n{}", std::str::from_utf8(ASCII_CHEWIE).unwrap());

    log::info!("Chewbacchus 2023 - Vogon Poetry Transceiver");
    log::info!("by: Wouter de Bie - wouter@evenflow.nl");
    log::info!("Number of poems: {}", poems.len());
    log::info!("Device ID: {}/{}", DEVICE_ID, TOTAL_DEVICES);

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio22;
    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio4;

    let di = ssd1306::I2CDisplayInterface::new(
        i2c::I2cDriver::new(
            peripherals.i2c0,
            sda,
            scl,
            &i2c::I2cConfig::new().baudrate(1000.kHz().into()),
        )
        .unwrap(),
    );

    let (tx, rx) = std::sync::mpsc::channel::<Poem>();

    let received = Arc::new(Mutex::new(0));
    let sent = Arc::new(Mutex::new(0));

    // Spawn display thread on core 1
    set_thread_spawn_configuration("display-thread\0", 8196, 5, Some(Core::Core1))?;
    let display_received = received.clone();
    let display_sent = sent.clone();
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
                ImageRaw::new(include_bytes!("../assets/logotype.raw"), 128);

            let mut image = Image::new(&logotype, Point::new(0, 0));

            // Show logo 3 times
            for _ in 0..3 {
                effects::up_in(&mut display, &mut image).unwrap();
                // std::thread::sleep(std::time::Duration::from_secs(1));
                effects::up_out(&mut display, &mut image).unwrap();
            }

            display.clear(BinaryColor::Off).unwrap();
            let character_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .line_height(LineHeight::Percent(150))
                .baseline(Baseline::Top)
                .build();

            let mut sub_logo_text = Text::with_text_style(
                "XIII\nNothing To See Here",
                Point::new(0, 0),
                character_style,
                text_style,
            );
            sub_logo_text.translate_mut(
                screen_center(&sub_logo_text) - sub_logo_text.bounding_box().top_left,
            );
            sub_logo_text.draw(&mut display).unwrap();
            display.flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(4));

            let mut boot_text = Text::with_text_style(
                "Vogon Poetry Transceiver\nVersion: 0x42\nBooting..",
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
                std::time::Duration::from_millis(500),
                true,
            )
            .unwrap();

            std::thread::sleep(std::time::Duration::from_secs(2));
            // system time now
            let mut last_received = std::time::SystemTime::now();
            loop {
                display.clear(BinaryColor::Off).unwrap();
                let wait = format!(
                    "Device {}/{}\nWaiting for Poetry..\nReceived: {}, sent: {}",
                    DEVICE_ID,
                    TOTAL_DEVICES,
                    display_received.lock().unwrap(),
                    display_sent.lock().unwrap()
                );
                let mut wait_text =
                    Text::with_text_style(&wait, Point::new(0, 0), character_style, text_style);
                wait_text
                    .translate_mut(screen_center(&wait_text) - wait_text.bounding_box().top_left);
                wait_text.draw(&mut display).unwrap();
                display.flush().unwrap();

                // Sleep 2 second
                std::thread::sleep(std::time::Duration::from_secs(4));

                // Wait for messages, but timeout after 1 second
                let msg = rx.recv_timeout(std::time::Duration::from_secs(1));

                if let Ok(poem) = msg {
                    last_received = std::time::SystemTime::now();
                    let src = poem.src;
                    display_poem(
                        poem,
                        &mut display,
                        &poems,
                        format!("Received from: {}/{}\n", src, TOTAL_DEVICES).as_str(),
                    );
                }
                // duration since last message
                else if std::time::SystemTime::now()
                    .duration_since(last_received)
                    .unwrap()
                    .as_secs()
                    > 10
                {
                    display.clear(BinaryColor::Off).unwrap();
                    log::info!("No poem received in the last 10 seconds..");
                    let mut no_poem_text = Text::with_text_style(
                        "No poem received in\nthe last 10 seconds..\nRandomly picking one..",
                        Point::new(0, 0),
                        character_style,
                        text_style,
                    );
                    no_poem_text.translate_mut(
                        screen_center(&no_poem_text) - no_poem_text.bounding_box().top_left,
                    );
                    no_poem_text.draw(&mut display).unwrap();
                    display.flush().unwrap();

                    std::thread::sleep(std::time::Duration::from_secs(4));

                    // Pick random poem
                    let poem_id = rand::thread_rng().gen_range(0..poems_len) as u8;
                    display_poem(
                        Poem {
                            id: poem_id,
                            src: DEVICE_ID.parse().unwrap(),
                        },
                        &mut display,
                        &poems,
                        "Random poem:\n",
                    );
                }
            }
        })?;

    // Setup ESP-NOW
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut wifi = Box::new(EspWifi::new(peripherals.modem, sysloop, Some(nvs)).unwrap());

    esp!(unsafe { esp_idf_sys::esp_wifi_set_mode(esp_idf_sys::wifi_mode_t_WIFI_MODE_STA) })
        .unwrap();

    esp!(unsafe {
        esp_idf_sys::esp_wifi_set_protocol(
            esp_idf_sys::wifi_interface_t_WIFI_IF_STA,
            esp_idf_sys::WIFI_PROTOCOL_LR.try_into().unwrap(),
        )
    })
    .unwrap();

    wifi.start()?;

    let esp_now = Arc::new(EspNow::take().unwrap());
    esp_now
        .add_peer(PeerInfo {
            peer_addr: BROADCAST,
            channel: ESP_NOW_CHANNEL,
            ifidx: 0,
            encrypt: false,
            ..Default::default()
        })
        .unwrap();

    let tx_recv = tx.clone();
    let esp_now_recv_cb = move |src: &[u8], data: &[u8]| {
        log::info!("Data recv from {}, len {}", mac_to_string(src), data.len());
        let recv_data = Poem {
            id: data[0],
            src: data[1],
        };
        tx_recv.send(recv_data).unwrap();
        let mut r = received.lock().unwrap();
        *r += 1;
    };
    esp_now.register_recv_cb(esp_now_recv_cb).unwrap();

    set_thread_spawn_configuration("send-thread\0", 8196, 15, None)?;
    let espnow_recv = esp_now.clone();
    let send_thread = std::thread::Builder::new()
        .stack_size(8196)
        .spawn(move || {
            let rng = &mut rand::thread_rng();

            let mut led = PinDriver::output(led).unwrap();
            led.set_high().unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(
                    rng.gen_range(SEND_DELAY_RANGE),
                ));

                let poem_id = rng.gen_range(0..poems_len) as u8;
                let device_id: u8 = DEVICE_ID.parse().unwrap();

                let payload: [u8; 2] = [poem_id, device_id];
                espnow_recv.send(BROADCAST, &payload).unwrap();

                log::info!("Broadcast poem {} from {}", poem_id, DEVICE_ID);
                let mut s = sent.lock().unwrap();
                *s += 1;

                // Blink gpio 21 three times
                for _ in 0..3 {
                    led.set_low().unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    led.set_high().unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
        })?;
    send_thread.join().unwrap();

    display_thread.join().unwrap();
    Ok(())
}

fn display_poem(poem: Poem, display: &mut Display, poems: &[String], intro_text: &str) {
    log::info!("Displaying poem id: {}, from {}", poem.id, poem.src);
    display.clear(BinaryColor::Off).unwrap();
    let text = &poems[poem.id as usize];
    let s = format!("{}{}", intro_text, text);
    effects::type_text(display, &s).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
}
