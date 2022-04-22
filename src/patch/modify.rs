use chrono::Local;
use lopdf::Stream;
use lopdf::{Document, Error as LopdfError, Dictionary, StringFormat, Object, ObjectId, dictionary};
use crate::patch::analyze::ResourcesPlace;
use std::mem;

use crate::patch::map::Mapper;

use crate::patch::analyze::{modify_map, Marks};

use crate::patch::constants::CREATOR;
use crate::patch::constants::PRODUCER;

pub fn modify_main(input: &str, page: u32) -> Result<Document, LopdfError> {

    // load doc and create object mapper to move objects around
    let input_doc = Document::load(input)?;
    let mut map = Mapper::new(&input_doc);
    let mark = modify_map(&input_doc, &mut map, page)?;

    let mut patched_doc = create_doc(input_doc, map);

    apply_marks(&mut patched_doc, &mark);

    Ok(patched_doc)
}

fn create_doc(mut input_doc: Document,mut mapper: Mapper) -> Document {

    let stream = collect_streams(&mut input_doc, mapper.get_stream_ids());
    let mut patched_doc = remap_objects(input_doc, &mut mapper);
    if let Ok(root) = patched_doc.get_object_mut((2, 0)) {
        purge_root(root, vec![ b"Type".to_vec(), b"Pages".to_vec(), b"Lang".to_vec() ]).ok();
    }

    remap_refrences(&mut patched_doc, &mapper);
    let tralier = &mut patched_doc.trailer;
    tralier.set(b"Root".to_vec(), (2, 0));
    tralier.set(b"Info".to_vec(), (1, 0));

    let info_obj = create_info_obj(CREATOR, PRODUCER);
    patched_doc.objects.insert((1, 0), info_obj);

    patched_doc.objects.insert((5, 0), stream);

    patched_doc
}

fn purge_root(root_obj: &mut Object, preserve: Vec<Vec<u8>>) -> Result<(), LopdfError> {
    for (key, v) in root_obj.as_dict_mut()?.iter_mut() {
        if !preserve.contains(key) {
            *v = Object::Reference(NULL_ID);
        }
    }
    Ok(())
}

fn apply_marks(doc: &mut Document, marks: &Marks) {

    let resources_id = match marks.resources_object_dereference {
        ResourcesPlace::RootPage => (3_u32, 0_u16),
        ResourcesPlace::CurrentPage => (4_u32, 0_u16),
        ResourcesPlace::LastObject => (doc.max_id, 0_u16),
    };

    if marks.from_page_extract_resources{
        let page_1_obj_result = doc.get_object_mut(resources_id);
        if let Ok(page_1_obj) = page_1_obj_result {
            match modify_page_obj(page_1_obj) {
                Some(pages_obj) => {doc.objects.insert((7, 0), pages_obj);},
                None => {doc.objects.insert((7, 0), Object::Dictionary(dictionary!{}));},
            }
        }
    }

    let page_1_obj_result = doc.get_object_mut((4, 0));
    if let Ok(page_1_obj) = page_1_obj_result {
        modify_page_obj(page_1_obj);
    }

    if let Ok(pages_obj) = doc.get_object_mut((3, 0)) {
        if let Ok(pages_dict) = pages_obj.as_dict_mut() {
            pages_dict.remove(b"Resources");
        }
    }

    doc.objects.entry((8, 0)).or_insert_with(|| Object::Dictionary(dictionary!{}));

    if marks.from_resources_extract_fonts {
        let resources_result = doc.get_object_mut((7, 0));
        if let Ok(resources_obj) = resources_result {
            match get_fonts(resources_obj) {
                Some(fonts) => {doc.objects.insert((8, 0), fonts);}
                None => {}
            }
        }
    }

    doc.objects.entry((9, 0)).or_insert_with(|| Object::Dictionary(dictionary!{}));

    if marks.from_resources_extract_extgstate {
        let resources_result = doc.get_object_mut((7, 0));
        if let Ok(resources_obj) = resources_result {
            match get_extgstate(resources_obj) {
                Some(extgstate) => {doc.objects.insert((9, 0), extgstate);}
                None => {}
            }
        }
    }

    if let Ok(resources_object) = doc.get_object_mut((7, 0)) {
        modify_obj(
            resources_object,
            (b"Font", Object::Reference((8, 0))),
            "couldn't modify Resources to add font ref");
        modify_obj(
            resources_object,
            (b"ExtGState", Object::Reference((9, 0))),
            "couldn't modify Resources to add ExtGState ref");
    }

    if let Ok(pages_obj) = doc.get_object_mut((3, 0)) {
        modify_obj(pages_obj,
                   (b"Kids",
                    Object::Array(vec![Object::Reference((4, 0))])),
                   "couldn't not modify pages object");
        modify_obj(pages_obj,
                   (b"Count",
                    Object::Integer(1)),
                   "couldn't not modify pages object");
    }

    if let Ok(page_obj) = doc.get_object_mut((4, 0)) {
        modify_obj(page_obj,
                   (b"Contents",
                    Object::Array(vec![Object::Reference((5, 0)),Object::Reference((6, 0))])),
                   "couldn't not modify page object");
    }
}

