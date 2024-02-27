use dualshock_driver::{DualShock4Driver, BLE};
use serialport;
use serde::{Serialize, Deserialize};
use serde_json;
use async_std;

#[derive(Serialize, Deserialize)]
struct Cmd
{
    pub x:f32,
    pub y:f32,
    pub ro:f32,
}

#[async_std::main]
async fn main() {
    let mut ds = DualShock4Driver::new().unwrap();
    
    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(std::time::Duration::from_millis(100))
        .open().unwrap();

    loop {
        let con_input = ds.read(BLE).await.unwrap();

        let cmd = Cmd{x:-0.5*con_input.sticks.left_x, y:0.5*con_input.sticks.left_y, ro:-0.5*con_input.sticks.right_x};

        let msg = serde_json::to_string(&cmd).unwrap();

        match port.write(msg.as_bytes()) {
            Ok(_)=>{
                println!("{}", msg);
            }
            Err(_)=>{

            }
        }
    }
}
