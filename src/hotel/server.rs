use tonic::{transport::Server, Request, Response, Status};
use hotel::hotel_service_server::{HotelService, HotelServiceServer};
use hotel::{HotelData, HotelResponse, SensorData};

pub mod hotel {
    tonic::include_proto!("hotel"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyHotelService {}

#[tonic::async_trait]
impl HotelService for MyHotelService {
    async fn send_data(
        &self,
        request: Request<HotelData>,
    ) -> Result<Response<HotelResponse>, Status> {
        let data = request.into_inner();

        let hotel_id = (data.hotel_room_device >> 24) & 0xFF;
        let room_id = (data.hotel_room_device >> 16) & 0xFF;
        let device_id = (data.hotel_room_device >> 8) & 0xFF;

        println!(
            "Received data from Hotel ID: {}, Room ID: {}, Device ID: {}",
            hotel_id, room_id, device_id
        );

        for sensor in data.sensors {
            let sensor_1 = (sensor.sensor_values >> 4) & 0xF;
            let sensor_2 = sensor.sensor_values & 0xF;

            println!(
                "Sensor Value 1: {}, Sensor Value 2: {}",
                sensor_1, sensor_2
            );
        }

        let reply = hotel::HotelResponse {
            message: format!("Data received from Hotel ID: {}", hotel_id),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let hotel_service = MyHotelService::default();

    println!("HotelServiceServer listening on {}", addr);

    Server::builder()
        .add_service(HotelServiceServer::new(hotel_service))
        .serve(addr)
        .await?;

    Ok(())
}

