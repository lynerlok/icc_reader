use std::env;

mod bytes_utils;
mod icc_utils;
mod parse_icc;
mod print_utils;
mod read_icc_types;
mod rs_types;

use crate::print_utils::print_usage;
use parse_icc::parse_icc;

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

    parse_icc(filename, &pt_num, &corr_scale);
}
