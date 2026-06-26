# ICC Reader: A Fast and Reliable ICC Profile Parser

A Rust utility for parsing and analyzing ICC profiles. This tool is designed to be fast and reliable, providing pretty-printed output for ICC profile data. It also includes a feature to calculate the delta of correction for achieving the correct gamma of the three primaries, which is useful for manually calibrating devices that are not compatible with ICC profiles.

## Features

- Parse ICC profiles and display the results in a pretty-printed format.
- Calculate and display the delta of correction for VCGT to achieve the correct gamma of the three primaries.

## Installation

To build and run this project:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository.
3. Run:
  ```sh
   cargo build --release
  ```

## Usage

```sh
icc-reader <icc_profile_file_name> [number_of_points_to_extract_from_vcgt] [correction_scale_in_your_monitor]
```

- `<icc_profile_file_name>`: Path to the ICC profile file.
- `[number_of_points_to_extract_from_vcgt]`: (Optional) Number of points to extract from the VCGT tag.
- `[correction_scale_in_your_monitor]`: (Optional) Correction scale value for your monitor.

**Example:**

```sh
icc-reader my_profile.icc 20 25
```

The program will extract 20 points along 0 to 1, and your monitor can adjust the value from -25 to +25.

**Output example for the vcgt:**

```
      Correction scale +/- 25
      In (RGB) -> Out (R,G,B)
      0.00  0, 0, 0
      0.05  0, 0, 0
      0.10  1, 1, 0
      0.15  1, 1, 0
      0.20  1, 1, 0
      0.25  1, 1, 0
      0.30  1, 1, 0
      0.36  1, 1, 0
      0.41  1, 1, 0
      0.46  1, 1, 0
      0.51  1, 1, 0
      0.56  1, 1, 0
      0.61  0, 0, 0
      0.66  0, 0, 0
      0.71  0, 0, 0
      0.76  0, 0, 0
      0.81  -0, -0, 0
      0.86  -0, -0, -0
      0.91  -0, -0, -0
      0.96  -0, -0, -0
```

## Roadmap

- Export profile information to JSON.
- Export curves to CSV.
- Add support for additional tags and types omitted in the prototype.
- Improve CLI arguments for better usability.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the [EUPL 1.2 License](LICENSE).
