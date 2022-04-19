use lopdf::{Document, Error as LopdfError, Dictionary, Object, ObjectId};
use crate::patch::constants::Error;
use crate::patch::map::Mapper;

#[derive(Debug)]
pub enum ResourcesPlace {
    RootPage,
    CurrentPage,
    LastObject,
}

#[derive(Debug)]
pub struct Marks {
    pub from_root_extract_pages: bool,
    pub from_page_extract_resources: bool,
    pub from_resources_extract_fonts: bool,
    pub resources_object_dereference: ResourcesPlace,
}

impl Marks {
    fn new() -> Marks {
        Marks {
            from_root_extract_pages: false,
            from_page_extract_resources: false,
            from_resources_extract_fonts: false,
            resources_object_dereference: ResourcesPlace::CurrentPage,
        }
    }
}

pub fn modify_map(doc: &Document, obj_map: &mut Mapper, x: u32) -> Result<Marks, LopdfError> {
    // Mark Objects for modification before inserting in patched doc
    let mut mark = Marks::new();

    // free id (1, n) to insert info object at that address, and remove old info object
    match get_info_id(&doc.trailer) {
        Some(id) => { obj_map.remap(id, 1); },
        None => obj_map.free_id_at(1),
    }

    // get the id of the root object and move it to id (2, 0)
    let root_obj_id = get_root_id(&doc.trailer);
    obj_map.remap(root_obj_id, 2);

    // get pages object and depending on whether it's a dictionary or an id
    // if it's an id then map it to (3, 0)
    // if it's a dictionary then we mark it for extraction and free id (3, 0)
    let pages_obj = get_pages_obj(doc.get_object(root_obj_id)?);
    let pages_id = match_on_id(pages_obj, "pages_obj").ok_or(LopdfError::ObjectIdMismatch)?;
    obj_map.remap(pages_id, 3);


    // get page 1 object and depending on whether it's a dictionary or an id
    // if it's an id then map it to (4, 0)
    // if it's a dictionary then we mark it for extraction and free id (4, 0)
    let (page_obj, page_id) = match get_page_x_obj(doc, x) {
        Some(page_x) => page_x,
        None => return Err(LopdfError::PageNumberNotFound(x)),
    };
    obj_map.remap(page_id, 4);

    // collect all refrenced id object into a vector of of ObjectId
    let page_content_ids = get_page_contents(doc, page_obj);
    for id in &page_content_ids {
        obj_map.remove_id_at(id, true);
    }
    obj_map.free_id_at(5);
    obj_map.free_id_at(6);

    let containing_resources = match get_page_containing_resources(doc, page_id) {
        Some(id) => id,
        None => return Err(LopdfError::ObjectNotFound)
    };

    mark.resources_object_dereference = check_resources(pages_id, page_id, containing_resources);
    if let ResourcesPlace::LastObject = mark.resources_object_dereference {
        obj_map.map_to_last(containing_resources)
    };

    let page_resources = get_page_resources(doc.get_object(containing_resources)?);
    match page_resources {
        Some(resources_obj) => {
            let resources_obj = match match_on_id(resources_obj, "resources_obj") {
                Some(id) => {obj_map.remap(id, 7); doc.get_object(id)?}
                None => {
                    obj_map.free_id_at(7); 
                    mark.from_page_extract_resources = true;
                    resources_obj
                }
            };
            match get_fonts_obj(resources_obj) {
                Some(fonts_obj) => {
                    match match_on_id(fonts_obj, "fonts_obj") {
                        Some(id) => {obj_map.remap(id, 8);},
                        None => {mark.from_resources_extract_fonts = true; obj_map.free_id_at(8)}
                    }
                }
                None => obj_map.free_id_at(8)
            };
            obj_map.free_ids_starting_from(8, 9);
        },
        None => obj_map.free_ids_starting_from(6, 11)
    };

    Ok(mark)
}

fn check_resources(pages: ObjectId, page: ObjectId, resources: ObjectId) -> ResourcesPlace {
    if resources == page {
        ResourcesPlace::CurrentPage
    } else if resources == pages {
        ResourcesPlace::RootPage
    } else {
        ResourcesPlace::LastObject
    }
}

