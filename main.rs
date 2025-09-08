use std::time::Instant;

#[derive(Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
    fn magnitude_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

fn mandelbrot_interactions(c: Complex, max_interactions: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..max_interactions {
        // the famous formula: z=z^2 + c
        z = z.multiply(&z,).add(&c);
        // If |z| > 2, the point escapes to infinity
        if z.magnitude_squared() > 4.0 {
            return i;
        }
    }
    max_interactions
}
fn pixel_to_complex(pixel_x: u32, pixel_y: u32, width: u32, height: u32) -> Complex {
    //Map pixel coordinates to complex plane
    // Standard view: real axis from -2.5 to 1.0, imaginary axis from -1.25 to 1.25
    let real = -2.5 + (pixel_x as f64 / width as f64) * 3.5;
    let imag = -1.25 + (pixel_y as f64 / height as f64) * 2.5;
    Complex::new(real, imag)
}

fn iterations_to_color(iterations: u32, max_iterations: u32) -> (u8, u8, u8) {
    if iterations == max_iterations {
        // Point is in the set - color it black
        (0, 0, 0)
    } else {
        //Point escaped - color based on how quickly it escaped

        let ratio = iterations as f64 / max_iterations as f64;
        let red = (255.0 * ratio) as u8;
        let green = (255.0 * ratio) as u8;
        let blue = (255.0 *ratio) as u8;
        (red, green, blue)
    }
}


fn generate_mandelbrot(width: u32, height: u32, max_iterations: u32) -> Vec<(u8, u8, u8)> {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    println!("Generating {}x{} Mandelbrot set with {} max iterations", width, height, max_iterations);
    println!("This is the computation we'll parallelize... \n");

    let start_time = Instant::now();
    let mut processed = 0;
    let total_pixels = width * height;

    for y in 0..height {
        for x in 0..width {
            //convert pixle coordinates to complex number
            let c = pixel_to_complex(x, y, width, height);

            //calculate how many iterations it takes to escape
            let iterations = mandelbrot_interactions(c, max_iterations);

            //calculate iterations to a color
            let color = iterations_to_color(iterations, max_iterations);
            pixels.push(color);

            processed += 1;

            //progerss inicator

            if processed % 1000 == 0 {
                let progress = (processed as f64 / total_pixels as f64) * 100.0;
                println!("Progress: {:.1}% ({}pixels)", progress, processed);
            }
        }
    }
    let duration = start_time.elapsed();
    println!("\nCompleted in {:?}", duration);
    println!("That's {:.2} Pixels per second", total_pixels as f64 / duration.as_secs_f64());

    pixels

}

fn save_as_ppm(pixels: &[(u8, u8, u8)], width: u32, height: u32, filename: &str) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Could not Create file");

    // PPM header
    writeln!(file, "P3").unwrap();
    writeln!(file, "{}{}", width, height).unwrap();
    writeln!(file, "225").unwrap();

    // Write pixel data

    for pixel in pixels {
        writeln!(file, "{}{}{}", pixel.0, pixel.1, pixel.2).unwrap();
    }
    println!("saved image as {}", filename);
}

fn main() {

    let width = 800;
    let height = 600;
    let max_iterations = 1000;

    println!("Single threaded generation of mandelbrot set");
    println!("============================================");

    let pixels = generate_mandelbrot(width, height, max_iterations);
    save_as_ppm(&pixels, width, height, "mandelbrot.ppm");

    println!("\nNow watch as the same computation gets parallelized");
    println!("\nEach pixel calculation is independent");
    println!("\nSome pixels (in the detailed border) take much longer than others.")
}