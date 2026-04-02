# Image Processor

An application written in Rust to take a list of pixel values and produce a complete `.bmp` file with correct header.

## How to use

The project is still in development so there is no easy way to use the application yet.

### Steps to install and use

* Clone the repository: `git clone https://github.com/kyle-lindsay/Image-Processor`

*  Navigate to `src` directory: `cd src`

*  Open `main.rs`

*  Change `WIDTH` and `HEIGHT` constants to desired image size in pixels

*  Currently, I have a loop to create a grid of black and white pixels in a square, but you can populate `Vec<u8> image` with any bgr (rgb is reversed) values you want, seperated with a comma `,`

*  run `bash image_processor.sh` in the project root directory, not in `src`

*  This will produce `image.hex` and `image.bmp`

*  `image.hex` will be a list of hexadecimal values representing the image file

*  `image.bmp` will be the complete `.bmp` file with correct header
