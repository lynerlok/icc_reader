use std::collections::HashMap;

use crate::types::icc::{S15Fixed16Number, XYZNumber};
use crate::utils::bytes::get_bits_of_byte;
use crate::utils::icc::{
    get_date_time_number, ICC_CLASS_PROFILE, ICC_PF_COLOR_SPACE, ICC_PRIM_PLATFORM,
};

pub fn parse_profile_info(icc_raw_data: &[u8]) {
    println!("---- Profile info ----");

    let pf_size: u32 = u32::from_be_bytes(icc_raw_data[0..=3].try_into().unwrap());
    println!("    Profile Size: {:?} bytes", pf_size);

    let pf_cmm_type_sig = String::from_utf8_lossy(&icc_raw_data[4..=7]);
    println!("    Profile CMM Type : {}", pf_cmm_type_sig);

    let pf_major_rev = &icc_raw_data[8];
    let pf_minor_rev = &icc_raw_data[9] >> 4;
    let pf_fix_rev = (&icc_raw_data[9] << 4) >> 4;
    println!(
        "    Profile Revision : {}.{}.{}",
        pf_major_rev, pf_minor_rev, pf_fix_rev
    );

    let pf_cls_sig_hash: HashMap<&str, &str> = HashMap::from(ICC_CLASS_PROFILE);
    let pf_cls_sig =
        match pf_cls_sig_hash.get(String::from_utf8_lossy(&icc_raw_data[12..=15]).as_ref()) {
            Some(val) => val.to_string(),
            None => "Unknown".to_string(),
        };

    println!("    Class : {} profile", pf_cls_sig);

    let pf_data_color_space = String::from_utf8_lossy(&icc_raw_data[16..=19]).to_string();
    let mut pf_data_color_space_trim = pf_data_color_space.split_ascii_whitespace().next().unwrap();

    if !ICC_PF_COLOR_SPACE.contains(&pf_data_color_space_trim) {
        pf_data_color_space_trim = "Unknown";
    }

    println!("    Color Space of data : {}", pf_data_color_space_trim);

    let pf_conn_space = String::from_utf8_lossy(&icc_raw_data[20..=23]).to_string();
    let mut pf_conn_space_trim = pf_conn_space.split_ascii_whitespace().next().unwrap();

    if !ICC_PF_COLOR_SPACE.contains(&pf_conn_space_trim) {
        pf_conn_space_trim = "Unknown";
    }

    println!("    Profile Connection Space : {}", pf_conn_space_trim);

    let pf_date = get_date_time_number(&icc_raw_data[24..=35].try_into().unwrap());
    println!("    Profile creation date : {}", pf_date);

    if u32::from_be_bytes(icc_raw_data[36..=39].try_into().unwrap()) != 0x61637370 {
        println!("    Profile file is not 'acsp'. File may be corrupted exiting…");
        return;
    }

    let pf_primary_sig_hash: HashMap<&str, &str> = HashMap::from(ICC_PRIM_PLATFORM);
    let pf_primary_sig =
        match pf_primary_sig_hash.get(String::from_utf8_lossy(&icc_raw_data[40..=43]).as_ref()) {
            Some(val) => val.to_string(),
            None => "Unknown".to_string(),
        };

    println!("    Primary plateform : {}", pf_primary_sig);
    let pf_flags_msb = match u16::from_be_bytes(icc_raw_data[44..=45].try_into().unwrap()) {
        0 => "Not embedded, can used independently",
        1 => "Embedded, cannot use independently",
        2 => "Not embedded, can used independently",
        3 => "Embedded, can use independently",
        _ => "Unknown",
    };
    println!("    Profile tags : {}", pf_flags_msb);
    let pf_dev_manuf = u32::from_be_bytes(icc_raw_data[48..=51].try_into().unwrap());
    let pf_dev_model = u32::from_be_bytes(icc_raw_data[52..=55].try_into().unwrap());
    println!("    Device manufacturer : {:X}", pf_dev_manuf);
    println!("    Device model : {:X}", pf_dev_model);

    let pf_dev_attrs = &get_bits_of_byte(icc_raw_data[59])[0..=3];
    let refl_attr = match pf_dev_attrs[0] {
        0 => "Reflective",
        1 => "Transparency",
        _ => "Unknown",
    };
    let gloss_attr = match pf_dev_attrs[1] {
        0 => "Glossy",
        1 => "Matte",
        _ => "Unknown",
    };
    let pos_attr = match pf_dev_attrs[2] {
        0 => "Positive",
        1 => "Negative",
        _ => "Unknown",
    };
    let color_attr = match pf_dev_attrs[3] {
        0 => "Color",
        1 => "B&W",
        _ => "Unknown",
    };
    println!(
        "    Attributes : {}, {}, {}, {}",
        refl_attr, gloss_attr, pos_attr, color_attr
    );

    let pf_render_int = match icc_raw_data[67] {
        0 => "Perceptual",
        1 => "Media-Relative Colorimetric",
        2 => "Saturation",
        3 => "ICC-Absolute Colorimetric",
        _ => "Unknown",
    };
    println!("    Rendering intent : {}", pf_render_int);

    let cie_x: S15Fixed16Number =
        S15Fixed16Number::from_be_bytes(icc_raw_data[68..=71].try_into().unwrap());
    let cie_y: S15Fixed16Number =
        S15Fixed16Number::from_be_bytes(icc_raw_data[72..=75].try_into().unwrap());
    let cie_z: S15Fixed16Number =
        S15Fixed16Number::from_be_bytes(icc_raw_data[76..=79].try_into().unwrap());

    let pf_xyz_illum: XYZNumber = [cie_x * 100, cie_y * 100, cie_z * 100];

    println!(
        "    Illuminant : {:.2} {:.2} {:.2}",
        pf_xyz_illum[0], pf_xyz_illum[1], pf_xyz_illum[2]
    );
    let pf_creator_sig = String::from_utf8_lossy(&icc_raw_data[80..=83]);
    let pf_creator_sig_hex = u32::from_be_bytes(icc_raw_data[80..=83].try_into().unwrap());
    println!(
        "    Profile Creator : 0x{:X} ({})",
        pf_creator_sig_hex, pf_creator_sig
    );
    println!("---- End of Profile info ----\n");
}
