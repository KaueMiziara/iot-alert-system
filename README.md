# Preventive Maintenance Alert System on ESP32
---

## About the Project
This project utilizes an ESP32 microcontroller to monitor data from an accelerometer and a temperature sensor.
The main goal is to sound an alarm if the readings exceed predefined limits, indicating potential issues with the monitored equipment.

A similar concept could be applied to monitor machines, alerting the staff in case of overheating or mechanical stresses, indicating possible malfunctions before
the equipment becomes inoperable.  

The two limits that trigger alarms are:

- **Mechanical Stress Limit**: If the mechanical stress on the equipment exceeds a certain threshold (defined as MECHANICAL_LIMIT), an alarm is triggered. This could be useful to detect excessive vibrations or impacts.

- **Temperature Limit**: If the temperature reading goes beyond a specified limit (defined as TEMPERATURE_LIMIT), an alarm is sounded. This helps identify overheating issues.

## How to Use
To replicate this project or adapt it for your specific use case, follow these steps:

### Hardware Setup
Connect the accelerometer and temperature sensor to the ESP32 following diagram provided below.
Make sure all connections are secure.

### Software Setup
Make sure to have Rust and Cargo [installed](https://www.rust-lang.org/tools/install).

Follow the steps in [The Rust on ESP3 Book](https://esp-rs.github.io/book/) to setup the toolchain.

### Compile and Flash
Once you have the environment set up, clone this repository and try to build it.

```bash
git clone https://github.com/KaueMiziara/esp32-obd-scan-sim.git

cargo build
```

- Note that the first build will take some time.

If it builds sucessfully, you may flash it into you ESP32.
The following example flashes the code in a board connected to [PORT] and opens the serial monitor to check for alerts and sensor readings:

```bash
cargo espflash flash -p [PORT] --monitor
```

## Customization
You can customize this project to suit your needs by adjusting the following parameters in the code:

- **MECHANICAL_LIMIT**: Change this value to set the threshold for detecting mechanical stress.
- **TEMPERATURE_LIMIT**: Modify this value to set the temperature limit for triggering alarms.
- **GPIO**: Adapt the pins and IO modes to your use case.

Feel free to adapt the code and hardware connections to meet the requirements of your specific application.

## Diagram (made using Fritzing)
Below are the schematic diagram illustrating the hardware connections:

![PreventiveMaintenanceSketch_schem](https://github.com/KaueMiziara/rs-esp32-simple-preventive-maintenance-example/assets/119542829/4bbb3d78-836c-4b30-9927-61ade57e1f59)

## Serial monitor output example
For reference, here's an example of the output you can expect to see in the serial monitor when running this project:

![ESP32-preventive-maintenance-serialmonitor](https://github.com/KaueMiziara/rs-esp32-simple-preventive-maintenance-example/assets/119542829/af8d09ee-ff44-432c-b906-2138424c6258)

## Credits
This project was developed by [Kauê Miziara](https://linkedin.com/in/kauemiziara/) as part of a Computer Engineering class.

## License
This project is dual-licensed under the [Apache License 2.0](LICENSE-APACHE) and the [MIT License](LICENSE-MIT) - see the respective license files for details.
