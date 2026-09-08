[![Build Status](https://github.com/sousandrei/ism330dhcx/workflows/Main/badge.svg)](https://github.com/sousandrei/ism330dhcx/actions)
[![Docs.rs](https://docs.rs/ism330dhcx/badge.svg)](https://docs.rs/ism330dhcx)
[![Crates.io](https://img.shields.io/crates/v/ism330dhcx)](https://crates.io/crates/ism330dhcx)

## Table of Contents

- [About the project](#about)
- [Usage](#usage)
- [Help Wanted](#help-wanted)
- [License](#license)

## <a name="about"></a> About the Project

This is a simple driver for ST's `ism330dhcx` sensor.

Documentation for that sensor can be found at ST's website

- [Sensor page](https://www.st.com/en/mems-and-sensors/ism330dhcx.html)
- [Datasheet](https://www.st.com/resource/en/datasheet/ism330dhcx.pdf)

## <a name="usage"></a> Usage

Check out the `examples` folder for simple implementation

To declare a sensor is pretty simple:

```rust
let sensor = Ism330Dhcx::new(&mut i2c).unwrap()
```

For four-wire SPI, configure the SPI device and chip select with your HAL, then
wrap it in `SpiDeviceBus`:

```rust
let mut spi = SpiDeviceBus::new(spi_device);
let sensor = Ism330Dhcx::new_spi(&mut spi).unwrap();
```

The caller selects SPI mode, clock frequency, electrical setup, and chip-select
handling according to the datasheet and HAL. Three-wire SPI is not supported.

If you want to use another address for the chip, you can do:

```rust
let sensor = Ism330Dhcx::new_with_address(&mut i2c, 0x6au8).unwrap()
```

Or alter it after the fact

```rust
sensor.set_address(0x6au8);
```

To configure the sensor, use the high-level methods:

```rust
sensor.set_accel_odr(&mut i2c, OdrXl::Hz52).unwrap();
sensor.set_boot(&mut i2c, true).unwrap();
```

The driver borrows the register transport for each operation, so the same bus
can be shared with other devices. Supported feature groups include accelerometer and
gyroscope configuration, user offsets, FIFO, motion events, interrupt/status
reads, embedded functions, sensor-hub access, timestamp reads, and OIS
configuration.

Motion events can be configured through the `Motion` trait and routed to either
interrupt pin through the event-routing methods:

```rust
sensor.set_tap_x(&mut i2c, true).unwrap();
sensor.set_tap_threshold_x(&mut i2c, 8).unwrap();
sensor.set_4d(&mut i2c, true).unwrap();
sensor.set_int1_double_tap(&mut i2c, true).unwrap();
```

Sensor-hub access configures an external sensor read and retrieves its latest
sample data without transferring ownership of the bus:

```rust
sensor
    .configure_sensor_hub_read(&mut i2c, 0x68, 0x20, 4, 0, false)
    .unwrap();
let mut external_data = [0; 4];
sensor.read_sensor_hub(&mut i2c, &mut external_data).unwrap();
```

Additional external sensors can be configured independently through
`SensorHubSlave`:

```rust
sensor
    .configure_sensor_hub_slave(
        &mut i2c,
        SensorHubReadConfig {
            slave: SensorHubSlave::Slave1,
            address: 0x68,
            register: 0x20,
            length: 6,
            odr: 0,
            batch: true,
        },
    )
    .unwrap();
```

Sensor-hub pull-ups, pass-through, auxiliary-sensor mode, reset, and operation
status are exposed through the `SensorHub` trait.

The embedded-function block includes two programmable processing engines:

- **FSM (Finite State Machine):** up to 16 programmable event-detection
  programs that can evaluate sensor data and generate individual interrupt
  outputs.
- **MLC (Machine Learning Core):** a configurable classifier with up to 8
  output signals for recognizing patterns in sensor data.

The driver provides enable and initialization controls, typed interrupt routing,
status accessors, FSM output and MLC source reads, long-counter operations, and
bounded access to the advanced configuration pages. The actual FSM and MLC
programs or models must be supplied by the application according to ST's
configuration tools and device documentation.

FSM and MLC routing and status are available through typed selectors:

```rust
sensor
    .set_fsm_int1(&mut i2c, FsmProgram::Program1, true)
    .unwrap();
sensor
    .set_mlc_int2(&mut i2c, MlcOutput::Output1, true)
    .unwrap();
```

Advanced FSM/MLC page data can be accessed with bounded
`read_embedded_page` and `write_embedded_page` operations.

Accelerometer and gyroscope self-tests are available through
`run_accel_self_test` and `run_gyro_self_test`. These methods configure the
recommended self-test data rate and range, wait for the sensor to settle,
acquire five samples with the supplied `SelfTestDelay`, evaluate the measured
change against the datasheet limits, and restore the previous configuration.
The returned `SelfTestResult` contains the per-axis output change and the
overall pass/fail result.

FIFO words are seven bytes: one tag byte followed by six data bytes. Use
`fifo_pop_entry` when tag counters and parity are needed; it decodes physical
sensor, temperature, timestamp, step-counter, sensor-hub, configuration-change,
and compressed-data entries while preserving reserved payloads.

For bits that operate together, the driver provides typed register fields. For
example, to set the accelerometer data rate you specify the desired data rate
and the driver updates the corresponding four bits.

```rust
sensor.set_accel_odr(&mut i2c, OdrXl::Hz52).unwrap();
```

## <a name="help-wanted"></a> Help wanted

All contributions are welcome!

If you are using or plan to use this crate, do not hesitate to open an issue or a PR.

The complete datasheet register map is represented under `src/registers/`.
Feature modules provide the public driver API and use the register definitions
for I2C and four-wire SPI access.

## <a name="license"></a> License

See [LICENSE](LICENSE) for more details.
