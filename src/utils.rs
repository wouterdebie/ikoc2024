use embedded_graphics::geometry::{Dimensions, Point};
use esp_idf_hal::{sys::EspError, task::thread::ThreadSpawnConfiguration};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn set_thread_spawn_configuration(
    name: &'static str,
    stack_size: usize,
    prio: u8,
    pin_to_core: Option<esp_idf_hal::cpu::Core>,
) -> Result<(), EspError> {
    ThreadSpawnConfiguration {
        name: Some(name.as_bytes()),
        stack_size,
        priority: prio,
        pin_to_core,
        ..Default::default()
    }
    .set()
}

pub fn screen_center_x<D>(d: &D) -> i32
where
    D: Dimensions,
{
    ((SCREEN_WIDTH / 2) as i32) - ((d.bounding_box().size.width / 2) as i32)
}

pub fn screen_center_y<D>(d: &D) -> i32
where
    D: Dimensions,
{
    ((SCREEN_HEIGHT / 2) as i32) - ((d.bounding_box().size.height / 2) as i32)
}

pub fn screen_center<D>(d: &D) -> Point
where
    D: Dimensions,
{
    Point::new(screen_center_x(d), screen_center_y(d))
}

pub fn mac_to_string(mac: &[u8]) -> String {
    let mut mac_str = String::new();
    for i in 0..mac.len() {
        mac_str.push_str(&format!("{:02x}", mac[i]));
        if i < mac.len() - 1 {
            mac_str.push(':');
        }
    }
    mac_str
}
