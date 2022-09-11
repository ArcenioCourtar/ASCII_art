#![allow(non_snake_case)] // because the crate/binary name is not in snake case, and I want ASCII in all caps. :(

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use native_dialog::FileDialog;
use std::fs;
use std::io::stdin;

fn main() {
	// Open the image, and resize if the width goes beyond value given
	// TODO: add command line arguments to change this value. currently always 100
	let img = open_file_resize(100);
	let dimensions = img.dimensions();

	// use .into_bytes() to save original image color data in a Vec
	// use value_calculation() to convert color data into something usable for ASCII art
	let original_values = img.into_bytes();
	// TODO: add different methods to calculate final value
	let recalc_values = value_calculation(original_values);

	// loads symbols stored in symbol_set.txt for use in the ASCII art
	// in case the file has no symbols in it or the file cannot be found, uses a hardcoded default
	let symbol_set = symbol_set_read();

	// prints the characters to the terminal to generate ASCII art.
	print_art(recalc_values, symbol_set, dimensions);
	
	// Waiting for user input, stop executable from closing instantly
	pause_window();
}

fn open_file_resize(size_boundary: u32) -> DynamicImage {
	let path = FileDialog::new()
		.set_location("~/Desktop")
		.show_open_single_file()
		.expect("How'd you do that?");

	// In case of no file being selected, panic!
	let path = match path {
		Some(path) => path,
		None => panic!("select a file please"),
	};

	// panic! when selecting a non-image file
	let mut img = match image::open(path) {
		Ok(image) => image,
		Err(error) => panic!("Problem opening file: {:?}", error),
	};

	// resize if width is higher than size_boundary. Otherwise do nothing.
	resize(&mut img, size_boundary);

	img
}

fn resize(img: &mut DynamicImage, target_size: u32) {
	let dimensions = img.dimensions();
	if dimensions.0 > target_size {
		*img = img.resize(target_size, 10000, FilterType::Gaussian);
	}
}

fn value_calculation(original_values: Vec<u8>) -> Vec<u8> {
	let mut recalc_values: Vec<u8> = Vec::new();
	let mut count = 0;

	// Add up RGB values and divide by three
	// TODO: learn how iterators work xd
	while count < original_values.len() - 2 {
		recalc_values.push(
			((original_values[count as usize] as u32
				+ original_values[count as usize + 1] as u32
				+ original_values[count as usize + 2] as u32)
				/ 3) as u8,
		);
		count += 3;
	}

	return recalc_values;
}

fn symbol_set_read() -> Vec<char> {
	let file_content_result = fs::read_to_string("./resources/symbol_set.txt");
	// if file is empty or file cannot be found, defaults to hardcoded list
	let file_content = match file_content_result {
		Ok(content) => {
			if content.len() > 0 {
				content
			} else {
				"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".to_string()
			}
		}
		_ => "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".to_string(),
	};
	let mut symbol_set: Vec<char> = Vec::new();
	for symbols in file_content.chars() {
		symbol_set.push(symbols);
	}
	symbol_set
}

fn print_art(recalc_values: Vec<u8>, symbol_set: Vec<char>, dimensions: (u32, u32)) {
	for (index, values) in recalc_values.iter().enumerate() {
		// This calculation determines what character equivalent should be printed for a pixel value of 0-255
		let character: usize = *values as usize * (symbol_set.len() - 1) / 255;
		if index % dimensions.0 as usize == 0 {
			print!("\n");
		}
		print!("{}", symbol_set[character])
	}
}

fn pause_window() {
	let mut input_string = String::new();
	println!("\nPress enter to close...");
    stdin().read_line(&mut input_string)
    	.ok();
}