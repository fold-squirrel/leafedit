use crate::commadline::Information;
use lopdf::{Document, Object, Error as LopdfError};

pub fn from_file(about: Information) -> Result<(), LopdfError> {
    match about {
        Information::PageSize { page, file } => print_page_size_from_file(page, file),
    }
}

fn print_page_size_from_file(page_number: u32, file: String) -> Result<(), LopdfError> {
    let doc = Document::load(file)?;

    let page_id = *doc.get_pages().get(&page_number).expect("page doesn't exist");

    let page_obj = doc.get_object(page_id)?;

    if let Ok((width, height)) = get_page_width_height(page_obj) {
        println!("{:.2}, {:.2}", width, height);
    }

    Ok(())
}

pub fn from_doc_get_page_width_height(doc: &Document, page_id: (u32, u16)) -> (f64, f64) {
    let page_obj = doc.get_object(page_id).expect("page doesn't exist");
    get_page_width_height(page_obj).expect("not a valid page obj")
}

fn get_page_width_height(page_obj: &Object) -> Result<(f64, f64), LopdfError> {
    let page_box_array = page_obj.as_dict()?.get(b"MediaBox")?.as_array()?;
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

