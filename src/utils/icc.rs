pub const ICC_PF_COLOR_SPACE: [&str; 25] = [
    "XYZ", "Lab", "Luv", "YCbr", "Yxy", "RGB", "GRAY", "HSV", "HLS", "CMYK", "CMY", "2CLR", "3CLR",
    "4CLR", "5CLR", "6CLR", "7CLR", "8CLR", "9CLR", "ACLR", "BCLR", "CCLR", "DCLR", "ECLR", "FCLR",
];

pub const ICC_CLASS_PROFILE: [(&str, &str); 7] = [
    ("scnr", "Input Device"),
    ("mntr", "Display Device"),
    ("prtr", "Output Device"),
    ("link", "DeviceLink"),
    ("spac", "ColorSpace Conversion"),
    ("abst", "Abstract"),
    ("nmcl", "Named Color"),
];

pub const ICC_PRIM_PLATFORM: [(&str, &str); 6] = [
    ("APPL", "Apple Computer, Inc."),
    ("MSFT", "Microsoft Corporation"),
    ("SGI", "Silicon Graphics, Inc."),
    ("SUNW", "Sun Microsystems, Inc."),
    ("TGNT", "Taligent, Inc."),
    ("*nix", "UNIX and Derivatives - *nix"),
];

pub const ICC_CHRM_TYPE: [(&str, (f64, f64), (f64, f64), (f64, f64)); 6] = [
    (
        "ITU-R BT.709-2",
        (0.640, 0.330),
        (0.300, 0.600),
        (0.150, 0.060),
    ),
    (
        "SMPTE RP145",
        (0.630, 0.340),
        (0.310, 0.595),
        (0.155, 0.070),
    ),
    (
        "EBU Tech. 3213-E",
        (0.640, 0.330),
        (0.290, 0.600),
        (0.150, 0.060),
    ),
    ("P22", (0.625, 0.340), (0.280, 0.605), (0.155, 0.070)),
    ("P3", (0.680, 0.320), (0.265, 0.690), (0.150, 0.060)),
    (
        "ITU-R BT.2020",
        (0.780, 0.292),
        (0.170, 0.797),
        (0.131, 0.046),
    ),
];

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
