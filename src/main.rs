#![allow(non_snake_case)] // because the crate/binary name is not in snake case, and I want ASCII in all caps. :(

use image::GenericImageView;

fn main() {
	// Open the image
	// TODO: let user select image to open
	let img = match image::open("./images/my_github_avatar.jpg") {
		Ok(image) => image,
		Err(error) => panic!("Problem opening file: {:?}", error),
	};

	// save dimensions and color values. Then convert them to a single value to generate ASCII art.
	// TODO: automatically resize large images so the ASCII art always fits in the terminal.
	let dimensions = img.dimensions();
	let byte_values: Vec<u8> = img.into_bytes();
	let mut recalc_values: Vec<u8> = Vec::new();
	let mut count = 0;
	// TODO: use a for loop and StepBy() to iterate over the correct elements instead of using count += 3.
	// TODO: add multiple ways to calculate final value
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
