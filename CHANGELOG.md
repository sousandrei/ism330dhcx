# Changelog

## [0.5.2](https://github.com/sousandrei/ism330dhcx/compare/v0.5.1...v0.5.2) (2023-12-17)


### Bug Fixes

* gyro and accel chain full scale values match configurable values ([#45](https://github.com/sousandrei/ism330dhcx/issues/45)) ([23dbc52](https://github.com/sousandrei/ism330dhcx/commit/23dbc52c8631a96d4dd7ce4c08c81a1ca48e603a))
* gyro and accel scale wrong: should be specified in milli-unit per bit! ([#49](https://github.com/sousandrei/ism330dhcx/issues/49)) ([2eac16c](https://github.com/sousandrei/ism330dhcx/commit/2eac16c615abcd4d0fb521023ab4cd479c497675))
* making datarates all in Hz ([#47](https://github.com/sousandrei/ism330dhcx/issues/47)) ([c8d2211](https://github.com/sousandrei/ism330dhcx/commit/c8d2211f50f092102874f422de4ec4570c4ad6d1))

## [0.5.1](https://github.com/sousandrei/ism330dhcx/compare/v0.5.0...v0.5.1) (2023-03-28)


### Miscellaneous Chores

* release 0.5.1 ([7b13928](https://github.com/sousandrei/ism330dhcx/commit/7b13928ebeecc9a12a4b2b56ebaf6b42fed8e401))

## [0.5.0](https://github.com/sousandrei/ism330dhcx/compare/v0.4.1...v0.5.0) (2023-01-03)


### Features

* add 52Hz to bdr bdrxl/gy ([#23](https://github.com/sousandrei/ism330dhcx/issues/23)) ([57bdf85](https://github.com/sousandrei/ism330dhcx/commit/57bdf852f9117863ada543be89277a74089de7ca))

## 0.4.1 (2022-09-17)


### Features

* adapt to custom address ([5fcddd2](https://github.com/sousandrei/ism330dhcx/commit/5fcddd228a76ae833d37f66bbe0dafffe8432b22))
* add copy + clone to sensor value ([68de07c](https://github.com/sousandrei/ism330dhcx/commit/68de07cd23b1712df0f17920f13157918020673a))
* add embedded-hal-mock for tests ([869c3df](https://github.com/sousandrei/ism330dhcx/commit/869c3df1755b3cb9db0628ebfe3ee3e56076978a))
* Adding first two registers ([0fc89d8](https://github.com/sousandrei/ism330dhcx/commit/0fc89d828166a76fd5f69e8e9cc9beaf77b32b39))
* Adding getters to all existing registers ([cd10234](https://github.com/sousandrei/ism330dhcx/commit/cd1023422ad1a5b5fed0c769962e83cfcbf01b6d))
* Adding getters to ctrl9xl ([f23a230](https://github.com/sousandrei/ism330dhcx/commit/f23a23072d2eb7da608d0b6e4c5c35466da1ad26))
* Adding new Sensors and stabilize bacis ones ([32ea022](https://github.com/sousandrei/ism330dhcx/commit/32ea022ebc24199b650c6dd858685fea8867bf29))
* All ORD and FS implemented for CTRL2_G ([a2f17c2](https://github.com/sousandrei/ism330dhcx/commit/a2f17c2e0b732397e2b6bd8763d0a4d6b5038de6))
* configure FIFO and read FIFO status. ([93b8243](https://github.com/sousandrei/ism330dhcx/commit/93b8243268529aefd2d3a1c538d9aa16c5cb20b0))
* Deriving whenever possible ([68bd51c](https://github.com/sousandrei/ism330dhcx/commit/68bd51c5d0c55b0a4111157234a93bde0e8c5c36))
* Initial commit ([98f11e6](https://github.com/sousandrei/ism330dhcx/commit/98f11e62ae165a0210a8652f94c59e262aa8509c))
* making address configurable ([fea8fa3](https://github.com/sousandrei/ism330dhcx/commit/fea8fa31427384429f45945cad13cf0623ad112e))
* pop from fifo ([2edf3b0](https://github.com/sousandrei/ism330dhcx/commit/2edf3b05c209001b646761e298aa764424caa572))
* set batch data rate for gyro and accel ([b1b25e7](https://github.com/sousandrei/ism330dhcx/commit/b1b25e7f9e4b1169fd3da06eec037644087fc008))
* swap new / new_with_address (new attempts to write to DEFAULT always) ([c37d95e](https://github.com/sousandrei/ism330dhcx/commit/c37d95ef0606429552834461c2515900f44b341a))


### Bug Fixes

* Changing scales to f32 ([70618b1](https://github.com/sousandrei/ism330dhcx/commit/70618b117c8e6fad94bd7b729b75228a24e895ec))
* Cleaning up bits before setting ([e8b4f45](https://github.com/sousandrei/ism330dhcx/commit/e8b4f4569f9692ce756738b456a3bc70341ee70a))
* don't copy measurements vector ([4269016](https://github.com/sousandrei/ism330dhcx/commit/4269016d1d0404ccee9db8f23908d99866ada49e))
* fifoctrl should go to ..13 ([a118506](https://github.com/sousandrei/ism330dhcx/commit/a11850632d014e26e244ec9bb3f70db9ac7bb4bf))
* Fix a bug on selecting gyro scale ([45966ca](https://github.com/sousandrei/ism330dhcx/commit/45966caf816dab44e69b0b295db69a098aabe9fe))
* Fix embedded-hal-mock dependency no-std ([ff4050f](https://github.com/sousandrei/ism330dhcx/commit/ff4050f295ad57dd14ab9f267c77d5ddb12ddc6a))
* Fix too low precision on dps to rag constant ([be82a6b](https://github.com/sousandrei/ism330dhcx/commit/be82a6baf2b4d3b6089f385e5ccc9640f1b2182c))
* fixing hz to khz on some instances ([bbae1af](https://github.com/sousandrei/ism330dhcx/commit/bbae1afa7a913a404aa7e79f5c05ad7e80da436b))
* handle empty queue and other values + defmt (should be made optional) ([c0fdccb](https://github.com/sousandrei/ism330dhcx/commit/c0fdccb31842cef700988b1e5316a97b5df5343a))
* Limiting the parameters when setting bits ([82dc06a](https://github.com/sousandrei/ism330dhcx/commit/82dc06ad5a7ed99343a024a8e5499626dfbafbd6))


### Miscellaneous Chores

* release 0.4.1 ([a4405ca](https://github.com/sousandrei/ism330dhcx/commit/a4405ca7bde3c3895e533aa162ebb35aac0dc941))
