pub fn get_date_time_number(date: &[u8; 12]) -> String {
    let year = u16::from_be_bytes(date[0..=1].try_into().unwrap()).to_string();
    let month = u16::from_be_bytes(date[2..=3].try_into().unwrap()).to_string();
    let day = u16::from_be_bytes(date[4..=5].try_into().unwrap()).to_string();
    let hours = u16::from_be_bytes(date[6..=7].try_into().unwrap()).to_string();
    let minutes = u16::from_be_bytes(date[8..=9].try_into().unwrap()).to_string();
    let seconds = u16::from_be_bytes(date[10..=11].try_into().unwrap()).to_string();

    format!(
        "{}-{}-{}, {:0>2}:{:0>2}:{:0>2}",
        year, month, day, hours, minutes, seconds
    )
}

pub fn itu_r_bt1886(input_signal: f32, white_lum: f32, black_lum: f32) -> Result<f32, String> {
    if !(0.0..=1.0).contains(&input_signal) {
        return Err(format!(
            "Input signal not in range 0.0..=1.0 : {}",
            input_signal
        ));
    }

    if white_lum < 0.0 || black_lum < 0.0 {
        return Err(format!(
            "Point luminance must a positive value ! White pt : {}, Blck pt : {}",
            white_lum, black_lum
        ));
    }

    let g: f32 = 2.4;
    let lum_diff: f32 = white_lum.powf(1.0 / g) - black_lum.powf(1.0 / g);
    let a: f32 = lum_diff.powf(g);
    let b: f32 = black_lum.powf(1.0 / g) / lum_diff;

    let max_signal = (input_signal + b).max(0.0);
    let lum = a * max_signal.powf(g);

    Ok(lum)
}
