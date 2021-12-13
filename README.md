[![Build Status](https://github.com/sousandrei/ism330dhcx/workflows/Main/badge.svg)](https://github.com/sousandrei/ism330dhcx/actions)
[![Docs.rs](https://docs.rs/ism330dhcx/badge.svg)](https://docs.rs/ism330dhcx)
[![Crates.io](https://img.shields.io/crates/v/ism330dhcx)](https://crates.io/crates/ism330dhcx)

## Table of Contents

- [About the project](#about)
- [Usage](#usage)
- [Help Wanted](#help-wanted)
- [License](#license)

## <a name="about"></a> About the Project 📃

This is a simple driver for ST's `ism330dhcx` sensor.

Documentation for that sensor can be found at ST's website

- [Sensor page](https://www.st.com/en/mems-and-sensors/ism330dhcx.html)
- [Datasheet](https://www.st.com/resource/en/datasheet/ism330dhcx.pdf)

## <a name="usage"></a> Usage 👀

Check out the `examples` folder for simple implementation

To declare a sensor is pretty simple:

```rust
let sensor = Ism330Dhcx::new(&mut i2c).unwrap()
```

If you want to use another address for the chip, you can do:

```rust
let sensor = Ism330Dhcx::new_with_address(&mut i2c, 0x6au8).unwrap()
```

Or alter it after the fact

```rust
sensor.set_address(0x6au8);
```

All registers have the bits addressed by their function, for example here se set the `BOOT` register in the `CTRL_3C` register to `1`

```rust
sensor.ctrl3c.set_boot(i2c, true).unwrap();
```

For bits that operate together, they have their custom type abstracted. For example, to set the accelerometer data rate you have to operate 4 bits. But here you just have to specify your desired data rate and the driver takes care of it.

```rust
// Sets the following bits
// ODR_XL3 to 0
// ODR_XL2 to 0
// ODR_XL1 to 1
// ODR_XL0 to 1

sensor
    .ctrl1xl
    .set_accelerometer_data_rate(i2c, ctrl1xl::Odr_Xl::Hz52)
    .unwrap();
```

## <a name="help-wanted"></a> Help wanted 🤝

All contributions are welcome!

If you are using or plan to use this create don't hesitate to open an issue or a PR.

Multiple registers are yet to be referenced!

## <a name="license"></a> License

See [LICENSE](https://github.com/sousandrei/firesquid/blob/master/LICENSE) for more details.
