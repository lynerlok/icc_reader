use crate::bytes_utils::*;
use crate::rs_types::*;

pub fn read_text_desc_type(icc_raw_data: &[u8], idx: usize) -> Result<(String, String), String> {
    let ascii_desc_size_range: &[u8; 4] = match icc_raw_data[(idx + 8)..=(idx + 11)].try_into() {
        Ok(range) => range,
        Err(err) => return Err(err.to_string()),
    };
    let ascii_desc_size = match usize::try_from(u32::from_be_bytes(*ascii_desc_size_range)) {
        Ok(usize) => usize,
        Err(err) => return Err(err.to_string()),
    };

    let ascii_desc_str =
        String::from_utf8_lossy(&icc_raw_data[(idx + 12)..(idx + 12 + ascii_desc_size)]);

    let utf_lang_code_range: [u8; 4] = match icc_raw_data
        [(idx + 12 + ascii_desc_size)..=(idx + 12 + ascii_desc_size + 3)]
        .try_into()
    {
        Ok(range) => range,
        Err(err) => return Err(err.to_string()),
    };
    let _utf_lang_code = u32::from_be_bytes(utf_lang_code_range);

    let utf_desc_size_range: &[u8; 4] = match icc_raw_data
        [(idx + 12 + ascii_desc_size + 4)..=(idx + 12 + ascii_desc_size + 7)]
        .try_into()
    {
        Ok(range) => range,
        Err(err) => return Err(err.to_string()),
    };

    let utf_desc_size = match usize::try_from(u32::from_be_bytes(*utf_desc_size_range)) {
        Ok(usize) => usize,
        Err(err) => return Err(err.to_string()),
    };

    let utf_desc_str = match read_utf16(
        &icc_raw_data[(idx + 12 + ascii_desc_size + 10)
            ..(idx + 12 + ascii_desc_size + 8 + (utf_desc_size * 2))],
    ) {
        Some(str) => str,
        None => return Err("Unable to read UTF16 string".to_string()),
    };

    Ok((ascii_desc_str.to_string(), utf_desc_str))
}

pub fn read_text_type(
    icc_raw_data: &[u8],
    idx: usize,
    end: usize,
) -> Result<(u32, String), String> {
    let text_sig_range: &[u8; 4] = match icc_raw_data[idx..=(idx + 3)].try_into() {
        Ok(range) => range,
        Err(err) => return Err(err.to_string()),
    };
    let text_sig = u32::from_be_bytes(*text_sig_range);
    let text_str = String::from_utf8_lossy(&icc_raw_data[(idx + 8)..(idx + end)]);

    Ok((text_sig, text_str.to_string()))
}

pub fn new_xyz(x: &[u8], y: &[u8], z: &[u8]) -> Result<XYZNumber, String> {
    let mut xyz: XYZNumber = [S15Fixed16Number::from_num(0); 3];
    let data_in = [x, y, z];
    for (idx, coor) in data_in.iter().enumerate() {
        let x_range: &[u8; 4] = match (*coor).try_into() {
            Ok(x_range) => x_range,
            Err(err) => return Err(err.to_string()),
        };
        let cie_coor: S15Fixed16Number = S15Fixed16Number::from_be_bytes(*x_range);
        xyz[idx] = cie_coor * 100;
    }

    Ok(xyz)
}

pub fn read_xyz_type(
    icc_raw_data: &[u8],
    idx: usize,
    end: usize,
) -> Result<(u32, XYZNumber), String> {
    if (idx + end) - (idx + 8) < 12 {
        return Err("Tag is not long enough to contain XYZ values…".to_string());
    }

    let xyz_sig_range: &[u8; 4] = match icc_raw_data[idx..=(idx + 3)].try_into() {
        Ok(range) => range,
        Err(err) => return Err(err.to_string()),
    };
    let xyz_sig = u32::from_be_bytes(*xyz_sig_range);
    let base_num_idx = idx + 8;

    let cie_x = &icc_raw_data[base_num_idx..=(base_num_idx + 3)];
    let cie_y = &icc_raw_data[(base_num_idx + 4)..=(base_num_idx + 7)];
    let cie_z = &icc_raw_data[(base_num_idx + 8)..=(base_num_idx + 11)];

    let xyz_number = match new_xyz(cie_x, cie_y, cie_z) {
        Ok(xyz_number) => xyz_number,
        Err(err) => return Err(err.to_string()),
    };

    Ok((xyz_sig, xyz_number))
}

