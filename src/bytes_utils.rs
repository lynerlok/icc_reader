use crate::rs_types::S15Fixed16Number;

pub fn get_bits_of_byte(byte: u8) -> [u8; 8] {
    let mut bits = [0u8; 8];
    for idx in 0..=7 {
        let shifted_byte = byte >> idx;
        let cur_bit = shifted_byte & 1;
        bits[7 - idx] = cur_bit;
    }
    bits
}

pub fn bytes_to_u32(bytes: &[u8]) -> Result<u32, String> {
    let bytes_arr: &[u8; 4] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };
    Ok(u32::from_be_bytes(*bytes_arr))
}

pub fn bytes_to_u16(bytes: &[u8]) -> Result<u16, String> {
    let bytes_arr: &[u8; 2] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };
    Ok(u16::from_be_bytes(*bytes_arr))
}

pub fn bytes_u32_usize(bytes: &[u8]) -> Result<usize, String> {
    let bytes_arr: &[u8; 4] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };
    let u32 = u32::from_be_bytes(*bytes_arr);
    let usize = match usize::try_from(u32) {
        Ok(usize) => usize,
        Err(err) => return Err(err.to_string()),
    };
    Ok(usize)
}

pub fn bytes_u16_usize(bytes: &[u8]) -> Result<usize, String> {
    let bytes_arr: &[u8; 2] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };
    let u16 = u16::from_be_bytes(*bytes_arr);
    let usize = match usize::try_from(u16) {
        Ok(usize) => usize,
        Err(err) => return Err(err.to_string()),
    };
    Ok(usize)
}

pub fn bytes_to_sf32(bytes: &[u8]) -> Result<S15Fixed16Number, String> {
    let bytes_arr: &[u8; 4] = match bytes.try_into() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };
    let sf32 = S15Fixed16Number::from_be_bytes(*bytes_arr);
    Ok(sf32)
}

pub fn read_utf16(slice: &[u8]) -> Option<String> {
    let iter =
        (0..(slice.len() / 2 - 1)).map(|i| u16::from_be_bytes([slice[2 * i], slice[2 * i + 1]]));

    std::char::decode_utf16(iter)
        .collect::<Result<String, _>>()
        .ok()
}
