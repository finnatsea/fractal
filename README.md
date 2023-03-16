# Mandelbrot Fractal Generator

A simple Rust program that generates a Mandelbrot fractal image and saves it as a grayscale PNG file.

## Dependencies

- [image](https://crates.io/crates/image) (version 0.24.5) for image manipulation and saving.
- [num-complex](https://crates.io/crates/num-complex) (version 0.4) for complex number calculations.

## Usage

1. Install Rust and Cargo, if you haven't already, by following the instructions on the [official website](https://www.rust-lang.org/tools/install).
2. Clone this repository:

```sh
git clone https://github.com/finnatsea/mandelbrot-fractal-generator.git
cd mandelbrot-fractal-generator
```

3. Run the program:
  
  ```sh
  cargo run
  ```
  The program will generate a grayscale Mandelbrot fractal image with dimensions 800x800 pixels and save it as a PNG file called "mandelbrot.png" in the same directory.



Customization
You can customize the image by changing the following variables in the `main.rs` file:

- `width`: The width of the generated image.
- `height`: The height of the generated image.
- `max_iterations`: The maximum number of iterations for determining if a point is in the Mandelbrot set or not.

After making changes, run the program again using `cargo run`.