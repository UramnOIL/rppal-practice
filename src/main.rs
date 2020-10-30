mod dht11;

use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;

const GPIO_PIN_DHT11: u8 = 23;

fn main() {
    let mut dht11 = dht11::new(Gpio::new().unwrap().get(GPIO_PIN_DHT11).unwrap());

    loop {
        println!("{:?}", dht11.read().unwrap());
        sleep(Duration::from_secs(1));
    }
}
