# Changelog

## 0.1.0 (2022-09-17)


### Features

* adapt to custom address ([c4e4ee9](https://github.com/sousandrei/ism330dhcx/commit/c4e4ee9431ae5b5e92204438437f527fbe76b1c3))
* add copy + clone to sensor value ([54d5843](https://github.com/sousandrei/ism330dhcx/commit/54d5843e35c74baf277033ffd015003893eff02f))
* add embedded-hal-mock for tests ([8bad860](https://github.com/sousandrei/ism330dhcx/commit/8bad86083c67b1811fced2b9d258204844446816))
* Adding first two registers ([56ddfc9](https://github.com/sousandrei/ism330dhcx/commit/56ddfc972e9c188fd8d64e46385731a368e65c5c))
* Adding getters to all existing registers ([dfd8589](https://github.com/sousandrei/ism330dhcx/commit/dfd8589c9a1bd639b95de647438fb4ec14705147))
* Adding getters to ctrl9xl ([e7ea31e](https://github.com/sousandrei/ism330dhcx/commit/e7ea31e832149585f4ea7a015bd6c47ea23b6b0e))
* Adding new Sensors and stabilize bacis ones ([3440f63](https://github.com/sousandrei/ism330dhcx/commit/3440f63461c9ac1e883916e650c8f14a0f7e88c5))
* All ORD and FS implemented for CTRL2_G ([dd13e0e](https://github.com/sousandrei/ism330dhcx/commit/dd13e0e30f91ef48973b7870b2c0cca653de4c53))
* configure FIFO and read FIFO status. ([4cf67d6](https://github.com/sousandrei/ism330dhcx/commit/4cf67d6ce751c26847c62fbb14a5dc52cf7e542e))
* Deriving whenever possible ([0e897b2](https://github.com/sousandrei/ism330dhcx/commit/0e897b21f9a58bc06148d4052d6376bdcef6f5d5))
* making address configurable ([0a2d9b0](https://github.com/sousandrei/ism330dhcx/commit/0a2d9b01edea7d3958c14b2ec707159c57acb96b))
* pop from fifo ([5a96aa4](https://github.com/sousandrei/ism330dhcx/commit/5a96aa474f2c6a0ba132bdd06197efd9ea74847d))
* set batch data rate for gyro and accel ([de06967](https://github.com/sousandrei/ism330dhcx/commit/de06967541884209a7e7dac21d54b14cf0a5cbdf))
* swap new / new_with_address (new attempts to write to DEFAULT always) ([fdf6926](https://github.com/sousandrei/ism330dhcx/commit/fdf692644ebeee95a25b956ce0a79010bc5b5bbe))


### Bug Fixes

* Changing scales to f32 ([beb94b5](https://github.com/sousandrei/ism330dhcx/commit/beb94b5e07c83f8b071813ddd56b7f311478e90b))
* Cleaning up bits before setting ([7d0ccf2](https://github.com/sousandrei/ism330dhcx/commit/7d0ccf280a5058312f43190a1486cd784c56cc9e))
* don't copy measurements vector ([ca83ad6](https://github.com/sousandrei/ism330dhcx/commit/ca83ad688da6b704db7bcf16b947dd2079992f27))
* fifoctrl should go to ..13 ([8c31a64](https://github.com/sousandrei/ism330dhcx/commit/8c31a6491d30f0c8dfb812bafb33cb8d9757c4cc))
* Fix a bug on selecting gyro scale ([8ed9bc7](https://github.com/sousandrei/ism330dhcx/commit/8ed9bc7d16bf1a9b10dd2398c63ac12bf7b8cb9e))
* Fix embedded-hal-mock dependency no-std ([49a8584](https://github.com/sousandrei/ism330dhcx/commit/49a858419c556619ff6eb04fddeb3174d7adaea2))
* Fix too low precision on dps to rag constant ([d915d07](https://github.com/sousandrei/ism330dhcx/commit/d915d07c511a291206ace5d3c4f43e0bf6ff4cf8))
* fixing hz to khz on some instances ([2a8daf9](https://github.com/sousandrei/ism330dhcx/commit/2a8daf91f35ac31da5ebbdd32abde47f05951339))
* handle empty queue and other values + defmt (should be made optional) ([5c78eb2](https://github.com/sousandrei/ism330dhcx/commit/5c78eb2b00449f80cf9359630c8b91045f8d3af2))
* Limiting the parameters when setting bits ([16e2c81](https://github.com/sousandrei/ism330dhcx/commit/16e2c81fe16c2bdd2800bc7e2199fb1a129cc2f9))
