mod dht11;

use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut dht11 = dht11::new(Gpio::new().unwrap().get(23).unwrap());

    loop {
        println!("{:?}", dht11.read().unwrap());
        sleep(Duration::from_secs(1));
    }
}
