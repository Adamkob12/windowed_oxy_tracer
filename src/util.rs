use crate::vec3::*;
use image::{GenericImageView, Rgb, RgbImage};

pub const INF: f64 = 999999999999.0;
pub const EPSILON: f64 = 0.0000000001;

pub fn in_range<T: PartialOrd>(x: T, min: T, max: T) -> bool {
    min < x && x < max
}

pub fn v3_to_rgb(v: V3) -> Rgb<u8> {
    let r: u8 = v.x.clamp(0.0, 255.0).round() as u8;
    let g: u8 = v.y.clamp(0.0, 255.0).round() as u8;
    let b: u8 = v.z.clamp(0.0, 255.0).round() as u8;

    Rgb([r, g, b])
}

pub fn draw_pixel(
    img: &mut RgbImage,
    x: i32,
    y: i32,
    draw_color: Rgb<u8>,
) -> Result<(), (u32, u32)> {
    //let corrected_x: i32 = img.width() as i32 / 2 + x;
    //let corrected_y: i32 = img.height() as i32 / 2 - y;

    let (corrected_x, corrected_y) = middle_to_topleft(img, x, y);

    if !img.in_bounds(corrected_x as u32, (corrected_y - 1) as u32) {
        return Err((corrected_x as u32, corrected_y as u32));
    }

    //println!("Drawing at: x({}), y({})", corrected_x, corrected_y);

    img.put_pixel(corrected_x, corrected_y - 1, draw_color);
    Ok(())
}

//make pixel at position (0, 0) the middle of the canvas, instead of the top left
pub fn middle_to_topleft(img: &RgbImage, x: i32, y: i32) -> (u32, u32) {
    (
        (img.width() as i32 / 2 + x) as u32,  //x
        (img.height() as i32 / 2 - y) as u32, //y
    )
}

pub fn topleft_to_middle(img: &RgbImage, x: u32, y: u32) -> (i32, i32) {
    (
        x as i32 - (img.width() as i32 / 2),
        -(y as i32) + (img.height() as i32 / 2),
    )
}

pub fn topleft_to_middle_by_dims(width: i32, height: i32, x: u32, y: u32) -> (i32, i32) {
    (x as i32 - (width / 2), -(y as i32) + (height / 2))
}
