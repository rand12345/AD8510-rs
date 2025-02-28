# AS8510-rs

[![Crates.io](https://img.shields.io/crates/v/as8510.svg)](https://crates.io/crates/as8510)
[![Docs.rs](https://docs.rs/as8510/badge.svg)](https://docs.rs/as8510)
[![License](https://img.shields.io/crates/l/as8510.svg)](https://github.com/yourusername/as8510/blob/main/LICENSE)

A `#![no_std]` async Rust driver for the [AS8510 current and voltage sensor](https://www.mouser.co.uk/datasheet/2/588/asset_pdf_25493221-3419315.pdf), designed for embedded systems using the `embedded-hal-async` SPI interface. This crate provides an easy-to-use API to read bi-directional current and voltage measurements with configurable gain settings.

The AS8510 is a high-precision data acquisition IC commonly used in automotive and industrial applications for current sensing over a shunt resistor and voltage measurements.

## Features

- Asynchronous API using `embedded-hal-async` for SPI communication.
- Supports configurable current gain: `Gain1`, `Gain25`, `Gain40`, `Gain100`.
- Supports configurable voltage gain: `Gain25`, `Gain40`.
- Current measurement ranges:
  - `Gain1`: +2076A / -1523A
  - `Gain25`: ±400A
  - `Gain40`: ±235A
  - `Gain100`: ±77A
- `#![no_std]` compatible for bare-metal embedded environments.
- Error handling for SPI communication and device status.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
as8510 = "0.1.0"
embedded-hal-async = "1.0"
```

Ensure your target platform provides an implementation of `embedded_hal_async::spi::SpiDevice`.

## Usage

Below is an example using the `esp-hal` crate for an ESP32-based setup. Adjust the SPI configuration and runtime according to your hardware and executor (e.g., `embassy_executor`).

```rust
use as8510::{As8510, Gain};
use esp_hal::spi::{Config, SpiDeviceWithConfig, Rate};
use core::time::Duration;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let spi_bus: Mutex<_, esp_hal::spi::master::SpiDmaBus<'_, esp_hal::Async>> = Mutex::new(spi);

    let spi_device_config = Config::default()
        .with_frequency(Rate::from_khz(1000))
        .with_mode(esp_hal::spi::Mode::_1);

    let spi_device = SpiDeviceWithConfig::new(&spi_bus, ss_pin, spi_device_config);

    if let Ok(mut device) = As8510::new(spi_device, Gain::Gain100, Gain::Gain25).await {
        loop {
            match device.get_current().await {
                Ok(amps) => println!("Current: {}A", amps),
                Err(e) => println!("Error reading current: {:?}", e),
            }
            embassy_time::Timer::after(Duration::from_millis(100)).await;
        }
    }
}
```

### Key Methods

- **`new(peri, current_gain, voltage_gain)`**: Initializes the AS8510 with the specified SPI device and gain settings.
- **`get_current()`**: Reads the current in amperes (A) asynchronously.
- **`get_voltage()`**: Reads the voltage (unverified implementation).

### Gain Configuration

| Gain       | Current Range (A) | Voltage Range |
|------------|-------------------|---------------|
| `Gain1`    | +2076 / -1523     | Not supported |
| `Gain25`   | ±400              | Supported     |
| `Gain40`   | ±235              | Supported     |
| `Gain100`  | ±77               | Not supported |

Note: Voltage measurements are restricted to `Gain25` and `Gain40`.

## Dependencies

- `embedded-hal-async`: For async SPI operations.

## License

This crate is licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please submit issues or pull requests to the [GitHub repository](https://github.com/rand12345/as8510).

## Notes

- The `get_voltage()` method is currently unverified and may require additional calibration or validation.
- Ensure your SPI device is configured correctly for the AS8510's requirements (e.g., 1 MHz frequency, Mode 1).

