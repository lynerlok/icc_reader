use crate::print_utils::print_usage;
use core::str;
use std::env;
use std::fs::File;
use std::io::Read;

mod print_utils;

fn get_date_time_number(date: &[u8; 12]) -> String {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage(&args);
        return;
    }

    println!("----- ICC READER -----");

    let filename = &args[1];

    match File::open(filename) {
        Ok(mut fd) => {
            let mut icc_raw_data: Vec<u8> = vec![];
            match fd.read_to_end(&mut icc_raw_data) {
                Ok(size) => {
                    println!("File Size : {}", size);
                    println!("---- Profile info ----");

                    let pf_size: u32 = u32::from_be_bytes(icc_raw_data[0..=3].try_into().unwrap());
                    println!("Profile Size: {:?} bytes", pf_size);

                    let pf_cmm_type_sig =
                        String::from_utf8_lossy(&icc_raw_data[4..=7]).to_uppercase();
                    println!("Profile CMM Type : {}", pf_cmm_type_sig);

                    let pf_major_rev = &icc_raw_data[8];
                    let pf_minor_rev = &icc_raw_data[9] >> 4;
                    let pf_fix_rev = (&icc_raw_data[9] << 4) >> 4;
                    println!(
                        "Profile Revision : {}.{}.{}",
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
                    println!("Class : {} profile", pf_cls_sig);

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
                    println!("Color Space of data : {}", pf_data_color_space);

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
                    println!("Profile Connection Space : {}", pf_conn_space);

                    let pf_date = get_date_time_number(&icc_raw_data[24..=35].try_into().unwrap());
                    println!("Profile creation date : {}", pf_date);

                    if u32::from_be_bytes(icc_raw_data[36..=39].try_into().unwrap()) != 0x61637370 {
                        println!("Profile file is not 'acsp'. File may be corrupted exiting…");
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
                    println!("Primary plateform : {}", pf_primary_sig);
                    let pf_flags_msb =
                        match (u32::from_be_bytes(icc_raw_data[44..=47].try_into().unwrap()) >> 16)
                            << 16
                        {
                            0 => "Not embedded, can used independently",
                            _ => "Unknown",
                        };
                    //let pf_dev_manuf = 0;
                    //let pf_dev_model = 0;
                    //let pf_dev_attrs = 0;
                    //let pf_render_int = 0;
                    //let pf_xyz_illum = 0;
                    //let pf_creator_sig = 0;
                }
                Err(err) => println!("Error while reading file ({}): {}", filename, err),
            }
        }
        Err(err) => println!("{} : {}", err, filename),
    }
}
