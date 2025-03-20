use fixed::types::extra::U16;
use fixed::FixedI32;

pub type S15Fixed16Number = FixedI32<U16>;
pub type XYZNumber = [S15Fixed16Number; 3];
pub type S15Fixed16Array = Vec<S15Fixed16Number>;
pub type Vcgt = (
    u32,      // vcgt_sig
    Vec<u16>, // red ramp
    Vec<u16>, // green ramp
    Vec<u16>, // blue ramp
    Vec<f32>, // max value for each channel
    Vec<f32>, // min value for each channel
    usize,    // # channels
    usize,    // # entries per channel
    usize,    // bitdepth
);
