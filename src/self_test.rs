#![allow(clippy::module_name_repetitions)]

/// Delay and sample timing interface used by the self-test routines.
pub trait SelfTestDelay {
    /// Wait for the specified number of milliseconds.
    fn delay_ms(&mut self, milliseconds: u32);
}

impl<T> SelfTestDelay for T
where
    T: embedded_hal::delay::DelayNs,
{
    fn delay_ms(&mut self, milliseconds: u32) {
        embedded_hal::delay::DelayNs::delay_ms(self, milliseconds);
    }
}

/// Result of a completed accelerometer or gyroscope self-test.
#[derive(Copy, Clone, Debug, PartialEq, defmt::Format)]
pub struct SelfTestResult {
    /// Self-test output change for X, Y, and Z.
    pub delta: [f32; 3],
    /// Whether every axis was within the datasheet acceptance range.
    pub passed: bool,
}

pub(crate) const SELF_TEST_SAMPLES: usize = 5;
pub(crate) const SELF_TEST_SETTLE_MS: u32 = 100;

pub(crate) fn average(samples: &[[f32; 3]; SELF_TEST_SAMPLES]) -> [f32; 3] {
    let mut result = [0.0; 3];
    for sample in samples {
        for (axis, value) in sample.iter().enumerate() {
            result[axis] += value;
        }
    }
    result.map(|value| value / SELF_TEST_SAMPLES as f32)
}

pub(crate) fn evaluate(
    normal: [f32; 3],
    self_test: [f32; 3],
    minimum: f32,
    maximum: f32,
) -> SelfTestResult {
    let delta = core::array::from_fn(|axis| (self_test[axis] - normal[axis]).abs());
    let passed = delta
        .iter()
        .all(|value| *value >= minimum && *value <= maximum);
    SelfTestResult { delta, passed }
}
