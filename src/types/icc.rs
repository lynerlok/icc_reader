use std::fmt::Display;

use fixed::types::extra::{U16, U8};
use fixed::{FixedI32, FixedU16, FixedU32};

pub type S15Fixed16Number = FixedI32<U16>;
pub type U8Fixed8Number = FixedU16<U8>;
pub type U16Fixed16Number = FixedU32<U16>;
pub type XYZNumber = [S15Fixed16Number; 3];

pub struct XYZNumberPrettyPrint(pub (u32, XYZNumber));

impl Display for XYZNumberPrettyPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:X}) {:.2} {:.2} {:.2}",
            self.0 .0, self.0 .1[0], self.0 .1[1], self.0 .1[2]
        )
    }
}
pub type S15Fixed16Array = Vec<S15Fixed16Number>;
pub type Vcgt = (
    u32,      // vcgt_sig
    Vec<u16>, // red ramp
    Vec<u16>, // green ramp
    Vec<u16>, // blue ramp
    Vec<f32>, // max value for each channel
    Vec<f32>, // min value for each channel
    usize,    // # channels
    usize,    // # entries per channel
    usize,    // bitdepth
);

pub struct Lut8 {
    pub lut_sig: u32,                    // lut_sig
    pub in_chans_num: u8,                // # Inputs channels
    pub out_chans_num: u8,               // # Outputs channels
    pub grid_pts_num: u8,                // # of grid points
    pub e_params: Vec<S15Fixed16Number>, // Encoded parameters
    pub in_table: Vec<u8>,               // Input table
    pub clut_table: Vec<u8>,             // CLUT table
    pub out_table: Vec<u8>,              // Output table
}

impl Display for Lut8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "      # Inputs channels : {}\n      # Outputs channels : {}\n      # CLut grid points : {}\n      Encoded parameters matrix {:?}\n      Input table : [{} Bytes]\n      Clut table : [{} Bytes]\n      Output table : [{} Bytes]",
            self.in_chans_num,
            self.out_chans_num,
            self.grid_pts_num,
            self.e_params,
            self.in_table.len(),
            self.clut_table.len(),
            self.out_table.len(),
        )
    }
}

pub struct Lut16 {
    pub lut_sig: u32,                    // lut_sig
    pub in_chans_num: u8,                // # Inputs channels
    pub out_chans_num: u8,               // # Outputs channels
    pub grid_pts_num: u8,                // # of grid points
    pub e_params: Vec<S15Fixed16Number>, // Encoded parameters
    pub in_table_entries_num: u16,       // # Input table entries
    pub out_table_entries_num: u16,      // # Output table entries
    pub in_table: Vec<u16>,              // Input table
    pub clut_table: Vec<u16>,            // CLUT table
    pub out_table: Vec<u16>,             // Output table
}

impl Display for Lut16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "      # Inputs channels : {}\n      # Outputs channels : {}\n      # CLut grid points : {}\n      Encoded parameters matrix {:?}\n      # Input table entries : {}\n      # Output table entries : {}\n      Input table : [{} Bytes]\n      Clut table : [{} Bytes]\n      Output table : [{} Bytes]",
            self.in_chans_num,
            self.out_chans_num,
            self.grid_pts_num,
            self.e_params,
            self.in_table_entries_num,
            self.out_table_entries_num,
            self.in_table.len(),
            self.clut_table.len(),
            self.out_table.len(),
        )
    }
}

pub type Lut = (Option<Lut8>, Option<Lut16>);

pub type DescType = (String, String);

pub struct DescTypePrettyPrint(pub DescType);

impl Display for DescTypePrettyPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASCII : {} \n      UTF16 : {}", self.0 .0, self.0 .1)
    }
}

pub struct TxtTypePrettyPrinter(pub (u32, String));

impl Display for TxtTypePrettyPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:X}) {}", self.0 .0, self.0 .1)
    }
}

pub struct TxtTypeTi3PrettyPrinter(pub (u32, String));

impl Display for TxtTypeTi3PrettyPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:X}) [{} Bytes]", self.0 .0, self.0 .1.len() - 1)
    }
}

//pub struct A2B {
//    in_chans_num: u8,
//    out_chans_num: u8,
//    b_curve:
//}

pub struct Curve {
    pub identity: bool,
    pub gamma: Option<U8Fixed8Number>,
    pub curve: Option<Vec<u16>>,
}

impl Display for Curve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.identity {
            return write!(f, "      Curve is identity curve");
        };

        if let Some(gamma) = self.gamma {
            return write!(f, "      Curve is a gamma curve : {}", gamma);
        };

        if let Some(curve) = &self.curve {
            return write!(
                f,
                "      Curve is an unidimensional transfer curve : [{} elems]",
                curve.len()
            );
        };

        Err(std::fmt::Error)
    }
}

pub struct Chrm {
    pub phs_col_type: String,
    pub chans_num: u16,
    pub chan_1: Option<(U16Fixed16Number, U16Fixed16Number)>,
    pub chan_2: Option<(U16Fixed16Number, U16Fixed16Number)>,
    pub chan_3: Option<(U16Fixed16Number, U16Fixed16Number)>,
}

impl Display for Chrm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.chans_num {
        1 => write!(f, "      Encoded value of phosphor or colorant type : {}\n      CIE xy channel 1 : {:?}", self.phs_col_type, self.chan_1.unwrap()),
        2 => write!(f, "      Encoded value of phosphor or colorant type : {}\n      CIE xy channel 1 : {:?}\n      CIE xy channel 2 : {:?}", self.phs_col_type, self.chan_1.unwrap(), self.chan_2.unwrap()),
        3 => write!(f, "      Encoded value of phosphor or colorant type : {}\n      CIE xy channel 1 : {:?}\n      CIE xy channel 2 : {:?}\n      CIE xy channel 3 : {:?}", self.phs_col_type, self.chan_1.unwrap(), self.chan_2.unwrap(),self.chan_3.unwrap()),
        _ => panic!("Chromaticity channels numbers > 3")
        }
    }
}

pub type MmodType = (u16, u16);

pub struct MmodTypePrettyPrint(pub MmodType);

impl Display for MmodTypePrettyPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "      Manufacturer : 0x{:X?}\n      Model : 0x{:X?}",
            self.0 .0, self.0 .1
        )
    }
}
