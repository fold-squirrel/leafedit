use std::char;
use lopdf::{content::Operation, Object, StringFormat};

#[derive(Debug)]
enum FontChar {
    F1(char),
    F2([u8; 2]),
}

#[derive(Debug)]
enum Font {
    Null,
    F1,
    F2,
}

fn lookup(ch: char) -> [u8; 2] {
    match ch {
        '\u{2713}' => [0, 1],
        _ => [0, 0]
    }
}

fn parse_text(text: String) -> Vec<FontChar> {
    let mut font_chars = vec![];

    for ch in text.chars(){
        if ch.is_ascii() {
            font_chars.push(FontChar::F1(ch));
        } else {
            font_chars.push(FontChar::F2(lookup(ch)));
        }
    }

    font_chars
}

fn flaten(font_ch_vec: Vec<FontChar>) -> Vec<(Object, Object)> {
    let mut pdf_string: Vec<(Object, Object)> = vec![];
    let mut current_font = Font::Null;
    let mut ascii_str = "".to_owned();
    let mut unicode: Vec<u8> = vec![];

    let f1 = Object::Name(b"F1".to_vec());
    let f2 = Object::Name(b"F2".to_vec());

    for font_char in font_ch_vec {
        match font_char {
            FontChar::F1(ch) => match current_font {
                Font::F1 => ascii_str.push(ch),
                Font::F2 => {
                    let f2 = f2.clone();
                    pdf_string.push((f2, Object::String(unicode, StringFormat::Hexadecimal)));
                    unicode = vec![];
                    ascii_str = format!("{ch}");
                    current_font = Font::F1;
                }
                Font::Null => {current_font = Font::F1; ascii_str.push(ch)}
            }
            FontChar::F2(ch) => match current_font {
                Font::F2 => unicode.append(&mut ch.to_vec()),
                Font::F1 => {
                    let f1 = f1.clone();
                    pdf_string.push((f1, Object::string_literal(ascii_str)));
                    ascii_str = "".to_owned();
                    unicode.append(&mut ch.to_vec());
                    current_font = Font::F2;
                }
                Font::Null => {current_font = Font::F2; unicode.append(&mut ch.to_vec())}
            }
        }
    };

    match current_font {
        Font::F1 => pdf_string.push((f1, Object::string_literal(ascii_str))),
        Font::F2 => pdf_string.push((f2, Object::String(unicode, StringFormat::Hexadecimal))),
        Font::Null => ()
    };

    pdf_string
}

pub fn write_text(px: u32, py: u32, size: u32, text: String) -> Vec<Operation> {
    let parsed = parse_text(text);
    let text_objects = flaten(parsed);

    let mut operations: Vec<Operation> = vec![
        Operation::new("BT", vec![]),
        Operation::new("Td", vec![px.into(), py.into()])
    ];

    for obj in text_objects {
        operations.push(Operation::new("Tf", vec![obj.0, size.into()]));
        operations.push(Operation::new("Tj", vec![obj.1]));
    }

    operations.push(Operation::new("ET", vec![]));
    operations
}
