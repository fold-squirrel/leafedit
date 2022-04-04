use std::collections::BTreeMap;
use lopdf::{Document, Object, Stream, ObjectId, Dictionary, dictionary};
pub fn patch(in_file_path: &String, out_file_path: &String) {
//	demos();
	let mut doc: Document = Document::default();
	doc.decompress();
	let mut first_page = ObjectId::default();
	let mut resources_id = ObjectId::default();
	let mut resources_obj: &mut Object = &mut Object::Null;
	let fonts_names_map: BTreeMap<String, String>;

	load_doc(&mut doc, in_file_path);
	get_first_page_checked(&mut doc, &mut first_page);
	get_page_resource(&mut doc, &first_page, &mut resources_obj, &mut resources_id);
	fonts_names_map = edit_font_dict(&mut doc, &mut resources_obj);

	edit_content_streams(&mut doc, &first_page, fonts_names_map);
	doc.objects.insert(resources_id, resources_obj.clone());
	add_content(&mut doc, &first_page);

	doc.compress();
	doc.save(out_file_path).unwrap();
}

fn add_content(doc: &mut Document, first_page: &ObjectId) {
	let content_id = doc.add_object(Stream::new(dictionary! {}, vec![]));
	let mut cont_array = doc.get_page_contents(*first_page);

	let page_obj = match doc.get_object_mut(*first_page) {
		Ok(obj) => obj,
		Err(err) => panic!("can't get page mut: {}", err),
	};
	let page_dict = match page_obj.as_dict_mut() {
		Ok(dict) => dict,
		Err(err) => panic!("can't get page dict mut: {}", err),
	};
	cont_array.push(content_id);

	let mut obj_vec: Vec<Object> = vec![];

	for id in &cont_array {
		obj_vec.push(Object::Reference(*id));
	}

	page_dict.set(b"Contents".to_vec(), Object::Array(obj_vec));

}

fn load_doc(doc: &mut Document, file_path: &String) {
	let doc_result = Document::load(file_path);

	match doc_result {
		Ok(mut loaded_doc) => {loaded_doc.decompress(); *doc = loaded_doc},
		Err(err) => panic!("error loading pdf: {}", err),
	};
}

fn get_first_page_checked(doc: &mut Document, first_page: &mut ObjectId) {
	let pages = doc.get_pages();

	match pages.len() {
		1 => *first_page = *pages.get(&1).unwrap(),
		_ => panic!("pdf contains more than one page"),
	};
}

fn get_page_resource(doc: &mut Document, page: &ObjectId, res: &mut Object, r_id: &mut ObjectId) {

	*res = match doc.get_or_create_resources(*page) {
		Ok(obj) => obj.clone(),
		Err(err) => panic!("error getting or creating resource: {}", err),
	};

	*r_id = match doc.get_page_resources(*page).1.get(0) {
		Some(resource_id) => resource_id.clone(),
		None => panic!("cant get page obj"),
	};
}

fn edit_font_dict(doc: &mut Document, resourse: &mut Object) -> BTreeMap<String, String> {
	let mut font_dict_cln = get_font_dict(doc, resourse);
	make_all_refs(doc, &mut font_dict_cln);

	let font_iter = font_dict_cln.iter();

	let mut font_map = BTreeMap::new();
	let mut new_key: String;
	let mut new_fonts = [ObjectId::default(); 2];
	let mut final_font_dict = Dictionary::new();
	for i in 1..=2 {
		new_key = format!("F{}", &i);
		new_fonts[i - 1] = doc.new_object_id();
		final_font_dict.set(new_key.clone(), new_fonts[i - 1]);
	}
	let mut i = 2;
	for (key, value) in font_iter {
		i+=1;

		new_key = format!("F{}", &i);
		final_font_dict.set(new_key.clone(), value.clone());
		font_map.insert(String::from_utf8(key.to_vec()).expect("Found invalid UTF-8"), new_key);
	};


	resourse.as_dict_mut().unwrap().set("Font", final_font_dict);
	
	add_fonts(doc, new_fonts[0], new_fonts[1]);

	font_map
}

