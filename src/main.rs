#![allow(non_snake_case)] // because the crate/binary name is not in snake case, and I want ASCII in all caps. :(

use image::GenericImageView;
use image::imageops::FilterType;
use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() {
	// Open the image
	let path = FileDialog::new()
	.set_location("~/Desktop")
	.show_open_single_file()
	.unwrap();

	let path = match path {
        Some(path) => path,
        None => return,
    };

	let mut img = match image::open(path) {
		Ok(image) => image,
		Err(error) => panic!("Problem opening file: {:?}", error),
	};

	// save dimensions and color values. Then convert them to a single value to generate ASCII art.
	let mut dimensions = img.dimensions();
	// resize image if it's dimensions exceed a certain boundary
	if dimensions.0 > 30 {
		img = img.resize(30, 30, FilterType::Gaussian);
		dimensions = (30, 30);
	}
	let byte_values: Vec<u8> = img.into_bytes();
	let mut recalc_values: Vec<u8> = Vec::new();
	let mut count = 0;
	// TODO: use a for loop to iterate over the correct elements instead of using count += 3.
	// TODO: add multiple ways to calculate values for recalc_values

	while count < byte_values.len() {
		recalc_values.push(
			((byte_values[count as usize] as u32
				+ byte_values[count as usize + 1] as u32
				+ byte_values[count as usize + 2] as u32)
				/ 3) as u8,
		);
		count += 3;
	}

	// Characters used to generate the art. I cannot index directly into a string like in C.
	// So I put all the characters of this string into a Vec I can index into.
	// You can modify this list without problem. As long as the length is not 0.
	// TODO: read list from an external file
	let char_list = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
	let mut char_list_vec: Vec<char> = Vec::new();
	for characters in char_list.chars() {
		char_list_vec.push(characters);
	}

	// prints the characters to the terminal to generate ASCII art.
	for (index, values) in recalc_values.iter().enumerate() {
		// This calculation determines what character equivalent should be printed for a pixel value of 0-255
		let character: usize = *values as usize * (char_list_vec.len() - 1) / 255;
		if index % dimensions.0 as usize == 0 {
			print!("\n");
		}
		print!("{}", char_list_vec[character])
	}
}
