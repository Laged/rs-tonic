use tonic::{Request, transport::Channel};
use hotel::hotel_service_client::HotelServiceClient;
use hotel::{HotelData, SensorData};
use hello_tonic::hotel_tools::{Config, generate};
use hello_tonic::hotel_simulator::simulate;

pub mod hotel {
    tonic::include_proto!("hotel");
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

    let config = Config { hotels: 100, rooms: 100, devices: 2 };
    let mut hotels = generate(config);

    simulate(&mut hotels);

    for hotel in hotels {
        for room in hotel.rooms {
            for device in room.devices {
                let hotel_room_device = pack_ids(hotel.hotel_id, room.room_id, device.device_id);
                let sensors: Vec<SensorData> = device.sensors.iter().map(|sensor| {
                    let sensor_values = pack_sensor_values(sensor.value1, sensor.value2);
                    SensorData { sensor_values }
                }).collect();

                let request = Request::new(HotelData {
                    hotel_room_device,
                    sensors,
                });

                let response = client.send_data(request).await?;
                println!("RESPONSE={:?}", response);
            }
        }
    }

    Ok(())
}