fn extract_key_value(tag_dict: &[u8], elem_size: usize) -> Result<Vec<usize>, String> {
    let vec_capacity = match elem_size {
        16 => 4,
        24 => 6,
        _ => 8,
    };

    let mut key_value_desc: Vec<usize> = Vec::with_capacity(vec_capacity);

    let name_str_offset = bytes_u32_usize(&tag_dict[0..=3])?;
    let name_str_size = bytes_u32_usize(&tag_dict[4..=7])?;
    let val_str_offset = bytes_u32_usize(&tag_dict[8..=11])?;
    let val_str_size = bytes_u32_usize(&tag_dict[12..=15])?;

    key_value_desc.push(name_str_offset);
    key_value_desc.push(name_str_size);
    key_value_desc.push(val_str_offset);
    key_value_desc.push(val_str_size);

    if elem_size > 16 {
        let disp_name_elem_offset = bytes_u32_usize(&tag_dict[16..=19])?;
        let disp_name_elem_size = bytes_u32_usize(&tag_dict[20..=23])?;

        key_value_desc.push(disp_name_elem_offset);
        key_value_desc.push(disp_name_elem_size);

        if elem_size > 24 {
            let disp_value_elem_offset = bytes_u32_usize(&tag_dict[24..=27])?;
            let disp_value_elem_size = bytes_u32_usize(&tag_dict[28..=31])?;

            key_value_desc.push(disp_value_elem_offset);
            key_value_desc.push(disp_value_elem_size);
        }
    }

    Ok(key_value_desc)
}

pub fn read_dict_type(
    icc_raw_data: &[u8],
    idx: usize,
    size: usize,
) -> Result<(u32, Vec<(String, String)>), String> {
    let dict_tag = &icc_raw_data[idx..(idx + size)];
    let mut dict_str: Vec<(String, String)> = vec![];
    let dict_sig = bytes_to_u32(&dict_tag[0..=3])?;
    let dict_size = bytes_u32_usize(&dict_tag[8..=11])?;
    let element_size = bytes_u32_usize(&dict_tag[12..=15])?;

    let element_last_idx = element_size / 4;

    for entry_num in 0..dict_size {
        let entry_idx = 16 + (16 * entry_num);
        let key_value_desc = extract_key_value(
            &dict_tag[entry_idx..(entry_idx + element_size)],
            element_size,
        )?;

        for idx in (0..element_last_idx).step_by(4) {
            let name_idx = key_value_desc[idx];
            let name_last_idx = name_idx + key_value_desc[idx + 1];
            let value_idx = key_value_desc[idx + 2];
            let value_last_idx = value_idx + key_value_desc[idx + 3];
            let name_range = &dict_tag[name_idx..name_last_idx];
            let value_range = &dict_tag[value_idx..value_last_idx];
            let name: String = String::from_utf8_lossy(name_range).to_string();
            let value: String = String::from_utf8_lossy(value_range).to_string();
            dict_str.push((name, value));
        }
    }
    Ok((dict_sig, dict_str))
}

pub fn read_sf32_type(
    icc_raw_data: &[u8],
    idx: usize,
    size: usize,
) -> Result<(u32, S15Fixed16Array), String> {
    let sf32_tag = &icc_raw_data[idx..(idx + size)];
    let sf32_sig = bytes_to_u32(&sf32_tag[0..=3])?;
    let mut sf32: S15Fixed16Array = Vec::with_capacity(size);

    for idx in (8..sf32_tag.len()).step_by(4) {
        let fixed: S15Fixed16Number = bytes_to_sf32(&sf32_tag[idx..idx + 4])?;
        sf32.push(fixed);
    }

    Ok((sf32_sig, sf32))
}

