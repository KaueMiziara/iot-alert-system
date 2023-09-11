#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, i2c, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
    IO,
};
use mpu6050::*;

use rs_esp32_simple_preventive_maintenance_example::absolute::Absolute;
use rs_esp32_simple_preventive_maintenance_example::alarm::alarm;
use rs_esp32_simple_preventive_maintenance_example::limit::Limit;

const MECHANICAL_LIMIT: f32 = 0.8;
const TEMPERATURE_LIMIT: f32 = 2.5;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

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

    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let (mut internal_led, mut buzzer, sda, scl) = (
        io.pins.gpio2.into_push_pull_output(),
        io.pins.gpio33.into_push_pull_output(),
        io.pins.gpio21,
        io.pins.gpio22,
    );

    let i2c = i2c::I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    delay.delay_ms(255u8);

    let mut mpu = Mpu6050::new(i2c);
    mpu.init(&mut delay)
        .expect("Error while initializing MPU6050");

    let mut acc_ref = mpu.get_acc();
    let temp_ref = mpu.get_temp();

    let mut reset_reference = true;

    println!("---");
    loop {
        if reset_reference {
            acc_ref = mpu.get_acc();

            reset_reference = false;
            delay.delay_ms(100u8);
        } else {
            let acc = mpu.get_acc();
            let gyro = mpu.get_gyro();
            let temp = mpu.get_temp();

            match acc {
                Ok(data) => {
                    println!("Accelerometer:");
                    println!("Ax: {} m/s^2", data[0]);
                    println!("Ay: {} m/s^2", data[1]);
                    println!("Az: {} m/s^2", data[2]);

                    let acc_ref_x = acc_ref.as_ref().unwrap()[0];

                    let mut delta = data[0] - acc_ref_x;

                    if delta.abs() >= MECHANICAL_LIMIT {
                        println!("MECHANICAL STRESS DETECTED!");
                        println!("Current: {}", data[0]);
                        println!("Reference: {}", acc_ref_x);
                        println!("Delta: {}", delta);

                        alarm(
                            &mut buzzer,
                            &mut internal_led,
                            &Limit::Mechanical,
                            &mut delay,
                        );
                    }
                }
                Err(_) => panic!("Error reading data from the accelerometer"),
            };

            match gyro {
                Ok(data) => {
                    println!("Gyroscope:");
                    println!("Gx: {} rad/s", data[0]);
                    println!("Gy: {} rad/s", data[1]);
                    println!("Gz: {} rad/s", data[2]);
                }
                Err(_) => panic!("Error reading data from the gyroscope"),
            };

            match temp {
                Ok(data) => {
                    println!("Temperature:\n{} ºC", data);

                    let temp_ref = temp_ref.as_ref().unwrap();
                    let mut delta = data - temp_ref;

                    if delta.abs() >= TEMPERATURE_LIMIT {
                        println!("OVERHEATING DETECTED");
                        println!("Current: {}", data);
                        println!("Reference: {}", temp_ref);
                        println!("Delta: {}", delta);

                        alarm(
                            &mut buzzer,
                            &mut internal_led,
                            &Limit::Temperature,
                            &mut delay,
                        );
                    }
                }
                Err(_) => panic!("Error reading data from the temperature sensor"),
            }

            println!("---");

            reset_reference = true;
            delay.delay_ms(500u16);
        }
    }
}
