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
	let mut count = 0;
	let mut count2 = 0;

	// testing if everything works correctly, before actually implementing proper code
	// Takes a while with this image. :)
	for elements in byte_values {
		println!("{}, {}: {}", count, count2, elements);
		count2 += 1;
		if count2 == 3 {
			count += 1;
			count2 = 0;
		}
	}
	println!("dimensions {:?}", dimensions);
}
