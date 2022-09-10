# ASCII_art
###### Another project guided/inspired by https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/ 
###### This time around it's a program that prints ASCII art in the terminal based on an image you give
---
manual will be written when I've added more features! Currently it just opens `my_github_avatar.jpg` stored in the `images` folder and prints an ASCII art equivalent.

---
TODO:
* Core feature
- [x] Print ASCII art in the terminal based on the input image. Print an error message and panic!🦀 if the image can't be found.
* Usability/Customizability
- [x] Make characters used modifyable
	- [] Change default list to something more readable
	- [] Let the user define a character list in a separate .txt file or command line args
- [] Resize image automatically when exceeding a certain size
	- [] Let the user define resize boundary through command line args
- [] Let the user select an image upon running the program, instead of hardcoding the image to be selected
- [] Add multiple ways to recalculate brightness/color values

Changelog:
- 23/08/2022: 
	- First commit
- 09/09/2022 episode 1: 
	- Put image color data per pixel in a vector. As well as converting that to something usable when generating ASCII art
- 09/09/2022 episode 2:
	- Now prints the ASCII art in the Terminal. 
Will continue tomorrow!