pub fn read_vcgt_type(
    icc_raw_data: &[u8],
    idx: usize,
    size: usize,
) -> Result<Option<Vcgt>, String> {
    let vcgt_tag = &icc_raw_data[idx..(idx + size)];
    let vcgt_sig = bytes_to_u32(&vcgt_tag[0..=3])?;

    if vcgt_sig != 0x76636774 {
        return Err(format!("Bad vcgt sig found : {}", vcgt_sig));
    }

    let gamma_type = bytes_to_u32(&vcgt_tag[8..=11])?;

    println!("Gamma type : {}", gamma_type);

    let vcgt: Option<Vcgt> = match gamma_type {
        0 => {
            println!("Gamma table found");
            let channels_num = bytes_u16_usize(&vcgt_tag[12..=13])?;
            let entries_num = bytes_u16_usize(&vcgt_tag[14..=15])?;
            let entry_size = bytes_u16_usize(&vcgt_tag[16..=17])?;
            let bitdepth = entry_size * 8;

            println!("Channels # : {}", channels_num);
            println!("Entries # : {}", entries_num);
            println!("Bitdepth : {}", bitdepth);

            if channels_num != 3 {
                return Err("channel number must be 3 (RGB)".to_string());
            }

            let mut r_ramp: Vec<u16> = Vec::with_capacity(entries_num + 1);
            let mut g_ramp: Vec<u16> = Vec::with_capacity(entries_num + 1);
            let mut b_ramp: Vec<u16> = Vec::with_capacity(entries_num + 1);
            let mut r_max = 0;
            let mut g_max = 0;
            let mut b_max = 0;
            let mut r_min = u16::MAX;
            let mut g_min = u16::MAX;
            let mut b_min = u16::MAX;

            for entry in (0..(entries_num * 2)).step_by(2) {
                let entry_idx: usize = entry + 18;
                match entry_size {
                    2 => {
                        let r_value = bytes_to_u16(&vcgt_tag[entry_idx..=(entry_idx + 1)])?;
                        let g_value = bytes_to_u16(
                            &vcgt_tag
                                [(entries_num * 2 + entry_idx)..=(entries_num * 2 + entry_idx + 1)],
                        )?;
                        let b_value = bytes_to_u16(
                            &vcgt_tag
                                [(entries_num * 4 + entry_idx)..=(entries_num * 4 + entry_idx + 1)],
                        )?;

                        r_ramp.push(r_value);
                        g_ramp.push(g_value);
                        b_ramp.push(b_value);

                        if r_max < r_value {
                            r_max = r_value;
                        }
                        if r_min > r_value {
                            r_min = r_value;
                        }
                        if g_max < g_value {
                            g_max = g_value;
                        }
                        if g_min > g_value {
                            g_min = g_value;
                        }
                        if b_max < b_value {
                            b_max = b_value;
                        }
                        if b_min > b_value {
                            b_min = b_value;
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Bad bitdepth, should be 8 or 16 bits find, {}",
                            bitdepth
                        ))
                    }
                }
            }
            if r_max.abs_diff(r_min) < u16::MAX / 20
                && g_max.abs_diff(g_min) < u16::MAX / 20
                && b_max.abs_diff(b_min) < u16::MAX / 20
            {
                println!("Warning ! Contrast velow 5%% (vcgt)");
                println!(
                    "min/max for red: {}/{} green: {}/{} blue: {}/{}",
                    r_min, r_max, g_min, g_max, b_min, b_max
                );
            }

            let vcgt: Vcgt = (
                vcgt_sig,
                r_ramp,
                g_ramp,
                b_ramp,
                vec![
                    r_max as f32 / u16::MAX as f32 * 100.0,
                    g_max as f32 / u16::MAX as f32 * 100.0,
                    b_max as f32 / u16::MAX as f32 * 100.0,
                ],
                vec![
                    r_min as f32 / u16::MAX as f32 * 100.0,
                    g_min as f32 / u16::MAX as f32 * 100.0,
                    b_min as f32 / u16::MAX as f32 * 100.0,
                ],
                channels_num,
                entries_num,
                bitdepth,
            );

            Some(vcgt)
        }
        1 => {
            println!("Simple gamma value found");
            let r_gamma = bytes_to_sf32(&vcgt_tag[12..=15])?;
            let r_min = bytes_to_sf32(&vcgt_tag[16..=19])?;
            let r_max = bytes_to_sf32(&vcgt_tag[20..=23])?;

            println!("Red channel : G{} Min{} Max{}", r_gamma, r_min, r_max);

            None
        }
        _ => return Err("Gamma type unknown (must be 0 or 1)".to_string()),
    };

    Ok(vcgt)
}
