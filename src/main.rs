use maps::*;
use pages::print_page;
pub mod noise;
pub mod complex;
pub mod maps;
pub mod pages;
fn main() {
    // say_hello();
    // let dots = noise::gen_map(8);
    let now = std::time::Instant::now();
    print_page(0.0, 0.0, 1.0, 1.0, 200, fractal_pixel);
    println!("{}", now.elapsed().as_millis());
}
