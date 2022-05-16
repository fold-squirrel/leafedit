use lopdf::{Error as LopdfError, Document, Object};
use lopdf::content::{Operation, Content};
use crate::Opr;
use super::editor::*;

pub fn edits(file: &str, save_as: &str, opr_vec: Vec<Opr>,) -> Result<(), LopdfError> {
    let mut doc = Document::load(file)?;

    edits_on_doc(&mut doc, opr_vec)?;

    doc.save(save_as)?;
    println!("edited");

    Ok(())
}

pub fn edits_on_doc(doc: &mut Document, oprs: Vec<Opr>,) -> Result<(), LopdfError> {
    apply_opr(doc.get_object_mut((6, 0))?, oprs)?;
    Ok(())
}

fn apply_opr(content_obj: &mut Object, opr_vec: Vec<Opr>) -> Result<(), LopdfError> {
    let content = content_obj.as_stream_mut()?;

    let mut operations: Vec<Operation> = vec![ 
        Operation::new("q", vec![]),
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
