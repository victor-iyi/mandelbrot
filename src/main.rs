mod img;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!(
            "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds: (usize, usize) =
        parser::parse_pair(&args[2], 'x').expect("Error parsing image dimensions.");
    let upper_left =
        parser::parse_complex(&args[3]).expect("Error parsing ujpper left corner point");
    let lower_right =
        parser::parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    img::render(&mut pixels, bounds, upper_left, lower_right);

    img::write_image(&args[1], &pixels, bounds).expect("Error writing PNG file.");
}
