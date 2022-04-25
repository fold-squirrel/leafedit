use lopdf::{Error as LopdfError, Document, Object};
use lopdf::content::{Operation, Content};
use crate::Opr;
use crate::PageSize as Size;
use super::editor::*;

pub fn edits(file: &str, save_as: &str, opr_vec: Vec<Opr>, size: Size) -> Result<(), LopdfError> {
    let mut doc = Document::load(file)?;
    let (_, height) = get_page_size(doc.get_object((4, 0))?)?;
    let scale = Scale::new(height, size);

    apply_opr(doc.get_object_mut((6, 0))?, opr_vec, scale)?;

    doc.save(save_as)?;
    println!("edited");

    Ok(())
}

struct Scale {
    pub scale: f64,
}

impl Scale {
    fn new(height: f64, page: Size) -> Scale {
        let scale = match page {
            Size::Word => height/792_f64,
            Size::A4 => height/842_f64,
        };
        Scale {scale}
    }

    fn get(&self) -> Vec<Object> {
        vec![
              Object::Real(self.scale),
              Object::Integer(0),
              Object::Integer(0),
              Object::Real(self.scale),
              Object::Integer(0),
              Object::Integer(0),
        ]
    }
}

fn apply_opr(content_obj: &mut Object, opr_vec: Vec<Opr>, sc: Scale) -> Result<(), LopdfError> {
    let content = content_obj.as_stream_mut()?;

    let mut operations: Vec<Operation> = vec![ 
        Operation::new("q", vec![]),
        Operation::new("cm", sc.get()),
        Operation::new("rg", vec![0.into(),0.into(),0.into()]),
        Operation::new("w", vec![1.into()]),
        Operation::new("J", vec![1.into()]),
        Operation::new("j", vec![1.into()]),
    ];

    for opr in opr_vec {
        let mut next_operation = match opr {
            Opr::WriteLine(px , py, size, text ) => {
                writer::write_text(px, py, size, text)
            }

            Opr::ChangeColor(color) => {
                painter::change_color(color)
            }

            Opr::ChangeRgb(red, green, blue) => {
                painter::change_rgb(red, green, blue)
            }

            Opr::SetWidth(width) => {
                drawer::set_width(width)
            }

            Opr::DrawLine(x1, y1, x2, y2) => {
                drawer::draw_line(x1, y1, x2, y2)
            }

            Opr::Raw(raw_operator, raw_operants) => {
                vec![Operation::new(&raw_operator, raw_operants)]
            }
        };
        operations.append(&mut next_operation);
    }

    operations.push(Operation::new("Q", vec![]));

    let contents = clean_stream(operations);
    let mut stream = content.content.to_owned();
    stream.append(&mut contents.encode()?);
    content.set_plain_content(stream.to_vec());

    Ok(())
}

fn clean_stream(operations: Vec<Operation>) -> Content {
    // un-implemented yet
    Content { operations }
}

fn get_page_size(page: &Object) -> Result<(f64, f64), LopdfError> {
    let page_box_array = page.as_dict()?.get(b"MediaBox")?.as_array()?;
    let width = &page_box_array[2];
    let height = &page_box_array[3];
    let width = to_number::<f64>(width)?;
    let height = to_number::<f64>(height)?;
    Ok((width, height))
}

fn to_number<T: From<i32> + From<f32>>(num: &Object) -> Result<T, LopdfError> {
    match num {
        Object::Real(width) => Ok(T::from(*width as f32)),
        Object::Integer(width) => Ok(T::from(*width as i32)),
        _ => Err(LopdfError::Type),
    }
}

