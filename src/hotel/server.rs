use tonic::{transport::Server, Request, Response, Status};
use std::sync::{Arc, Mutex};
use hotel::hotel_service_server::{HotelService, HotelServiceServer};
use hotel::{HotelData, HotelResponse};

pub mod hotel {
    tonic::include_proto!("hotel");
}

#[derive(Clone, Default, Debug)]
pub struct Processed {
    pub count: Arc<Mutex<u64>>,
}
impl Processed {
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0))
        }
    }
    pub fn increment(&self) {
        let mut lock = self.count.lock().unwrap();
        *lock += 1;
    }
    pub fn current(&self) -> u64 {
        let lock = self.count.lock().unwrap();
        lock.clone()
    }
}


#[derive(Debug, Default)]
pub struct MyHotelService {
    count: Processed
}
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
        
        self.count.increment();
        let current = self.count.current();
        for sensor in data.sensors {
            let sensor_1 = (sensor.sensor_values >> 4) & 0xF;
            let sensor_2 = sensor.sensor_values & 0xF;
            print!("\r{:08b},{:08b},{:08b},{:08b},{:08b} {:?}", hotel_id, room_id, device_id, sensor_1, sensor_2, current);
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

    println!("HotelServiceServer!");

    Server::builder()
        .add_service(HotelServiceServer::new(hotel_service))
        .serve(addr)
        .await?;

    Ok(())
}

