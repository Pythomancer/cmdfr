pub type Map = fn(f32, f32) -> usize;
const CHARLIST: [char; 7] = ['.', '-', ':', '*', 'o', '#', '%'];
pub fn char_map(val: usize) -> char {
    let spot = val as f32 / usize::MAX as f32;
    let spotu = (spot * CHARLIST.len() as f32).floor() as usize;
    CHARLIST[spotu.min(CHARLIST.len() - 1)]
}
pub const SILENT: bool = true;
pub fn print_page(
    x_center: f32,
    y_center: f32,
    x_rad: f32,
    y_rad: f32,
    linelen: usize,
    image: Map,
) {
    let sw = 2.0 * x_rad / linelen as f32;
    let mut x: f32 = x_center - x_rad;
    let mut y: f32 = y_center - y_rad;
    while y < y_center + y_rad {
        while x < x_center + x_rad {
            let o = char_map(image(x, y));
            if !SILENT {
                print!("{} ", o);
            }
            x += sw;
        }
        if !SILENT {
            println!();
        }
        y += sw;
        x = x_center - x_rad;
    }
}
