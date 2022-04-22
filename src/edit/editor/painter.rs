use crate::commadline::Color;
use lopdf::content::Operation;

pub fn change_color(color: Color) -> Vec<Operation> {
    match color {
        Color::White => vec![Operation::new("rg", vec![1.into(), 1.into(), 1.into()])],
        Color::Red => vec![Operation::new("rg", vec![1.into(), 0.into(), 0.into()])],
        Color::Green => vec![Operation::new("rg", vec![0.into(), 1.into(), 0.into()])],
        Color::Blue => vec![Operation::new("rg", vec![0.into(), 0.into(), 1.into()])],
        Color::Black => vec![Operation::new("rg", vec![0.into(), 0.into(), 1.into()])],
    }
}

pub fn change_rgb(red: u8, green: u8, blue: u8) -> Vec<Operation> {
    let r = red as f32/255_f32; let g = green as f32/255_f32; let b = blue as f32/255_f32;
    vec![Operation::new("rg", vec![r.into(), g.into(), b.into()])]
}
