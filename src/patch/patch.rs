use lopdf::Stream;
use lopdf::content::Operation;
use lopdf::{Error as LopdfError, Dictionary, Object, ObjectId, dictionary};
use lopdf::content::Content;
use lopdf::Document;

use crate::patch::matrix::Matrix;
use crate::patch::modify::modify_main;
use crate::patch::constants::*;

pub fn patch(file: &str, save_as: &str, page: u32) -> Result<(), LopdfError> {
    let mut final_doc = modify_main(file, page)?;

    let old_fonts = change_font_names(final_doc.get_object_mut((8, 0))?.as_dict_mut()?);
    let action = |mut operation: Operation| -> Operation {
        if operation.operator == "Tf" {
            let i = old_fonts.iter().position(
                |x| x.as_bytes() == operation.operands[0].as_name().unwrap()
                ).unwrap();
            operation.operands[0] = Object::Name(format!("F{}", i + 3).as_bytes().to_vec());
        }
        operation
    };

    if let Ok(Object::Stream(stream)) = final_doc.get_object_mut((5, 0)) {
        let content_matrix = StreamParser::extract(stream, vec!["cm", "Q", "q"]);
        let cm = reverse_cm(content_matrix);
        let cm_encoded = Content { operations: vec![Operation::new("cm", cm.to_vec())] };
        stream.content.append(&mut cm_encoded.encode()?);

        StreamParser::modify(stream, action);
    }

    let xobject_streams_ids = StreamParser::find_all_form_streams(&final_doc);
    for id in xobject_streams_ids {
        println!("id: {:?}", id);
        if let Ok(Object::Stream(stream)) = final_doc.get_object_mut(id) {
            StreamParser::modify(stream, action);
        }
    }

    final_doc.prune_objects();

    add_strean(&mut final_doc);

    embed_font_files(&mut final_doc);

    final_doc.save(save_as)?;

    Ok(())
}

fn add_strean(final_doc: &mut Document) {
    final_doc.objects.insert((6, 0), Object::Stream(Stream {
        dict: dictionary!{"Length" => 0},
        content: b"".to_vec(),
        allows_compression: true,
        start_position: None 
    }));
}

fn embed_font_files(doc: &mut Document) {
    doc.objects.insert((9, 0), Object::Dictionary(f_9()));
    doc.objects.insert((10, 0), Object::Dictionary(f_10()));
    doc.objects.insert((11, 0), Object::Stream(f_11()));
    doc.objects.insert((12, 0), Object::Dictionary(f_12()));
    doc.objects.insert((13, 0), Object::Stream(f_13()));
    doc.objects.insert((14, 0), Object::Stream(f_14()));
    doc.objects.insert((15, 0), Object::Dictionary(f_15()));
    doc.objects.insert((16, 0), Object::Dictionary(f_16()));
    doc.objects.insert((17, 0), Object::Stream(f_17()));
}

fn reverse_cm(content_matrix: Content) -> [Object; 6] {
    let mut cm_operations = vec![Matrix::defualt()];
    for operation in content_matrix.operations {
        match operation.operator.as_str() {
            "cm" => if let Some(mut matrix) = Matrix::new(operation.operands) {
                if let Some(old_matrix) = cm_operations.pop() {
                    matrix.multiply(old_matrix);
                    cm_operations.push(matrix);
                };
            }
            "Q" => {
                cm_operations.pop();
            },
            "q" => {
                if let Some(last) = cm_operations.pop() {
                    cm_operations.push(last);
                    cm_operations.push(last);
                }
            }
            _ => {},
        }
    }
    Matrix::inverse(&mut cm_operations)
}

fn change_font_names(dict: &mut Dictionary) -> Vec<String> {
    let mut keys = vec![];
    for (k, _) in dict.iter() {
        keys.push(String::from_utf8(k.to_owned()).unwrap());
    };
    let mut values = vec![];
    for k in keys.iter() {
        values.push(dict.remove(k.as_bytes()).unwrap());
    }
    dict.set(b"F1".to_vec(), Object::Reference((9, 0)));
    dict.set(b"F2".to_vec(), Object::Reference((10, 0)));
    for i in 0..values.len() {
        dict.set(format!("F{}", i + 3).as_bytes(), values.remove(0));
    }
    keys
}

struct StreamParser {}

impl StreamParser {
    fn extract(stream: &mut Stream, keys: Vec<&str>) -> Content {
        let mut operations = vec![];
        if let Ok(content) = stream.decode_content() {
            for operation in content.operations {
                if keys.contains(&operation.operator.as_str()) {
                    operations.push(operation);
                }
            }
        }
        Content { operations }
    }

    fn modify<A: Fn(Operation) -> Operation>(stream: &mut Stream, action: A) {
        let mut operations = vec![];
        if let Ok(content) = stream.decode_content() {
            for operation in content.operations {
                operations.push(action(operation));
            }
        }
        let contents = Content {operations};
        if let Ok(encoded_stream) =  contents.encode() {
            stream.dict.set(b"Length".to_vec(), Object::Integer(encoded_stream.len() as i64));
            stream.content = encoded_stream;
        }
    }

    fn find_all_form_streams(doc: &Document) -> Vec<ObjectId> {
        let mut ids = vec![];
        for (k, v) in doc.objects.iter() {
            if let Object::Stream(stream_obj) = v {
                if let Ok(Object::Name(value)) = stream_obj.dict.get(b"Subtype") {
                    if value == b"Form" {
                        if let Ok(Object::Reference(id)) = stream_obj.dict.get(b"Resources") {
                            if *id == (7, 0) {
                                ids.push(*k);
                            }
                        }
                    }
                }
            }
        }
        ids
    }
}

