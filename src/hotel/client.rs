use hello_tonic::hotel_simulator::simulate;
use hello_tonic::hotel_tools::{generate, Config};
use hotel::hotel_service_client::HotelServiceClient;
use hotel::{HotelData, SensorData};
use tonic::Request;

pub mod hotel {
    tonic::include_proto!("hotel");
}

struct Value(u32);

impl Value {
    fn new(val: u32) -> Self {
        Value(val)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032b}", self.0)
    }
}

struct Packet {
    ids: Value,
    sensors: Value,
}

impl Packet {
    fn new(val1: u32, val2: u32) -> Self {
        Packet {
            ids: Value::new(val1),
            sensors: Value::new(val2),
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ids, self.sensors)
    }
}

fn pack_ids(hotel_id: u8, room_id: u8, device_id: u8) -> u32 {
    ((hotel_id as u32) << 24) | ((room_id as u32) << 16) | ((device_id as u32) << 8)
}

fn pack_sensor_values(sensor_1: u8, sensor_2: u8) -> u32 {
    ((sensor_1 as u32) << 4) | (sensor_2 as u32)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HotelServiceClient::connect("http://[::1]:50051").await?;
    println!("HotelServiceClient!");
    let mut prog: u64 = 0;
    let config = Config {
        hotels: 100,
        rooms: 50,
        devices: 2,
    };
    let mut hotels = generate(config);

    simulate(&mut hotels);

    for hotel in hotels {
        for room in hotel.rooms {
            for device in room.devices {
                let hotel_room_device = pack_ids(hotel.hotel_id, room.room_id, device.device_id);
                let sensors: Vec<SensorData> = device
                    .sensors
                    .iter()
                    .map(|sensor| {
                        let sensor_values = pack_sensor_values(sensor.value1, sensor.value2);
                        let packet = Packet::new(hotel_room_device, sensor_values);
                        prog += 1;
                        print!("\r{}\t{}", packet, prog);
                        //print!("\x1B[2J\x1B[1;1H");
                        SensorData { sensor_values }
                    })
                    .collect();
                let request = Request::new(HotelData {
                    hotel_room_device,
                    sensors,
                });
                client.send_data(request).await?;
            }
        }
    }

    Ok(())
}
