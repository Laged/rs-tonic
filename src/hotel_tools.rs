use rand::Rng;

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

pub fn generate(config: Config) -> Vec<Hotel> {
    let mut hotels = Vec::with_capacity(config.hotels);
    
    for hotel_id in 0..config.hotels as u8 {
        let mut rooms = Vec::with_capacity(config.rooms);
        
        for room_id in 0..config.rooms as u8 {
            let mut devices = Vec::with_capacity(config.devices);
            
            for device_id in 0..config.devices as u8 {
                let sensors = vec![
                    Sensor { leg_id: 0, value1: 0, value2: 0 },
                    Sensor { leg_id: 1, value1: 0, value2: 0 },
                ];
                
                devices.push(Device { device_id, sensors });
            }
            
            rooms.push(Room { room_id, devices });
        }
        
        hotels.push(Hotel { hotel_id, rooms });
    }
    
    hotels
}

