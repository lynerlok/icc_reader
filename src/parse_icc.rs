use core::str;
use std::fs::File;
use std::io::Read;

use crate::bytes_utils::get_bits_of_byte;
use crate::icc_utils::get_date_time_number;
use crate::read_icc_types::*;
use crate::rs_types::{S15Fixed16Number, XYZNumber};

pub fn parse_icc(filename: &str, pt_num: &usize, corr_scale: &usize) {
    println!("----- ICC READER -----");

    match File::open(filename) {
        Ok(mut fd) => {
            let mut icc_raw_data: Vec<u8> = vec![];
            match fd.read_to_end(&mut icc_raw_data) {
                Ok(size) => {
                    println!("File Size : {}", size);
                    println!("---- Profile info ----");

                    let pf_size: u32 = u32::from_be_bytes(icc_raw_data[0..=3].try_into().unwrap());
                    println!("    Profile Size: {:?} bytes", pf_size);

                    let pf_cmm_type_sig =
                        String::from_utf8_lossy(&icc_raw_data[4..=7]).to_uppercase();
                    println!("    Profile CMM Type : {}", pf_cmm_type_sig);

                    let pf_major_rev = &icc_raw_data[8];
                    let pf_minor_rev = &icc_raw_data[9] >> 4;
                    let pf_fix_rev = (&icc_raw_data[9] << 4) >> 4;
                    println!(
                        "    Profile Revision : {}.{}.{}",
                        pf_major_rev, pf_minor_rev, pf_fix_rev
                    );

                    let pf_cls_sig =
                        match u32::from_be_bytes(icc_raw_data[12..=15].try_into().unwrap()) {
                            0x73636E72 => "Input Device",
                            0x6D6E7472 => "Display Device",
                            0x70727472 => "Output Device",
                            0x6C696E6B => "DeviceLink",
                            0x73706163 => "ColorSpace Conversion",
                            0x61627374 => "Abstract",
                            0x6E6D636C => "Named Color",
                            _ => "Unknown",
                        };
                    println!("    Class : {} profile", pf_cls_sig);

                    let pf_data_color_space =
                        match u32::from_be_bytes(icc_raw_data[16..=19].try_into().unwrap()) {
                            0x58595A20 => "XYZ",
                            0x4C616220 => "Lab",
                            0x4C757620 => "Luv",
                            0x59436272 => "YCbr",
                            0x59787920 => "Yxy",
                            0x52474220 => "RGB",
                            0x47524159 => "GRAY",
                            0x48535620 => "HSV",
                            0x484C5320 => "HLS",
                            0x434D594B => "CMYK",
                            0x434D5920 => "CMY",
                            0x32434C52 => "2CLR",
                            0x33434C52 => "3CLR",
                            0x34434C52 => "4CLR",
                            0x35434C52 => "5CLR",
                            0x36434C52 => "6CLR",
                            0x37434C52 => "7CLR",
                            0x38434C52 => "8CLR",
                            0x39434C52 => "9CLR",
                            0x41434C52 => "ACLR",
                            0x42434C52 => "BCLR",
                            0x43434C52 => "CCLR",
                            0x44434C52 => "DCLR",
                            0x45434C52 => "ECLR",
                            0x46434C52 => "FCLR",
                            _ => "Unknown",
                        };
                    println!("    Color Space of data : {}", pf_data_color_space);

                    let pf_conn_space =
                        match u32::from_be_bytes(icc_raw_data[20..=23].try_into().unwrap()) {
                            0x58595A20 => "XYZ",
                            0x4C616220 => "Lab",
                            0x4C757620 => "Luv",
                            0x59436272 => "YCbr",
                            0x59787920 => "Yxy",
                            0x52474220 => "RGB",
                            0x47524159 => "GRAY",
                            0x48535620 => "HSV",
                            0x484C5320 => "HLS",
                            0x434D594B => "CMYK",
                            0x434D5920 => "CMY",
                            0x32434C52 => "2CLR",
                            0x33434C52 => "3CLR",
                            0x34434C52 => "4CLR",
                            0x35434C52 => "5CLR",
                            0x36434C52 => "6CLR",
                            0x37434C52 => "7CLR",
                            0x38434C52 => "8CLR",
                            0x39434C52 => "9CLR",
                            0x41434C52 => "ACLR",
                            0x42434C52 => "BCLR",
                            0x43434C52 => "CCLR",
                            0x44434C52 => "DCLR",
                            0x45434C52 => "ECLR",
                            0x46434C52 => "FCLR",
                            _ => "Unknown",
                        };
                    println!("    Profile Connection Space : {}", pf_conn_space);

                    let pf_date = get_date_time_number(&icc_raw_data[24..=35].try_into().unwrap());
                    println!("    Profile creation date : {}", pf_date);

                    if u32::from_be_bytes(icc_raw_data[36..=39].try_into().unwrap()) != 0x61637370 {
                        println!("    Profile file is not 'acsp'. File may be corrupted exiting…");
                        return;
                    }

                    let pf_primary_sig =
                        match u32::from_be_bytes(icc_raw_data[40..=43].try_into().unwrap()) {
                            0x4150504C => "Apple Computer, Inc.",
                            0x4D534654 => "Microsoft Corporation",
                            0x53474920 => "Silicon Graphics, Inc.",
                            0x53554E57 => "Sun Microsystems, Inc.",
                            0x54474E54 => "Taligent, Inc.",
                            0x2A6E6978 => "*nix",
                            _ => {
                                &("Unknown (".to_owned()
                                    + str::from_utf8(&icc_raw_data[40..=43]).unwrap()
                                    + ")")
                            }
                        };
                    println!("    Primary plateform : {}", pf_primary_sig);
                    let pf_flags_msb =
                        match u16::from_be_bytes(icc_raw_data[44..=45].try_into().unwrap()) {
                            0 => "Not embedded, can used independently",
                            1 => "Embedded, cannot use independently",
                            2 => "Not embedded, can used independently",
                            3 => "Embedded, can use independently",
                            _ => "Unknown",
                        };
                    println!("    Profile tags : {}", pf_flags_msb);
                    let pf_dev_manuf =
                        u32::from_be_bytes(icc_raw_data[48..=51].try_into().unwrap());
                    let pf_dev_model =
                        u32::from_be_bytes(icc_raw_data[52..=55].try_into().unwrap());
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
                    let pf_creator_sig =
                        String::from_utf8_lossy(&icc_raw_data[80..=83]).to_uppercase();
                    let pf_creator_sig_hex =
                        u32::from_be_bytes(icc_raw_data[80..=83].try_into().unwrap());
                    println!(
                        "    Profile Creator : 0x{:X} ({})",
                        pf_creator_sig_hex, pf_creator_sig
                    );
                    println!("---- End of Profile info ----");

                    println!("---- TAG Table ----");
                    let pf_tag_table_size: usize =
                        u32::from_be_bytes(icc_raw_data[128..=131].try_into().unwrap())
                            .try_into()
                            .unwrap();

                    println!("    Number of tags in the file : {}", pf_tag_table_size);
                    let mut tags: Vec<(u32, String, u32, u32)> =
                        Vec::with_capacity(pf_tag_table_size);
                    for (idx, tag_num) in (0..(pf_tag_table_size)).enumerate() {
                        let tag_idx: usize = 132 + (tag_num * 12);
                        let tag_sig = u32::from_be_bytes(
                            icc_raw_data[tag_idx..=(tag_idx + 3)].try_into().unwrap(),
                        );
                        let tag_str =
                            String::from_utf8_lossy(&icc_raw_data[tag_idx..=(tag_idx + 3)]);
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
                    for tag in tags {
                        let idx: usize = usize::try_from(tag.2).unwrap();
                        let size: usize = usize::try_from(tag.3).unwrap();
                        match tag.0 {
                            0x64657363 => {
                                println!("    Description :");
                                let (ascii_desc_str, utf_desc_str) =
                                    match read_text_desc_type(&icc_raw_data, idx) {
                                        Ok(desc) => desc,
                                        Err(err) => {
                                            println!("Error while parsing ICC desc tag : {}", err);
                                            return;
                                        }
                                    };
                                println!("      ASCII : {}", ascii_desc_str,);
                                println!("      UTF16 : {}", utf_desc_str);
                            }
                            0x63707274 => {
                                println!("    Copyright :");
                                let (cprt_sig, cprt_str) =
                                    match read_text_type(&icc_raw_data, idx, size) {
                                        Ok(cprt) => cprt,
                                        Err(err) => {
                                            println!("Error while parsing copyright tag : {}", err);
                                            return;
                                        }
                                    };
                                println!("      ({:X}) {}", cprt_sig, cprt_str);
                            }
                            0x646D6E64 => {
                                let (dmnd_ascii_str, dmnd_utf_str) =
                                    match read_text_desc_type(&icc_raw_data, idx) {
                                        Ok(dmnd) => dmnd,
                                        Err(err) => {
                                            println!(
                                                "Error while parsing device manufacturer tag : {}",
                                                err
                                            );
                                            return;
                                        }
                                    };
                                println!("    Device Manufacturer name :");
                                println!("      ASCII : {}", dmnd_ascii_str);
                                println!("      UTF16 : {}", dmnd_utf_str);
                            }
                            0x77747074 => {
                                let (xyz_sig, xyz_number) =
                                    match read_xyz_type(&icc_raw_data, idx, size) {
                                        Ok(xyz) => xyz,
                                        Err(err) => {
                                            println!(
                                                "Unable to read Media white point illuminant : {}",
                                                err
                                            );
                                            return;
                                        }
                                    };
                                println!("    Media white point illuminant : ");
                                println!(
                                    "      ({:X}) {:.2} {:.2} {:.2}",
                                    xyz_sig, xyz_number[0], xyz_number[1], xyz_number[2]
                                );
                            }
                            0x626B7074 => {
                                let (xyz_sig, xyz_number) =
                                    match read_xyz_type(&icc_raw_data, idx, size) {
                                        Ok(xyz) => xyz,
                                        Err(err) => {
                                            println!(
                                                "Unable to read Media black point illuminant : {}",
                                                err
                                            );
                                            return;
                                        }
                                    };
                                println!("    Media black point illuminant : ");
                                println!(
                                    "      ({:X}) {:.4} {:.4} {:.4}",
                                    xyz_sig, xyz_number[0], xyz_number[1], xyz_number[2]
                                );
                            }
                            0x44657644 => {
                                let (txt_sig, txt_str) =
                                    match read_text_type(&icc_raw_data, idx, size) {
                                        Ok(txt) => txt,
                                        Err(err) => {
                                            println!(
                                            "Unable to read characterization device values : {}",
                                            err
                                        );
                                            return;
                                        }
                                    };
                                println!("    Characterizaton device values (ti3 file): ");
                                println!("      ({:X}) [{} Bytes]", txt_sig, txt_str.len() - 1);
                            }
                            0x43494544 => {
                                let (txt_sig, txt_str) = match read_text_type(
                                    &icc_raw_data,
                                    idx,
                                    size,
                                ) {
                                    Ok(txt) => txt,
                                    Err(err) => {
                                        println!(
                                            "Unable to read characterization measurement values : {}",
                                            err
                                        );
                                        return;
                                    }
                                };
                                println!("    Characterizaton measurement values (ti3 file): ");
                                println!("      ({:X}) [{} Bytes]", txt_sig, txt_str.len() - 1);
                            }
                            0x74617267 => {
                                let (txt_sig, txt_str) =
                                    match read_text_type(&icc_raw_data, idx, size) {
                                        Ok(txt) => txt,
                                        Err(err) => {
                                            println!(
                                                "Unable to read characterization target : {}",
                                                err
                                            );
                                            return;
                                        }
                                    };
                                println!("    Characterizaton target (ti3 file): ");
                                println!("      ({:X}) [{} Bytes]", txt_sig, txt_str.len() - 1);
                            }
                            0x6C756D69 => {
                                let (xyz_sig, xyz_number) =
                                    match read_xyz_type(&icc_raw_data, idx, size) {
                                        Ok(xyz) => xyz,
                                        Err(err) => {
                                            println!("Unable to read luminance : {}", err);
                                            return;
                                        }
                                    };
                                println!("    Luminance : ");
                                println!("      ({:X}) {:.2} cd/m²", xyz_sig, xyz_number[1] / 100);
                            }
                            0x6D657461 => {
                                let (dict_sig, dict) =
                                    match read_dict_type(&icc_raw_data, idx, size) {
                                        Ok(dict) => dict,
                                        Err(err) => {
                                            println!("Unable to read dist : {}", err);
                                            return;
                                        }
                                    };
                                println!("    Metadatas : ({:X})", dict_sig);
                                for (key, value) in dict {
                                    println!("      {} | {}", key, value);
                                }
                            }
                            0x61727473 => {
                                let (sf32_sig, sf32_vec) =
                                    match read_sf32_type(&icc_raw_data, idx, size) {
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
                            0x76636774 => {
                                match read_vcgt_type(&icc_raw_data, idx, size) {
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
                                            let step: usize =
                                                ((1.0 / *pt_num as f32) * 256.0).ceil() as usize;
                                            for idx in (0..chan_len).step_by(step) {
                                                let lin_val: f32 = idx as f32 / 256.0;
                                                println!(
                                                    "      {:.2}  {:.0}, {:.0}, {:.0}",
                                                    idx as f32 / 256.0,
                                                    (((vcgt.1[idx] as f32 / 256.0) / 256.0)
                                                        - lin_val)
                                                        * *corr_scale as f32,
                                                    (((vcgt.1[idx] as f32 / 256.0) / 256.0)
                                                        - lin_val)
                                                        * *corr_scale as f32,
                                                    (((vcgt.3[idx] as f32 / 256.0) / 256.0)
                                                        - lin_val)
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
                            _ => {
                                let tag_sig_range: &[u8; 4] =
                                    match icc_raw_data[idx..=(idx + 3)].try_into() {
                                        Ok(range) => range,
                                        Err(err) => {
                                            println!("Unable to get unknown u32 tag sig : {}", err);
                                            return;
                                        }
                                    };
                                let tag_sig_u32 = u32::from_be_bytes(*tag_sig_range);
                                let tag_sig_str =
                                    String::from_utf8_lossy(&icc_raw_data[idx..=(idx + 3)]);
                                println!(
                                    "Unknown signature {} ({:X}) : ({:X}) {}",
                                    tag.1, tag.0, tag_sig_u32, tag_sig_str
                                );
                            }
                        }
                    }
                    println!("---- ----");
                }
                Err(err) => println!("Error while reading file ({}): {}", filename, err),
            }
        }
        Err(err) => println!("{} : {}", err, filename),
    }
}
