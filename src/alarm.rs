use crate::limit::Limit;
use hal::{prelude::*, Delay};

pub fn alarm(
    buzzer: &mut hal::gpio::GpioPin<
        hal::gpio::Output<hal::gpio::PushPull>,
        hal::gpio::Bank1GpioRegisterAccess,
        hal::gpio::DualCoreInteruptStatusRegisterAccessBank1,
        hal::gpio::InputOutputAnalogPinType,
        hal::gpio::Gpio33Signals,
        33,
    >,
    led: &mut hal::gpio::GpioPin<
        hal::gpio::Output<hal::gpio::PushPull>,
        hal::gpio::Bank0GpioRegisterAccess,
        hal::gpio::DualCoreInteruptStatusRegisterAccessBank0,
        hal::gpio::InputOutputAnalogPinType,
        hal::gpio::Gpio2Signals,
        2,
    >,
    limit: &Limit,
    delay: &mut Delay,
) {
    let buzzes: u8 = match limit {
        Limit::Mechanical => 3,
        Limit::Temperature => 9,
    };

    for _ in 0..buzzes {
        buzzer.set_high().unwrap();
        led.set_high().unwrap();

        alarm_time(limit, delay);

        buzzer.set_low().unwrap();
        led.set_low().unwrap();

        alarm_time(limit, delay);
    }
}

fn alarm_time(limit: &Limit, delay: &mut Delay) {
    match limit {
        Limit::Mechanical => delay.delay_ms(100u8),
        Limit::Temperature => delay.delay_ms(50u8),
    }
}
