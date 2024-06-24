use colour::Colour;

mod circle_degrees;
mod colour;
mod percentage_f32;

fn main() {
    let colour = Colour::new_rgb(86, 186, 114);

    println!("{:?}", colour.hsv_tuple());
}
