mod dht11;

use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;

const GPIO_PIN_DHT11: u8 = 23;

fn main() -> Result<(), rppal::gpio::Error>{
    let mut dht11 = dht11::DHT11::new(Gpio::new()?.get(GPIO_PIN_DHT11)?);
    loop {
        println!("{:?}", dht11.read());
        sleep(Duration::from_secs(10));
    }
}