fn add_fonts(doc: &mut Document, f1: ObjectId, f2: ObjectId) {
	let libre_font = include_bytes!("../../include/Liberation_sans_font_striped.bin");
	let libre_font_vec: Vec<u8> = libre_font.iter().cloned().collect();
	let embed_libre_font = doc.add_object(Stream::new(dictionary! {"Length1" => libre_font_vec.len() as i32}, libre_font_vec).with_compression(false));
	let font_descripter = doc.add_object(dictionary! {
		"Type" => "FontDescriptor",
		"FontName" => "BAAAAA+LiberationSans",
		"Flags" => 4,
		"FontBBox" => vec![(-543 as i32).into(), (-303 as i32).into(), (1300 as i32).into(), (979 as i32).into()],
		"ItalicAngle" => 0,
		"Ascent" => 0,
		"Descent" => 0,
		"CapHeight" => 979,
		"StemV" => 80,
		"FontFile2" => embed_libre_font,
	});
	let to_unicode = include_bytes!("../../include/Liberation_sans_font_to_unicode.bin");
	let to_unicode_vec: Vec<u8> = to_unicode.iter().cloned().collect();
	let to_unicode_obj = doc.add_object(Stream::new(dictionary! {}, to_unicode_vec).with_compression(true));
	doc.objects.insert(f1, Object::Dictionary(dictionary! {
		"Type" => "Font",
		"Subtype" => "TrueType",
		"BaseFont" => "BAAAAA+LiberationSans",
		"FirstChar" => 0,
		"LastChar" => 65,
		"Widths" => vec![ 750.into(), 666.into(), 666.into(), 722.into(), 722.into(),
		666.into(), 610.into(), 777.into(), 722.into(), 277.into(), 500.into(),
		666.into(), 556.into(), 833.into(), 722.into(), 777.into(), 666.into(),
		777.into(), 722.into(), 666.into(), 610.into(), 722.into(), 666.into(),
		943.into(), 666.into(), 666.into(), 610.into(), 556.into(), 556.into(),
		500.into(), 556.into(), 556.into(), 277.into(), 556.into(), 556.into(),
		222.into(), 222.into(), 500.into(), 222.into(), 833.into(), 556.into(),
		556.into(), 556.into(), 556.into(), 333.into(), 500.into(), 277.into(),
		556.into(), 500.into(), 722.into(), 500.into(), 500.into(), 500.into(),
		556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(),
		556.into(), 556.into(), 556.into(), 556.into(), 277.into(), 277.into(),
		277.into()],
		"FontDescriptor" => font_descripter,
		"ToUnicode" => to_unicode_obj,
	}));
	let check_mark_font = include_bytes!("../../include/Symbola_font_striped.bin");
	let check_mark_font_vec: Vec<u8> = check_mark_font.iter().cloned().collect();
	let embed_check_mark_font = doc.add_object(Stream::new(dictionary! {"Length1" => check_mark_font_vec.len() as i32}, check_mark_font_vec).with_compression(false));
	let font_descripter = doc.add_object(dictionary! {
		"Type" => "FontDescriptor",
		"FontName" => "CAAAAA+Symbola",
		"Flags" => 4,
		"FontBBox" => vec![(-838 as i32).into(), (-341 as i32).into(), (2991 as i32).into(), (925 as i32).into()],
		"ItalicAngle" => 0,
		"Ascent" => 0,
		"Descent" => 0,
		"CapHeight" => 925,
		"StemV" => 80,
		"FontFile2" => embed_check_mark_font,
	});
	let to_unicode = include_bytes!("../../include/Symbola_font_to_unicode.bin");
	let to_unicode_vec: Vec<u8> = to_unicode.iter().cloned().collect();
	let to_unicode_obj = doc.add_object(Stream::new(dictionary! {}, to_unicode_vec).with_compression(true));
	doc.objects.insert(f2, Object::Dictionary(dictionary! {
		"Type" => "Font",
		"Subtype" => "TrueType",
		"BaseFont" => "CAAAAA+Symbola",
		"FirstChar" => 0,
		"LastChar" => 1,
		"Widths" => vec![626.into(), 784.into()],
		"FontDescriptor" => font_descripter,
		"ToUnicode" => to_unicode_obj,
	}));
}


