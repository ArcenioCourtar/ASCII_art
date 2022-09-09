#![allow(non_snake_case)] // because the crate/binary name is not in snake case :)

use image::GenericImageView;

fn main() {
	let img = match image::open("./images/my_github_avatar.jpg") {
		Ok(image) => image,
		Err(error) => panic!("Problem opening file: {:?}", error),
	};

	// save dimensions of image
	let dimensions = img.dimensions();
	// store RGB values in vector
	let byte_values: Vec<u8> = img.into_bytes();
	let mut brightness_values: Vec<u8> = Vec::new();
	let limit = byte_values.len();
	let mut count = 0;
	let mut count2 = 0;

	// testing if everything works correctly, before actually implementing proper code
	while count < limit {
		brightness_values.push(
			((byte_values[count as usize] as u32
				+ byte_values[count as usize + 1] as u32
				+ byte_values[count as usize + 2] as u32)
				/ 3) as u8,
		);
		count += 3;
	}
	// test code. I should learn how to write tests instead of this!
	count = 0;
	for elements in byte_values {
		// println!("{}, {}", count, elements);
		count += 1;
	}
	for elements in brightness_values {
		// println!("{}, {}", count2, elements);
		count2 += 1;
	}
	println!(
		"Count byte values: {} - Count brightness values: {}",
		count, count2
	);
	println!(
		"dimensions {:?} - Expected brightness values: {}",
		dimensions,
		(dimensions.0 * dimensions.1)
	);
}
