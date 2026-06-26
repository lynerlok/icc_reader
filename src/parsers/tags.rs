use crate::types::icc::{
    DescTypePrettyPrint, MmodTypePrettyPrint, TxtTypePrettyPrinter, TxtTypeTi3PrettyPrinter,
    XYZNumberPrettyPrint,
};

use crate::parsers::icc_types::*;

pub fn parse_icc_tags(icc_raw_data: &[u8], pt_num: &usize, corr_scale: &usize) {
    println!("---- TAG Table ----");
    let pf_tag_table_size: usize = u32::from_be_bytes(icc_raw_data[128..=131].try_into().unwrap())
        .try_into()
        .unwrap();

    println!("    Number of tags in the file : {}", pf_tag_table_size);
    let mut tags: Vec<(u32, String, u32, u32)> = Vec::with_capacity(pf_tag_table_size);
    for tag_num in 0..(pf_tag_table_size) {
        let tag_idx: usize = 132 + (tag_num * 12);
        let tag_sig = u32::from_be_bytes(icc_raw_data[tag_idx..=(tag_idx + 3)].try_into().unwrap());
        let tag_str = String::from_utf8_lossy(&icc_raw_data[tag_idx..=(tag_idx + 3)]);
        let tag_offset = u32::from_be_bytes(
            icc_raw_data[(tag_idx + 4)..=(tag_idx + 7)]
                .try_into()
                .unwrap(),
        );
        let tag_size = u32::from_be_bytes(
            icc_raw_data[(tag_idx + 8)..=(tag_idx + 11)]
                .try_into()
                .unwrap(),
        );
        tags.push((tag_sig, tag_str.to_string(), tag_offset, tag_size));
    }

    println!("---- TAG INFO ----");

    for (tag_idx, tag) in tags.iter().enumerate() {
        let idx: usize = usize::try_from(tag.2).unwrap();
        let size: usize = usize::try_from(tag.3).unwrap();

        let tag_name_arr = tag.0.to_be_bytes();
        let tag_str = String::from_utf8_lossy(&tag_name_arr).to_string();

        match tag_str.as_str() {
            "desc" => {
                // 0x64657363
                let desc_str = match read_text_desc_type(icc_raw_data, idx) {
                    Ok(desc) => DescTypePrettyPrint(desc),
                    Err(err) => {
                        println!("Error while parsing ICC desc tag : {}", err);
                        return;
                    }
                };

                println!("    Description :");
                println!("      {}", desc_str);
            }
            "cprt" => {
                //0x63707274
                let cprt_str = match read_text_type(icc_raw_data, idx, size) {
                    Ok(cprt) => TxtTypePrettyPrinter(cprt),
                    Err(err) => {
                        println!("Error while parsing copyright tag : {}", err);
                        return;
                    }
                };

                println!("    Copyright :");
                println!("      {}", cprt_str);
            }
            "dmnd" => {
                //0x646D6E64
                let dmnd_str = match read_text_desc_type(icc_raw_data, idx) {
                    Ok(dmnd) => DescTypePrettyPrint(dmnd),
                    Err(err) => {
                        println!("Error while parsing device manufacturer tag : {}", err);
                        return;
                    }
                };
                println!("    Device Manufacturer name :");
                println!("      {}", dmnd_str);
            }
            "wtpt" => {
                //0x77747074
                let xyz_disp = match read_xyz_type(icc_raw_data, idx, size) {
                    Ok(xyz) => XYZNumberPrettyPrint(xyz),
                    Err(err) => {
                        println!("Unable to read Media white point illuminant : {}", err);
                        return;
                    }
                };
                println!("    Media white point illuminant : ");
                println!("      {}", xyz_disp);
            }
            "bkpt" => {
                // 0x626B7074
                let xyz_disp = match read_xyz_type(icc_raw_data, idx, size) {
                    Ok(xyz) => XYZNumberPrettyPrint(xyz),
                    Err(err) => {
                        println!("Unable to read Media black point illuminant : {}", err);
                        return;
                    }
                };
                println!("    Media black point illuminant : ");
                println!("      {}", xyz_disp);
            }
            "DevD" => {
                //0x44657644
                let ti3_disp = match read_text_type(icc_raw_data, idx, size) {
                    Ok(ti3) => TxtTypeTi3PrettyPrinter(ti3),
                    Err(err) => {
                        println!("Unable to read characterization device values : {}", err);
                        return;
                    }
                };
                println!("    Characterizaton device values (ti3 file): ");
                println!("      {}", ti3_disp);
            }
            "CIED" => {
                //0x43494544
                let ti3_disp = match read_text_type(icc_raw_data, idx, size) {
                    Ok(txt) => TxtTypeTi3PrettyPrinter(txt),
                    Err(err) => {
                        println!(
                            "Unable to read characterization measurement values : {}",
                            err
                        );
                        return;
                    }
                };
                println!("    Characterizaton measurement values (ti3 file): ");
                println!("      {}", ti3_disp);
            }
            "targ" => {
                //0x74617267
                let ti3_disp = match read_text_type(icc_raw_data, idx, size) {
                    Ok(txt) => TxtTypeTi3PrettyPrinter(txt),
                    Err(err) => {
                        println!("Unable to read characterization target : {}", err);
                        return;
                    }
                };
                println!("    Characterizaton target (ti3 file): ");
                println!("      {}", ti3_disp);
            }
            "lumi" => {
                //0x6C756D69
                let (xyz_sig, xyz_number) = match read_xyz_type(icc_raw_data, idx, size) {
                    Ok(xyz) => xyz,
                    Err(err) => {
                        println!("Unable to read luminance : {}", err);
                        return;
                    }
                };
                println!("    Luminance : ");
                println!("      ({:X}) {:.2} cd/m²", xyz_sig, xyz_number[1] / 100);
            }
            "meta" => {
                //0x6D657461
                let (dict_sig, dict) = match read_dict_type(icc_raw_data, idx, size) {
                    Ok(dict) => dict,
                    Err(err) => {
                        println!("Unable to read dist : {}", err);
                        return;
                    }
                };
                println!("    Metadatas : ({:X})", dict_sig);

                let mut max_str_size = 0;

                for (key, _val) in dict.iter() {
                    let key_len = key.len();
                    if key_len > max_str_size {
                        max_str_size = key_len;
                    }
                }

                dict.iter().for_each(|o| {
                    print!("      {}", o.0);
                    let str_size = (max_str_size / 2) - (o.0.len() / 2);
                    let mut str = String::with_capacity(str_size);
                    for _c in 0..str_size {
                        str.push(' ');
                    }
                    println!("{} | {}", str, o.1);
                });
            }
            "arts" => {
                //0x61727473
                let (sf32_sig, sf32_vec) = match read_sf32_type(icc_raw_data, idx, size) {
                    Ok(sf32) => sf32,
                    Err(err) => {
                        println!("Unable to read arts tag : {}", err);
                        return;
                    }
                };
                println!(
                    "    Absolute to media relative transform : ({:X}) ",
                    sf32_sig
                );
                println!("      Matrix : Bardford (ICC Recommendation)");
                for (idx, val) in sf32_vec.iter().enumerate() {
                    if idx % 3 == 0 {
                        print!("       ");
                    }
                    print!("{}", val);
                    if idx % 3 == 2 {
                        println!();
                    } else {
                        print!(" ");
                    }
                }
            }
            "vcgt" => {
                //0x76636774
                match read_vcgt_type(icc_raw_data, idx, size) {
                    Ok(vcgt) => match vcgt {
                        Some(vcgt) => {
                            println!("    Video Card Gamma Table : ({:X})", vcgt.0);
                            println!("      Max Values (RGB) : {:.2?}", vcgt.4);
                            println!("      Min Values (RGB) : {:.2?}", vcgt.5);
                            println!("      Channels : {}", vcgt.6);
                            println!("      Entries per channel : {}", vcgt.7);
                            println!("      Bitdepth : {} bits", vcgt.8);
                            println!(
                                "      Number of value to display : {} (step : {})",
                                pt_num,
                                (100.0 / *pt_num as f32)
                            );
                            println!("      Correction scale +/- {}", corr_scale);
                            println!("      In (RGB) -> Out (R,G,B)");
                            let chan_len = vcgt.1.len();
                            let step: usize = ((1.0 / *pt_num as f32) * 256.0).ceil() as usize;
                            for idx in (0..chan_len).step_by(step) {
                                let lin_val: f32 = idx as f32 / 256.0;
                                println!(
                                    "      {:.2}  {:.0}, {:.0}, {:.0}",
                                    idx as f32 / 256.0,
                                    (((vcgt.1[idx] as f32 / 256.0) / 256.0) - lin_val)
                                        * *corr_scale as f32,
                                    (((vcgt.1[idx] as f32 / 256.0) / 256.0) - lin_val)
                                        * *corr_scale as f32,
                                    (((vcgt.3[idx] as f32 / 256.0) / 256.0) - lin_val)
                                        * *corr_scale as f32,
                                );
                            }
                        }
                        None => println!("VCGT Gamma type 1 not implemented yet…"),
                    },
                    Err(err) => {
                        println!("Unable to read vcgt tag : {}", err);
                        return;
                    }
                };
            }
            "A2B0" | "A2B1" =>
            // 0x41324230
            // 0x42324131
            {
                match read_a2b_type(icc_raw_data, idx, size) {
                    Ok(a2b) => match a2b {
                        (Some(lut8), None) => {
                            println!("    {} Table ({:X})", tag_str, lut8.lut_sig);
                            println!("{}", lut8);
                        }
                        (None, Some(lut16)) => {
                            println!("    {} Table ({:X})", tag_str, lut16.lut_sig);
                            println!("{}", lut16);
                        }
                        _ => {
                            println!(
                                "Unable to read {} tag neither 8bits nor 16bits sig",
                                tag_str
                            );
                            return;
                        }
                    },
                    Err(err) => {
                        println!("Unable to read {} tag : {}", tag_str, err);
                        return;
                    }
                }
            }
            "B2A0" | "B2A1" =>
            // 0x42324130
            // 0x42324131
            {
                match read_b2a_type(icc_raw_data, idx, size) {
                    Ok(b2a) => match b2a {
                        (Some(lut8), None) => {
                            println!("    {} Table ({:X})", tag_str, lut8.lut_sig);
                            println!("{}", lut8);
                        }
                        (None, Some(lut16)) => {
                            println!("    {} Table ({:X})", tag_str, lut16.lut_sig);
                            println!("{}", lut16);
                        }
                        _ => {
                            println!(
                                "Unable to read {} tag neither 8bits nor 16bits sig",
                                tag_str
                            );
                            return;
                        }
                    },
                    Err(err) => {
                        println!("Unable to read {} tag : {}", tag_str, err);
                        return;
                    }
                }
            }
            "rXYZ" =>
            // 0x7258595A
            {
                let xyz_disp = match read_xyz_type(icc_raw_data, idx, size) {
                    Ok(xyz) => XYZNumberPrettyPrint(xyz),
                    Err(err) => {
                        println!("Unable to read Red matrix coordinates : {}", err);
                        return;
                    }
                };
                println!("    Red matrix coordinates : ");
                println!("      {}", xyz_disp);
            }
            "gXYZ" => {
                // 0x6758595A
                let xyz_disp = match read_xyz_type(icc_raw_data, idx, size) {
                    Ok(xyz) => XYZNumberPrettyPrint(xyz),
                    Err(err) => {
                        println!("Unable to read Green matrix coordinates : {}", err);
                        return;
                    }
                };
                println!("    Green matrix coordinates : ");
                println!("      {}", xyz_disp);
            }
            "bXYZ" => {
                //0x6258595A
                let xyz_disp = match read_xyz_type(icc_raw_data, idx, size) {
                    Ok(xyz) => XYZNumberPrettyPrint(xyz),
                    Err(err) => {
                        println!("Unable to read Blue matrix coordinates: {}", err);
                        return;
                    }
                };
                println!("    Blue matrix coordinates : ");
                println!("      {}", xyz_disp);
            }
            "rTRC" => {
                // 0x72545243
                let curve_type = match read_curve_type(icc_raw_data, idx) {
                    Ok(curve) => curve,
                    Err(err) => {
                        println!("Unable to read CurveType : {}", err);
                        return;
                    }
                };
                println!("    rTRC :");
                println!("{}", curve_type);
            }
            "gTRC" => {
                //0x67545243
                let curve_type = match read_curve_type(icc_raw_data, idx) {
                    Ok(curve) => curve,
                    Err(err) => {
                        println!("Unable to read CurveType : {}", err);
                        return;
                    }
                };
                println!("    gTRC :");
                println!("{}", curve_type);
            }
            "bTRC" => {
                //0x62545243
                let curve_type = match read_curve_type(icc_raw_data, idx) {
                    Ok(curve) => curve,
                    Err(err) => {
                        println!("Unable to read CurveType : {}", err);
                        return;
                    }
                };
                println!("    rTRC :");
                println!("{}", curve_type);
            }
            "chrm" => {
                let chrm_type = match read_chrm_type(icc_raw_data, idx) {
                    Ok(chrm) => chrm,
                    Err(err) => {
                        println!("Unable to read ChromaticityType : {}", err);
                        return;
                    }
                };
                println!("    Chromaticity :");
                println!("{}", chrm_type);
            }
            "mmod" => {
                let mmod_str = match read_mmod_type(icc_raw_data, idx) {
                    Ok(mmod) => MmodTypePrettyPrint(mmod),
                    Err(err) => {
                        println!("Error while parsing APPLE Make and Model tag : {}", err);
                        return;
                    }
                };
                println!("    APPLE Make and Model :");
                println!("{}", mmod_str);
            }
            _ => {
                let tag_sig_range: &[u8; 4] = match icc_raw_data[idx..=(idx + 3)].try_into() {
                    Ok(range) => range,
                    Err(err) => {
                        println!("Unable to get unknown u32 tag sig : {}", err);
                        return;
                    }
                };
                let tag_sig_u32 = u32::from_be_bytes(*tag_sig_range);
                let tag_sig_str = String::from_utf8_lossy(&icc_raw_data[idx..=(idx + 3)]);
                println!(
                    "! [Unknown signature {} ({:X}) : ({:X}) {}]",
                    tag.1, tag.0, tag_sig_u32, tag_sig_str
                );
            }
        }

        if tag_idx != (pf_tag_table_size - 1) {
            println!("----");
        }
    }
    println!("---- ----");
}