fn get_font_dict(doc: &mut Document, resourse: &mut Object) -> Dictionary {
	let resourse_dict_result = resourse.as_dict_mut();

	let resourse_dict = match resourse_dict_result {
		Ok(resourse_dict) => resourse_dict,
		Err(err) => panic!("error while getting resourse_dict: {}", err),
	};

	let fonts_obj = match resourse_dict.get_mut(b"Font"){
		Ok(fonts_obj) => fonts_obj.clone(),
		Err(_err) => Object::Dictionary(dictionary! {}),
	};

	let mut fonts_obj_deref = match fonts_obj.as_reference() {
		Ok(font_deref) => {
			let font_obj_derefd = match doc.get_object(font_deref) {
				Ok(obj_refd) => obj_refd.clone(),
				Err(err) => panic!("error dereffing fonts_obj_deref: {}", err),
			};
			doc.delete_object(font_deref);
			font_obj_derefd
		},
		Err(_) => fonts_obj,
	};

	let font_dict = match fonts_obj_deref.as_dict_mut() {
		Ok(font_dict) => font_dict,
		Err(err) => panic!("error getting font_dict {}", err),
	};

	font_dict.clone()
}

fn make_all_refs(doc: &mut Document, font_dict: &mut Dictionary) {
	let mut inline_font_dict: &Dictionary;
	for (_key, value) in font_dict.iter_mut() {
		match value.as_reference() {
			Ok(_) => continue,
			Err(_) => inline_font_dict = match value.as_dict() {
				Ok(font_dict) => font_dict,
				Err(err) => panic!("error getting inline_font_dict: {}", err),
			},
		};
		*value = Object::Reference(doc.add_object(Object::Dictionary(inline_font_dict.clone())));
	}
}

fn edit_content_streams(doc: &mut Document, p_id: &ObjectId, f_map: BTreeMap<String, String>) {
	let mut str_stream: String = "".to_string();
	let mut last = ObjectId::default();

	let mut cm_vec = Vec::<Matrix>::new();
	cm_vec.push(Matrix::new());
	for content_id in doc.get_page_contents(*p_id) {
		last = content_id;
		get_content_str(doc, &content_id, &mut str_stream);

		edit(&mut str_stream, &f_map, &mut cm_vec);

		doc.change_content_stream(content_id, str_stream.as_bytes().to_vec());
	};

	let mat = Matrix::inverse(&mut cm_vec);
	let mut mat_s = find_scale(&doc);
	mat_s.multiply(mat);
	let m = [mat_s.cm[0], mat_s.cm[1], mat_s.cm[2], mat_s.cm[3], mat_s.cm[4], mat_s.cm[5]];
	let str = format!("{} {} {} {} {} {} cm ", m[0], m[1], m[2], m[3], m[4], m[5]);
	str_stream.push_str(str.as_str());
	doc.change_content_stream(last, str_stream.as_bytes().to_vec());
}

fn find_scale(doc: &Document) -> Matrix {
	let mut page_iter = doc.page_iter();
	let first_page = match doc.get_object(match page_iter.next() {
			Some(page) => page,
			None => panic!("imposseble"),
		}) {
		Ok(first_page) => first_page,
		Err(err) => panic!("??? {}", err),
	};

	let h = first_page.as_dict().unwrap()
		.get(b"MediaBox").unwrap()
		.as_array().unwrap()
		.get(3).unwrap()
		.as_f64().unwrap();

	let scale = h/842f64;
	let round_scale = (scale * 1000f64).round() / 1000f64;


	Matrix { cm: [round_scale, 0f64, 0f64, round_scale, 0f64, 0f64], i: 0 }
}

fn get_content_str(doc: &mut Document, content_id: &ObjectId, str_stream: &mut String) {
	let cn_stream: &Stream;
	let content: &Object;

	content = match doc.get_object(*content_id) {
		Ok(content) => content,
		Err(err) => panic!("error getting the content obj: {}", err),
	};

	cn_stream = match content.as_stream() {
		Ok(stream) => stream,
		Err(err) => panic!("error getting content stream: {}", err),
	};

	*str_stream = String::from_utf8(cn_stream.content.clone()).expect("Found invalid UTF-8");
}

#[derive(Clone, Debug)]
struct Matrix {
	cm: [f64; 6],
	i: u8,
}

impl Matrix {
	fn new() -> Matrix {
		Matrix { cm: [1f64, 0f64, 0f64, 1f64, 0f64, 0f64], i: 0 }
	}

	fn clear(&mut self) {
		self.cm = [1f64, 0f64, 0f64, 1f64, 0f64, 0f64];
		self.i = 0;
	}

