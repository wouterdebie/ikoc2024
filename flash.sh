#!/usr/bin/env bash
DEVICE_ID=1 cargo run --release -- -p /dev/cu.usbserial-1430 -b 256000
