use rppal::gpio::Level::{High, Low};
use rppal::gpio::Mode::{Input, Output};
use rppal::gpio::{Error as GpioError, IoPin, Level, Pin};
use std::result::Result;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct DHT11 {
    pin: IoPin,
}

impl DHT11 {
    pub fn new(pin: Pin) -> DHT11 {
        DHT11 {
            pin: pin.into_io(Output),
        }
    }

    pub fn read(&mut self) -> Result<Measure, Error> {
        let mut pin = &mut self.pin;

        //  ハンドシェイク
        pin.set_mode(Output);
        pin.set_high_and_sleep(Duration::from_micros(5))?;
        pin.set_low_and_sleep(Duration::from_millis(20))?;
        pin.set_high();

        let mut bytes = [0u8; 5];
        pin.set_mode(Input);

        //
        pin.wait_while(High)?;
        pin.wait_while(Low)?;
        pin.wait_while(High)?;

        //  読み込み
        for byte in bytes.iter_mut() {
            for _ in 0..8 {
                *byte <<= 1;
                pin.wait_while(Low)?;
                if pin.wait_while(High)? > Duration::from_micros(30) {
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
            Err(Error::CheckSum)
        }
    }
}

#[derive(Debug)]
pub struct Measure {
    temperature: u8,
    humidity: u8,
}

#[derive(Debug)]
pub enum Error {
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
    fn wait_while(&self, level: Level) -> Result<Duration, Error>;
}

impl Wait for IoPin {
    fn wait_while(&self, level: Level) -> Result<Duration, Error> {
        let start = Instant::now();
        let end: Instant = {
            while self.read() == level {
                if Instant::now() - start > Duration::from_millis(250) {
                    return Err(Error::TimeOut);
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
