use crate::hotel_tools::Hotel;
use rand::Rng;

pub fn simulate(hotels: &mut Vec<Hotel>) {
    let mut rng = rand::thread_rng();
    let mut prog = 0;
    for hotel in hotels.iter_mut() {
        for room in hotel.rooms.iter_mut() {
            for device in room.devices.iter_mut() {
                for sensor in device.sensors.iter_mut() {
                    sensor.value1 = rng.gen_range(0..=255);
                    sensor.value2 = rng.gen_range(0..=255);
                    prog += 1;
                    print!("\r{:?}", prog);
                }
            }
        }
    }
    println!(" # of sensors simulated!");
}
