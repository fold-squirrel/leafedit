use lopdf::content::Operation;

pub fn draw_line(x1: u32, y1: u32, x2: u32, y2: u32) -> Vec<Operation> {
    vec![
        Operation::new("m", vec![x1.into(), y1.into()]),
        Operation::new("l", vec![x2.into(), y2.into()]),
        Operation::new("s", vec![]),
    ]
}

pub fn set_width(width: u32) -> Vec<Operation> {
    vec![
        Operation::new("w", vec![width.into()])
    ]
}
