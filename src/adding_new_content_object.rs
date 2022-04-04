use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, Stream, dictionary};

#[allow(dead_code)]
pub fn run_demo(file_name: &String, student_name: String) {
	let mut doc = Document::load(file_name).unwrap();
	doc.decompress();
	let pages = doc.get_pages();
	let first_page = pages.get(&1).unwrap();
	println!("{:?} page 1 id", first_page);
/*	[allow(unused_variables)]
	let stream = doc.get_page_contents(*first_page);
	println!("{:?}", stream[0]);
#[allow(unused_variables)]
	let operation = doc.get_and_decode_page_content(*first_page);
	println!("{}", String::from_utf8(operation.unwrap().encode().unwrap()).unwrap());
*/	let (_res, res_v) = doc.get_page_resources(*first_page);
	println!("res: {:?}", res_v.get(0).unwrap());
	println!("-------Creating a dictionary-------");
	let font_id = doc.add_object(dictionary! { 
		"Type" => "Font",
		"Subtype" => "Type1",
		"BaseFont" => "Helvetica",
	});
	let res_obj = doc.get_object(*res_v.get(0).unwrap()).unwrap();
	let mut new_dic = lopdf::Dictionary::new();
	for (key, value) in res_obj.as_dict().unwrap() {
		println!("key: {:#?}, value: {:#?}", String::from_utf8_lossy(key), value);
		if String::from_utf8_lossy(key).eq("Font") {
			let mut font_dic = lopdf::Dictionary::new();
			font_dic.set("F1", font_id);
			for (k, v) in value.as_dict().unwrap() {
				font_dic.set(k.clone(), v.clone());
			}
			new_dic.set("Font", font_dic);
		} else { new_dic.set(key.clone(), value.clone()); } }
	println!("\n\nnew_dic {:#?}", new_dic);
	doc.objects.insert(*res_v.get(0).unwrap(), Object::Dictionary(new_dic));
	println!("\n\nfinal res: {:?}", doc.get_object(*res_v.get(0).unwrap()).unwrap());
	println!("\n\n---------new content---------");
	let content = Content { 
		operations: 
			vec![ 
			Operation::new("BT", vec![]), 
			Operation::new("Tf", vec!["F1".into(), 14.into()]),
			Operation::new("Tm", vec![
						   (1 as i8).into(),
						   0.into(),
						   0.into(),
						   (-1 as i8).into(),
						   100.into(),
						   220.into(),
			]),
			Operation::new("Tj", vec![Object::string_literal(student_name)]),
			Operation::new("ET", vec![]), 
			],
	};
	let cevvv: Vec<Operation> = vec![Operation::new("mm", vec![])];
	print!("{:?}", cevvv);
	let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
	let mut new_dic = lopdf::Dictionary::new();
	for (key, value) in doc.get_object(*first_page).unwrap().as_dict().unwrap() {
		println!("key: {:#?}, value: {:#?}", String::from_utf8_lossy(key), value);
		if String::from_utf8_lossy(key).eq("Contents") {
			let mut con_vec = value.as_array().unwrap_or(&vec![value.clone()]).clone();
			con_vec.push(content_id.into());
			new_dic.set("Contents", con_vec.clone());
		} else { new_dic.set(key.clone(), value.clone()); }
	}
	println!("\n\n\n\ncon: {:#?}\n\n\n", new_dic);
	doc.objects.insert(*first_page, Object::Dictionary(new_dic));
	doc.save("saved.pdf").unwrap();
}

