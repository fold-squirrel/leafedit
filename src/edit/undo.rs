use lopdf::{Error as LopdfError, Document, Object};

pub fn undo_last(file: String, save_as: String) -> Result<(), LopdfError> {
    let mut doc = Document::load(file)?;
    remove_last(doc.get_object_mut((6, 0))?)?;
    doc.save(save_as)?;
    println!("undone");
    Ok(())
}

fn remove_last(contents: &mut Object) -> Result<(), LopdfError> {
    let contents = contents.as_stream_mut()?;

    if !contents.content.is_empty() {
        let mut operations = contents.decode_content()?;
        let mut i = 0;
        loop {
            if let Some(last) = operations.operations.pop() {
                if last.operator == "Q" {i+=1};
                if last.operator == "q" {i-=1};
            } else {
                break;
            }
            if i == 0 {break;}
        }
        contents.set_plain_content(operations.encode()?);
    }

    Ok(())
}
