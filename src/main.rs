use std::env;

mod parsers;
mod types;
mod utils;

use crate::utils::print::print_usage;

use std::fs::File;
use std::io::Read;

use crate::parsers::profile_info::parse_profile_info;
use crate::parsers::tags::parse_icc_tags;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 4 || args.len() < 2 {
        print_usage(&args);
        return;
    }

    let filename = &args[1];

    let mut pt_num: usize = 20;
    let mut corr_scale: usize = 25;

    if args.len() != 2 {
        pt_num = match args[2].parse::<usize>() {
            Ok(usize) => usize,
            Err(err) => {
                println!("Unable to parse pt_num arg : {}", err);
                return;
            }
        };
        corr_scale = match args[3].parse::<usize>() {
            Ok(usize) => usize,
            Err(err) => {
                println!("Unable to parse corr_scale arg : {}", err);
                return;
            }
        };
    }

    println!("----- ICC READER -----\n");

    match File::open(filename) {
        Ok(mut fd) => {
            let mut icc_raw_data: Vec<u8> = vec![];
            match fd.read_to_end(&mut icc_raw_data) {
                Ok(..) => {
                    parse_profile_info(&icc_raw_data);
                    parse_icc_tags(&icc_raw_data, &pt_num, &corr_scale);
                }
                Err(err) => println!("Error while reading file ({}): {}", filename, err),
            }
        }
        Err(err) => println!("{} : {}", err, filename),
    }
}
