# Chewbacchus 2024 Throw

## Description
This is the code for throws I made for the Chewbaccus 2024 parade. It's a
Vogon Poetry transceiver that sends and receives poetry from and to other
devices. It's based on an esp32 board and uses an SSd1306 OLED display. This project is written
in Rust and uses the esp-idf framework.

Demo: https://photos.app.goo.gl/udVBA6jNoyL7UNUZA

## Building and installing
Installing and running: `DEVICE_ID=1 cargo run --release -- -p /dev/cu.usbserial-1410 -b 256000`.
Make sure to change the device ID to a unique number for each device. Secondly, make sure you have
an esp32 toolchain with `espup` installed.

## Inner workings

Devices have a hardcoded list of 42 poems that they can send and receive. Devices use ESP-NOW to
broadcast "poems" to one another. The protocol is very simple: a device sends a packet with two
bytes; a POEM_ID (0..41) and their DEVICE_ID (1..42). Other devices are listening and display
the sent poem as soon as they receive them.

If a device hasn't received a poem in 10 seconds, it will pick a random poem.

## Hardware
For the project I used an [AITIP ESP32 Lite v1.0.0](https://www.amazon.com/gp/product/B0BCJT8KDX/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&th=1) and a [Makerfocus SSD1306 OLED Display](https://www.amazon.com/gp/product/B08LQM9PQQ/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1). I connected pin 0 to SDA and pin 4 to SCL.

## Thanks

- Intergalactic Krewe of Chewbacchus for organizing the parade. And their Overlords for awarding me with the best throw award.
- r/vogonpoetrycircle the poems that aren't written by Douglas Adams.
