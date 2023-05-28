use std::error::Error;
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Model: {}", DeviceInfo::new()?.model());

    Ok(())
}
