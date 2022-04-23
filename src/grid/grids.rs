use crate::commadline::{PageSize as Page, GridType as Grid, Opr, Color};
use lopdf::Error as LopdfError;

const COLOURS: [(u8, u8, u8); 7] = [
    to_rgb(0x5F4149),
    to_rgb(0xEB3247),
    to_rgb(0xBC4349),
    to_rgb(0xF35645),
    to_rgb(0xF6A73B),
    to_rgb(0xFAD32F),
    to_rgb(0xa6896b),
];

pub fn generate(size: Page, grid: Grid, file: &str, save_as: &str) -> Result<(), LopdfError> {

    let (width, height) = get_page_dimentions(&size);
    let oprs = match grid {
        Grid::Full => full(width, height),
        Grid::Sub(x, y) => sub(width, height, x, y),
        Grid::Mark(x, y) => mark(width, height, x, y),
    };

    crate::edit::apply::edits(file, save_as, oprs, size)?;
    println!("gridded");
    Ok(())
}

fn mark(width: u32, height: u32, x: u32, y: u32) -> Vec<Opr> {
    vec![
        Opr::ChangeColor(Color::Grey),
        Opr::Raw("w".to_string(), vec![0.5.into()]),
        Opr::DrawLine(0, y, width, y),
        Opr::DrawLine(x, 0, x, height),
        Opr::ChangeColor(Color::Red),
        Opr::SetWidth(2),
        Opr::DrawLine(x, y, x, y),
        Opr::WriteLine(x + 2, y + 4, 10, format!("({}, {})", x, y)),
    ]
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
    for i in (x-1)..=(x+1) {
        let (red, green, blue) = COLOURS[(i%7) as usize];
        oprs.push(Opr::ChangeRgb(red, green, blue));
        oprs.push(Opr::DrawLine(i*20, 0, i*20, height));
        oprs.push(Opr::WriteLine(i*20 + 2, height - 20, 10, i.to_string()));
    };
    for i in (y-1)..=(y+1) {
        let (red, green, blue) = COLOURS[(i%7) as usize];
        oprs.push(Opr::ChangeRgb(red, green, blue));
        oprs.push(Opr::DrawLine(0, i*20, width, i*20));
        let text = if i < 10 {format!("  {}", i)} else {format!("{}", i)};
        oprs.push(Opr::WriteLine(5, i*20 + 2, 10, text));
    };
    oprs
}

fn full(width: u32, height: u32) -> Vec<Opr> {
    let mut oprs = vec![Opr::SetWidth(2)];

    for i in 1..=width/20 {
        let (red, green, blue) = COLOURS[(i%7) as usize];
        oprs.push(Opr::ChangeRgb(red, green, blue));
        oprs.push(Opr::DrawLine(i*20, 0, i*20, height));
        oprs.push(Opr::WriteLine(i*20 + 2, height - 20, 10, i.to_string()));
    }
    for i in 1..height/20 {
        let (red, green, blue) = COLOURS[(i%7) as usize];
        oprs.push(Opr::ChangeRgb(red, green, blue));
        oprs.push(Opr::DrawLine(0, i*20, width, i*20));
        let text = if i < 10 {format!("  {}", i)} else {format!("{}", i)};
        oprs.push(Opr::WriteLine(5, i*20 + 2, 10, text));
    }

    oprs
}

const fn to_rgb(hex: u32) -> (u8, u8, u8) {
    ((hex >> 16) as u8, ((hex >> 8) & 255) as u8, (hex & 255) as u8)
}

fn get_page_dimentions(page: &Page) -> (u32, u32) {
    match page {
        Page::A4 => (595, 842),
        Page::Word => (612, 792),
    }
}
