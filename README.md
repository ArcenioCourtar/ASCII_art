# ASCII_art
###### Another project guided/inspired by https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/ 
###### This time around it's a program that prints ASCII art in the terminal based on an image you give
---
manual:

This program lets you select an image through your OS' file dialog and prints an ASCII_art equivalent in the terminal/console. 
You can change the symbols used to generate the art in `symbol_set.txt` found in the `resources` folder (left for "darker" spots and right for "brighter" spots). In case of an empty `symbol_set.txt`, or the file not being found, defaults to a hardcoded symbol set.
This is important when you want to convert images with less color variety, like images of cartoons/anime. 
Currently resizes images with a pixel width higher than 100 down to a width of 100, retaining aspect ratio. Will add feature to change this value with command line args.

---
BUGS:
Currently does not work with images that store transparency data or other data that's not just RBG values of each pixel (.png, .gif, etc).

---
TODO:
* Core feature
- [x] Print ASCII art in the terminal based on the input image. Print an error message and `panic!`🦀 if the image can't be found.
* BUGFIXES
- [ ] Account for image types that store more than just RGB values.
* Usability/Customizability
- [x] Make characters used modifyable
	- [x] Let the user define a character list in a separate `.txt` file
- [x] Resize image automatically when exceeding a certain size
	- [x] Retain original aspect ratio of image
		- [x] Fix shifting of ASCII art when converting an image that's not 1:1 
	- [ ] Let the user define resize boundary through command line args
- [x] Let the user select an image upon running the program, instead of hardcoding the image to be selected 
	- [ ] Replace instances of `panic!` with a message and a prompt asking if the user wants to select another image or terminate the program
- [ ] Add multiple ways to recalculate brightness/color values
	- [ ] Add brightness inversion option
* Refactoring
- [x] Wrap my code in separate functions
- [x] Make all variables in `main()` unmutable (Did this actually make my code better/safer? No clue, but it was a fun exercise)
- [x] Figure out how to not make the console window close immediately when running the executable

Changelog:
- 23/08/2022: 
	- First commit.
- 09/09/2022 episode 1: 
	- Put image color data per pixel in a vector. As well as converting that to something usable when generating ASCII art.
- 09/09/2022 episode 2:
	- Now prints the ASCII art in the Terminal. 
- 10/09/2022: 
	- Added file dialog (should work cross platform).
	- Added resizing functionality.
- 11/09/2022:
	- Refactored the code, fixed some bugs
	- Added option for user to change symbols in symbol_set.txt
	- Removed example image, since user can now select images manually
