use chrono::Local;
use lopdf::{Error as LopdfError, Document, Object, StringFormat, dictionary};

use crate::CREATOR;
use crate::PRODUCER;

pub fn merge_patched_docs(files: Vec<String>, save_as: String) -> Result<(), LopdfError> {

    let mut fonts = LazyFontId::new();

    let mut final_doc = Document::with_version("1.7");
    let info = final_doc.add_object(Object::Dictionary(dictionary! {
        "Creator" => Object::String(CREATOR.as_bytes().to_vec(), StringFormat::Literal),
        "Producer" => Object::string_literal(PRODUCER),
        "CreationDate" => Local::now()
    }));

    let root = final_doc.add_object(Object::Dictionary(dictionary!{
        "Type" => "Catalog",
        "Pages" => (3, 0),
    }));

    let pages = final_doc.new_object_id();

    let mut kids: Vec<Object> = vec![];
    let mut count = 0;

    for file in files {
        let mut doc_to_merge = Document::load(file)?;
        doc_to_merge.renumber_objects_with(final_doc.max_id + 1);

        let page_id = doc_to_merge.page_iter().next().ok_or(LopdfError::PageNumberNotFound(1))?;

        let mut page_obj = doc_to_merge
            .objects
            .remove(&page_id)
            .ok_or(LopdfError::ObjectNotFound)?;

        let resourses = doc_to_merge.get_object_mut(
            page_obj.as_dict()?.get(b"Resources")?.as_reference()?)?;

        let fonts_id = resourses.as_dict()?.get(b"Font")?.as_reference()?;
        let fonts_obj = doc_to_merge.get_object_mut(fonts_id)?;

        let font_dict = fonts_obj.as_dict_mut()?;
        let f1 = font_dict.remove(b"F1").ok_or(LopdfError::DictKey)?.as_reference()?;
        let f2 = font_dict.remove(b"F2").ok_or(LopdfError::DictKey)?.as_reference()?;

        let mut fonts_vec = vec![];
        let mut font_ids_vec = vec![];
        for (key, _) in font_dict.iter() {
            font_ids_vec.push(key.to_owned());
        }
        for key in font_ids_vec {
            let id = font_dict.remove(&key).unwrap();
            fonts_vec.push((key, id));
        }

        font_dict.set(b"F1".to_vec(), Object::Reference(fonts.get_f1(f1)));
        font_dict.set(b"F2".to_vec(), Object::Reference(fonts.get_f2(f2)));

        for(key, value) in fonts_vec {
            font_dict.set(key, value);
        }


        page_obj.as_dict_mut()?.set(b"Parent".to_vec(), Object::Reference(pages));
        let page_id = final_doc.add_object(page_obj);

        kids.push(Object::Reference(page_id));
        count+=1;

        for doc_object in doc_to_merge.objects.into_values() {
            final_doc.add_object(doc_object);
        }
    }

    final_doc.objects.insert(pages, Object::Dictionary(dictionary!{
        "Type" => "Pages",
        "Kids" => Object::Array(kids),
        "Count" => Object::Integer(count as i64),
    }));

    final_doc.trailer.set(b"Info".to_vec(), info);
    final_doc.trailer.set(b"Root".to_vec(), root);

    final_doc.prune_objects();
    final_doc.renumber_objects();

    final_doc.save(save_as)?;
    println!("merged");

    Ok(())
}

struct LazyFontId {
    f1: Option<(u32, u16)>,
    f2: Option<(u32, u16)>,
}

impl LazyFontId {
    fn new() -> LazyFontId {
        LazyFontId { f1: None, f2: None }
    }

    fn get_f1 (&mut self, f1: (u32, u16)) -> (u32, u16) {
        if self.f1.is_none() {
            self.f1 = Some(f1);
            self.f1.unwrap()
        } else {
            self.f1.unwrap()
        }
    }

    fn get_f2 (&mut self, f2: (u32, u16)) -> (u32, u16) {
        if self.f2.is_none() {
            self.f2 = Some(f2);
            self.f2.unwrap()
        } else {
            self.f2.unwrap()
        }
    }
}