	fn push(&mut self, num: f64) {
		for i in 0..5 {
			self.cm[i] = self.cm[i+1];
		}
		self.cm[5] = num;
		self.i += 1;
	}

	fn multiply(&mut self, old: Matrix) {
		let matrix_a = [
			[old.cm[0], old.cm[1], old.cm[4]],
			[old.cm[2], old.cm[3], old.cm[5]],
			[     0f64,      0f64,      1f64]
		];
		let matrix_b = [
			[self.cm[0], self.cm[1], self.cm[4]],
			[self.cm[2], self.cm[3], self.cm[5]],
			[      0f64,       0f64,       1f64]
		];
		let mut out = [
			[ 0f64, 0f64, 0f64],
			[ 0f64, 0f64, 0f64],
			[ 0f64, 0f64, 0f64]
		];

		for i in 0..3 {
			for j in 0..3 {
				for k in 0..3 {
					out[i][j] += matrix_a[i][k] * matrix_b[k][j];
				}
			}
		}

		self.cm = [out[0][0], out[0][1], out[1][0], out[1][1], out[0][2], out[1][2]];
		self.i = 0;
	}

	fn inverse(cm_vec: &mut Vec::<Matrix>) -> Matrix {
		let mut matrix = Matrix::new();
		
		for x in cm_vec {
			x.multiply(matrix);
			matrix = x.clone();
		}

		let m = [
			[matrix.cm[0], matrix.cm[1], matrix.cm[4]],
			[matrix.cm[2], matrix.cm[3], matrix.cm[5]],
			[        0f64,         0f64,         1f64]
		];

		let mut determinant = 0f64;
		for i in 0..3 {
			determinant +=  m[0][i]*(m[1][(i+1)%3]*m[2][(i+2)%3] - m[1][(i+2)%3]*m[2][(i+1)%3]);
		}

		let mut out = [
			[ 0f64, 0f64, 0f64],
			[ 0f64, 0f64, 0f64],
			[ 0f64, 0f64, 0f64]
		];

		for i in 0..3 {
			for j in 0..3 {
				let n = m[(j+1)%3][(i+1)%3] * m[(j+2)%3][(i+2)%3];
				let l = m[(j+1)%3][(i+2)%3] * m[(j+2)%3][(i+1)%3];
				let o: f64 = n - l;

				if o.eq(&0f64) {
					out[i][j] = 0f64;
					continue;
				}

				out[i][j] = o / determinant;
			}
		}

		matrix.cm = [out[0][0], out[0][1], out[1][0], out[1][1], out[0][2], out[1][2]];
		matrix
	}
}


fn edit(str_stream: &mut String, f_map: &BTreeMap<String, String>, cm_vec: &mut Vec::<Matrix>) {
	let stream_lines = str_stream.lines();
	let mut new_stream: String = "".to_string();
	let mut new_line: String;

	let mut is_text_block: bool = false;
	let mut matrix = cm_vec.last().unwrap().clone();
	for line in stream_lines {
		new_line = "".to_string();

		for word in line.split(' ') {
			let mut new_word: String = word.to_string();
			if word.eq("BT") {is_text_block = true};

			if !is_text_block {
				if word.eq("cm") {
					if matrix.i.ne(&6) {continue;};
					matrix.multiply(match cm_vec.pop() {
						Some(old_matrix) => old_matrix,
						None => Matrix::new(),
					});
					cm_vec.push(matrix.clone());
				}

				if word.eq("q") {
					cm_vec.push(match cm_vec.last() {
					Some(new_matrix) => new_matrix.clone(),
					None => Matrix::new(),
					})
				};

				if word.eq("Q") {
					cm_vec.pop();
				};

				match word.parse::<f64>() {
					Ok(num) => matrix.push(num),
					Err(_) => matrix.clear(),
				}
			}
			else {
				let mut chars = word.clone().chars();
				if chars.nth(0).unwrap_or(' ').eq(&'/') {
					new_word = match f_map.get(&chars.collect::<String>()) {
						Some(new_font_name) => "/".to_string() + new_font_name,
						None => word.to_string(),
					};
					
				}
			}

			if word.eq("ET") {is_text_block = false};
			new_line += &(new_word + &" ".to_string());
		}


		new_stream += &(new_line + &"\n".to_string());
	};

	*str_stream = new_stream;
}
