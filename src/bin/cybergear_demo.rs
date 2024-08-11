use can_motor_rs::cybergear::CyberGear;
use can_motor_rs::CanBuffer;
use futures_timer::Delay;
use futures_util::StreamExt;
use socketcan::{tokio::CanSocket, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut socket = CanSocket::open("can0").unwrap();

    let mut cyber_gear = CyberGear::default();

    let frame = cyber_gear.enable_torque().as_socketcan_can_frame().unwrap();
    socket.write_frame(frame)?.await?;

    loop {
        let frame = cyber_gear.run_torque(0.0).as_socketcan_can_frame().unwrap();
        socket.write_frame(frame)?.await?;

        println!("{:?}", frame);

        if let Some(buf) = socket.next().await {
            match buf {
                Ok(frame) => {
                    let can_buffer = CanBuffer::from_socketcan_can_frame(frame).unwrap();
                    cyber_gear.receive(&can_buffer);
                    println!("{:?}", cyber_gear.feedback().clone());
                }
                Err(err) => eprintln!("Error: {:?}", err),
            }
        }

        Delay::new(Duration::from_millis(10)).await?;
    }
}
