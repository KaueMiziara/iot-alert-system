#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, i2c, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
    IO,
};
use mpu6050::*;

// Compile, flash and run:
// source ~/export-esp.sh
// cargo espflash --release --monitor

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Initialize Delay
    let mut delay = Delay::new(&clocks);

    // Initialize IO && Pin definitions
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let (mut internal_led, mut buzzer, sda, scl) = (
        io.pins.gpio2.into_push_pull_output(),
        io.pins.gpio33.into_push_pull_output(),
        io.pins.gpio21,
        io.pins.gpio22,
    );

    // Configure I2C
    let i2c = i2c::I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    delay.delay_ms(255u8);

    // Initialize MPU6050 module
    let mut mpu = Mpu6050::new(i2c);
    mpu.init(&mut delay)
        .expect("Error while initializing MPU6050");

    loop {}
}
