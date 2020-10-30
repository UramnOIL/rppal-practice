use crate::Error::{CheckSum, TimeOut};
use rppal::gpio::Level::{High, Low};
use rppal::gpio::Mode::{Input, Output};
use rppal::gpio::{Error as GpioError, Gpio, IoPin, Level, Pin};
use std::result::Result;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let mut dht11 = DHT11::new(Gpio::new().unwrap().get(23).unwrap());

    loop {
        println!("{:?}", dht11.read().unwrap());
        sleep(Duration::from_secs(1));
    }
}

struct DHT11 {
    pin: IoPin,
}

#[derive(Debug)]
enum Error {
    Gpio(GpioError),
    TimeOut,
    CheckSum,
}

impl From<GpioError> for Error {
    fn from(e: GpioError) -> Error {
        Error::Gpio(e)
    }
}

trait Wait {
    fn wait(&self, level: Level) -> Result<Duration, Error>;
}

impl Wait for IoPin {
    fn wait(&self, level: Level) -> Result<Duration, Error> {
        let start = Instant::now();
        let end: Instant = {
            while self.read() == level {
                if Instant::now() - start > Duration::from_millis(250) {
                    return Err(TimeOut);
                }
            }
            Instant::now()
        };
        Ok(end - start)
    }
}

trait SetLevelAndSleep {
    fn set_low_and_sleep(&mut self, duration: Duration) -> Result<(), Error>;

    fn set_high_and_sleep(&mut self, duration: Duration) -> Result<(), Error>;
}

impl SetLevelAndSleep for IoPin {
    fn set_low_and_sleep(&mut self, duration: Duration) -> Result<(), Error> {
        self.set_low();
        sleep(duration);
        Ok(())
    }

    fn set_high_and_sleep(&mut self, duration: Duration) -> Result<(), Error> {
        self.set_high();
        sleep(duration);
        Ok(())
    }
}

impl DHT11 {
    fn new(pin: Pin) -> DHT11 {
        DHT11 {
            pin: pin.into_io(Output),
        }
    }

    fn read(&mut self) -> Result<Measure, Error> {
        let mut pin = &mut self.pin;

        pin.set_mode(Output);
        pin.set_low_and_sleep(Duration::from_millis(18))?;
        pin.set_high_and_sleep(Duration::from_micros(40))?;

        let mut bytes = [0u8; 5];
        pin.set_mode(Input);

        for byte in bytes.iter_mut() {
            for _ in 0..8 {
                *byte <<= 1;
                pin.wait(Low)?;
                if pin.wait(High)? > Duration::from_micros(30) {
                    *byte |= 1;
                }
            }
        }

        let sum: u16 = bytes.iter().map(|byte| *byte as u16).sum();
        if bytes[4] as u16 == sum & 0x00FF {
            Ok(Measure {
                temperature: bytes[2],
                humidity: bytes[0],
            })
        } else {
            Err(CheckSum)
        }
    }
}

#[derive(Debug)]
struct Measure {
    temperature: u8,
    humidity: u8,
}
