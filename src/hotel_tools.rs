use rand::Rng;
use std::fmt;

pub struct Config {
    pub hotels: usize,
    pub rooms: usize,
    pub devices: usize,
}

pub struct Hotel {
    pub hotel_id: u8,
    pub rooms: Vec<Room>,
}

pub struct Room {
    pub room_id: u8,
    pub devices: Vec<Device>,
}

pub struct Device {
    pub device_id: u8,
    pub sensors: Vec<Sensor>,
}

pub struct Sensor {
    pub leg_id: u8,
    pub value1: u8,
    pub value2: u8,
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}{:08b}{:08b}", self.leg_id, self.value1, self.value2)
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.device_id)?;
        for sensor in &self.sensors {
            write!(f, "{}", sensor)?;
        }
        Ok(())
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.room_id)?;
        for device in &self.devices {
            write!(f, "{}", device)?;
        }
        Ok(())
    }
}

impl fmt::Display for Hotel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.hotel_id)?;
        for room in &self.rooms {
            write!(f, "{}", room)?;
        }
        Ok(())
    }
}

pub fn generate(config: Config) -> Vec<Hotel> {
    let mut hotels = Vec::with_capacity(config.hotels);
    for hotel_id in 0..config.hotels as u8 {
        let mut rooms = Vec::with_capacity(config.rooms);
        for room_id in 0..config.rooms as u8 {
            let mut devices = Vec::with_capacity(config.devices);
            for device_id in 0..config.devices as u8 {
                let sensors = vec![
                    Sensor { leg_id: 0, value1: rand::thread_rng().gen(), value2: rand::thread_rng().gen() },
                    Sensor { leg_id: 1, value1: rand::thread_rng().gen(), value2: rand::thread_rng().gen() },
                ];
                devices.push(Device { device_id, sensors });
            }
            rooms.push(Room { room_id, devices });
        }
        hotels.push(Hotel { hotel_id, rooms });
    }
    hotels
}