fn modify_page_obj(page_obj: &mut Object) -> Option<Object> {
    modify_obj(page_obj,
               (b"Parent", Object::Reference((3, 0))),
               "couldn't not modify pages object");
    modify_obj(page_obj,
               (b"Resources", Object::Reference((7, 0))),
               "couldn't not modify pages object")
}


fn traverse<A: Fn(&mut Object)>(doc: &mut Document, action: A) {
    fn traverse_array<A: Fn(&mut Object)>(array: &mut [Object], action: &A) {
        for item in array.iter_mut() {
            traverse_object(item, action);
        }
    }
    fn traverse_dictionary<A: Fn(&mut Object)>(dict: &mut Dictionary, action: &A) {
        for (_, v) in dict.iter_mut() {
            traverse_object(v, action);
        }
    }
    fn traverse_object<A: Fn(&mut Object)>(object: &mut Object, action: &A) {
        action(object);
        match *object {
            Object::Array(ref mut array) => traverse_array(array, action),
            Object::Dictionary(ref mut dict) => traverse_dictionary(dict, action),
            Object::Stream(ref mut stream) => traverse_dictionary(&mut stream.dict, action),
            Object::Reference(_id) => { }
            _ => {}
        };
    }
    for (_, v) in doc.objects.iter_mut() {
        traverse_object(v, &action)
    }
}

fn remap_refrences(doc: &mut Document, mapper: &Mapper) {
    traverse(doc, |object| {
        if let Object::Reference(ref mut id) = *object {
            match mapper.index_of(*id) {
                Some(u) => *id = (u as u32, 0),
                None => *id = (0, 0),
            }
        }
    });

    clean_doc(doc);
}

const NULL_ID: ObjectId = (0, 0);

fn clean_doc(doc: &mut Document) {
    traverse(doc, |object| {
        if let Object::Array(ref mut array) = *object {
            let mut i = 0;
            for _ in 0..array.len() {
                if let Object::Reference(id) = array[i] {if id == NULL_ID {
                    array.remove(i);
                } else {
                    i+=1;
                }}
            }
        }
    });

    traverse(doc, |object| {
        if let Object::Dictionary(ref mut dict) = *object {
           let mut keys = vec![];
           for (k, v) in dict.iter() {
               match v {
                   Object::Reference(id) => if *id == NULL_ID {keys.push(k.clone())}
                   Object::Array(array) => if array.is_empty() {keys.push(k.clone())}
                   _ => {}
               }
           }
           for k in keys {
               dict.remove(&k);
           }
        }
    })
}

fn remap_objects(mut doc: Document, mapper: &mut Mapper) -> Document {
    let mut patched_doc = Document::new();

    for _ in 1..=mapper.len() {
        patched_doc.new_object_id();
    }

    for (i, id_option) in mapper.iter().enumerate() {
        if let Some(id) = id_option {
            if let Ok(obj) = doc.get_object_mut(*id) {
                patched_doc.objects.insert((i as u32, 0), mem::replace(obj, Object::Null));
            }
        }
    };

    patched_doc
}


fn collect_streams(doc: &mut Document, stream_id: &Vec<ObjectId>) -> Object {
    let mut streams: Vec<u8> = vec![];

    for id in stream_id  {
        if let Ok(Object::Stream(stream_obj)) = doc.get_object_mut(*id) {
            stream_obj.decompress();
            streams.push(0xA);
            streams.append(&mut stream_obj.content)
        }
    }

    Object::Stream(Stream {
        dict: dictionary!{}, content: streams, allows_compression: true, start_position: None
    })
}

fn get_fonts(resources_obj: &mut Object) -> Option<Object> {
    modify_obj(resources_obj,
               (b"Font", Object::Reference((8, 0))),
               "couldn't not modify Resources object")
}

fn get_extgstate(resources_obj: &mut Object) -> Option<Object> {
    modify_obj(resources_obj,
               (b"ExtGState", Object::Reference((9, 0))),
               "couldn't not modify Resources object")
}

fn modify_obj(obj: &mut Object, key_value: (&[u8], Object), error: &str) -> Option<Object> {
    let (key, mut value) = key_value;
    let previous_obj: Option<Object>;

    match obj.as_dict_mut() {
        Ok(mut_dict) => {
            match mut_dict.get_mut(key) {
                Ok(old) => {mem::swap(&mut value, old); previous_obj = Some(value)}
                Err(_) => {mut_dict.set(key, value); previous_obj = None}
            }
        }
        Err(_) => panic!("{}", error),
    }
    previous_obj
}

fn create_info_obj(creater: &str, producer: &str) -> Object {
    Object::Dictionary(dictionary! {
        "Creator" => Object::String(creater.as_bytes().to_vec(), StringFormat::Literal),
        "Producer" => Object::string_literal(producer),
        "CreationDate" => Local::now()
    })
}