fn get_fonts_obj(resources_obj: &Object) -> Option<&Object> {
    let font_obj_result = get_obj(resources_obj,
            b"Font",
            "resources_obj is not a dictionary");

    match font_obj_result {
        Ok(font_obj) => Some(font_obj),
        Err(_) => None,
    }
}

fn get_page_containing_resources(doc: &Document, page_id: ObjectId) -> Option<ObjectId> {
    let current_page = doc.get_object(page_id).ok()?.as_dict().ok()?;
    if current_page.has(b"Resources") {
        Some(page_id)
    } else {
        let parent_id = current_page.get(b"Parent").ok()?.as_reference().ok()?;
        get_page_containing_resources(doc, parent_id)
    }
}

fn get_page_resources(page_obj: &Object) -> Option<&Object> {
    let resources_obj_result = get_obj(page_obj,
            b"Resources",
            "page_obj is not a dictionary");

    match resources_obj_result {
        Ok(resources_obj) => Some(resources_obj),
        Err(_) => None,
    }
}

fn get_page_contents(doc: &Document, page_obj: &Object) -> Vec<ObjectId> {
    let content_obj_result = get_obj(page_obj,
            b"Contents",
            "page_obj is not a dictionary");

    let content_obj = match content_obj_result {
        Ok(content_obj) => content_obj,
        Err(_) => &Object::Null
    };

    let content_obj = if let Object::Reference(content_ref) = content_obj {
        match doc.get_object(*content_ref) {
            Ok(refrenced) => {
                match refrenced {
                    Object::Array(_) => refrenced,
                    Object::Stream(_) => content_obj,
                    _ => panic!("not a valid content object"),
                }
            },
            _ => panic!(),
        }
    } else {
        content_obj
    };

    let mut obj_id_vec: Vec<(u32, u16)> = vec![];
    match content_obj {
        Object::Array(ids_vec) => {
            for id_obj in ids_vec {
                match id_obj {
                    Object::Reference(id) => obj_id_vec.push(*id),
                    _ => panic!("/Contents key contains an array with invalid object id"),
                }
            }
        },
        Object::Reference(single_obj_id) => obj_id_vec.push(*single_obj_id),
        Object::Null => {},
        _ =>  panic!("/Contents key contains an invalid object id"),
    }

    obj_id_vec
}

fn match_on_id(obj: &Object, obj_name: &str) -> Option<ObjectId> {
    match obj {
        Object::Reference(id) => Some(*id),
        Object::Dictionary(_dict) => None,
        _ => panic!("{} is neither a dictionary nor a refrence", obj_name),
    }
}

fn get_info_id(trailer: &Dictionary) -> Option<ObjectId> {
    match trailer.get(b"Info") {
        Ok(key) => {
            match_on_id(key, "trailer")
        },
        Err(_) => None,
    }
}

fn get_root_id(trailer: &Dictionary) -> ObjectId {
    let root_key = match trailer.get(b"Root") {
        Ok(key) => key,
        Err(_) => panic!("trailer object doesn't contain /Root key"),
    };

    match root_key.as_reference() {
        Ok(root_obj_id) => root_obj_id,
        Err(_) => panic!("/Root doesn't refrence the Root object"),
    }
}

fn get_pages_obj(root_obj: &Object) -> &Object {
    let pages_obj_result = get_obj(root_obj,
                                   b"Pages",
                                   "root_obj not a dictionary");

    match pages_obj_result {
        Ok(pages_obj) => pages_obj,
        Err(_) => panic!("Root dictionary doesn't contain /Pages key")
    }
}

fn get_page_x_obj(doc: &Document, x: u32) -> Option<(&Object, ObjectId)> {
    if let Some(id) = doc.page_iter().nth((x - 1) as usize) {
        match doc.get_object(id) {
            Ok(obj) => Some((obj, id)),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn get_obj<'a>(obj: &'a Object, key: &[u8], not_dict: &str) -> Result<&'a Object, Error> {
    let obj_dict = match obj.as_dict() {
        Ok(dict) => dict,
        Err(_) => panic!("{}", not_dict),
    };

    match obj_dict.get(key) {
        Ok(obj) => Ok(obj),
        Err(_) => Err(Error::KeyNotPresent),
    }
}

