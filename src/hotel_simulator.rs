use rand::Rng;
use crate::hotel_tools::{Hotel, Sensor};

pub fn simulate(hotels: &mut Vec<Hotel>) {
    let mut rng = rand::thread_rng();

    for hotel in hotels.iter_mut() {
        for room in hotel.rooms.iter_mut() {
            for device in room.devices.iter_mut() {
                for sensor in device.sensors.iter_mut() {
                    sensor.value1 = rng.gen_range(0..=15); // 4-bit value
                    sensor.value2 = rng.gen_range(0..=15); // 4-bit value
                }
            }
        }
    }
}

