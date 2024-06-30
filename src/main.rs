use colour::Colour;

mod circle_degrees;
mod colour;
mod percentage_f32;

fn main() {
    let colour = Colour::from_hex(0x55f323);

    println!("{:?}", colour.hsv_tuple());
}
