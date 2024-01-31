use devices::Devices;

fn main() {
    match Devices::usb() {
        Ok(devices) => {
            for device in devices {
                println!("Device: {:?}", device);
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
