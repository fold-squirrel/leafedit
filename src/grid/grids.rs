use crate::commadline::{GridType as Grid, Opr, Color};
use lopdf::{Error as LopdfError, Object, Document};

use crate::info::display;

const COLOURS: [(u8, u8, u8); 7] = [
    to_rgb(0x5F4149),
    to_rgb(0xEB3247),
    to_rgb(0xBC4349),
    to_rgb(0xF35645),
    to_rgb(0xF6A73B),
    to_rgb(0xFAD32F),
    to_rgb(0xa6896b),
];

pub fn generate(grid: Grid) -> Result<(), LopdfError> {

    let (file, save) = match grid {
        Grid::Full { ref file, ref save_as } => (file, save_as),
        Grid::Sub { ref file, ref save_as, .. } => (file, save_as),
        Grid::Mark { ref file, ref save_as, .. } => (file, save_as)
    };

    let mut doc = Document::load(file)?;
    let (width, height) = display::from_doc_get_page_width_height(&doc, (4, 0));
    let oprs = match grid {
        Grid::Full { .. } => full(width as u32, height as u32),
        Grid::Sub { x, y, .. } => sub(width as u32, height as u32, x, y),
        Grid::Mark { x, y, rotate, .. } => mark(rotate, x, y),
    };

    crate::edit::apply::edits_on_doc(&mut doc, oprs)?;
    doc.save(save)?;
    println!("gridded");
    Ok(())
}

fn mark(rotate_text: bool, x: u32, y: u32) -> Vec<Opr> {
    let text = format!("({}, {})", x, y);
    let mut oprs = vec![
        Opr::ChangeColor(Color::Grey),
        Opr::Raw("w".to_string(), vec![0.5.into()]),
        Opr::DrawLine(x + 4, y, x - 4, y),
        Opr::DrawLine(x, y - 4, x, y + 4),
        Opr::ChangeColor(Color::Red),
        Opr::SetWidth(1),
        Opr::DrawLine(x, y, x, y),
    ];
    if rotate_text {
        let tm = vec![
            Object::Integer(0),
            Object::Integer(1),
            Object::Integer(-1),
            Object::Integer(0),
            Object::Integer(x as i64 + 10),
            Object::Integer(y as i64 + 2),
        ];
        let tf = vec![
            Object::Name(b"F1".to_vec()),
            Object::Integer(10),
        ];
        let tj = vec![
            Object::string_literal(text),
        ];
        oprs.append(&mut vec![
            Opr::Raw("BT".to_string(), vec![]),
            Opr::Raw("Tf".to_string(), tf),
            Opr::Raw("Tm".to_string(), tm),
            Opr::Raw("Tj".to_string(), tj),
            Opr::Raw("ET".to_string(), vec![]),
        ])
    } else {
        oprs.push(Opr::WriteLine(x + 2, y + 4, 10, text));
    }
    oprs
}

fn sub(width: u32, height: u32, x: u32, y: u32) -> Vec<Opr> {
    let mut oprs = vec![Opr::Raw("w".to_string(), vec![0.3.into()])];

    oprs.push(Opr::ChangeColor(Color::Grey));
    for i in 1..10 {
        oprs.push(Opr::DrawLine((x-1)*20+i*4, (y-1)*20, (x-1)*20+i*4, (y+1)*20));
    }
    for i in 1..10 {
        oprs.push(Opr::DrawLine((x-1)*20, (y-1)*20+i*4, (x+1)*20, (y-1)*20+i*4));
    }
    oprs.push(Opr::SetWidth(1));

    oprs.append(&mut grid_from_to(x - 1, x + 1, height, true));
    oprs.append(&mut grid_from_to(y - 1, y + 1, width, false));

    oprs
}

fn grid_from_to(from: u32, to: u32, length: u32, vertical: bool) -> Vec<Opr> {
    let mut oprs = vec![];
    for i in from..=to {
        let (red, green, blue) = COLOURS[(i as usize %COLOURS.len()) as usize];
        oprs.push(Opr::ChangeRgb(red, green, blue));
        let text = if i < 10 {format!("0{}", i)} else {format!("{}", i)};
        if vertical {
            oprs.push(Opr::DrawLine(i*20, 0, i*20, length + 1));
            oprs.push(Opr::WriteLine(i*20 + 2, 5, 10, text));
        } else {
            oprs.push(Opr::DrawLine(0, i*20, length + 1, i*20));
            oprs.push(Opr::WriteLine(5, i*20 + 2, 10, text));
        }
    }
    oprs
}

fn full(width: u32, height: u32) -> Vec<Opr> {
    let mut oprs = vec![Opr::SetWidth(1)];

    oprs.append(&mut grid_from_to(1, width/20, height, true));
    oprs.append(&mut grid_from_to(1, height/20, width, false));

    oprs
}

const fn to_rgb(hex: u32) -> (u8, u8, u8) {
    ((hex >> 16) as u8, ((hex >> 8) & 255) as u8, (hex & 255) as u8)
}
