use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, Stream, StringFormat,dictionary};

pub fn run_demo() {
	let mut doc = Document::with_version("1.7");
	let pages_id = doc.new_object_id();
	let libre_font = include_bytes!("../include/Liberation_sans_font_striped.bin");
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
	let to_unicode = include_bytes!("../include/Liberation_sans_font_to_unicode.bin");
	let to_unicode_vec: Vec<u8> = to_unicode.iter().cloned().collect();
	let to_unicode_obj = doc.add_object(Stream::new(dictionary! {}, to_unicode_vec).with_compression(true));
	let libre_font_obj = doc.add_object(dictionary! {
		"Type" => "Font",
		"Subtype" => "TrueType",
		"BaseFont" => "BAAAAA+LiberationSans",
		"FirstChar" => 0,
		"LastChar" => 64,
		"Widths" => vec![750.into(), 666.into(), 666.into(), 722.into(), 722.into(), 666.into(), 610.into(), 777.into(), 722.into(), 277.into(), 500.into(), 666.into(), 556.into(), 833.into(), 722.into(), 777.into(), 666.into(), 777.into(), 722.into(), 666.into(), 610.into(), 722.into(), 666.into(), 943.into(), 666.into(), 666.into(), 610.into(), 556.into(), 556.into(), 500.into(), 556.into(), 556.into(), 277.into(), 556.into(), 556.into(), 222.into(), 222.into(), 500.into(), 222.into(), 833.into(), 556.into(), 556.into(), 556.into(), 556.into(), 333.into(), 500.into(), 277.into(), 556.into(), 500.into(), 722.into(), 500.into(), 500.into(), 500.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 277.into(), 277.into()],
		"FontDescriptor" => font_descripter,
		"ToUnicode" => to_unicode_obj,
	});
	let check_mark_font = include_bytes!("../include/Symbola_font_striped.bin");
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
	let to_unicode = include_bytes!("../include/Symbola_font_to_unicode.bin");
	let to_unicode_vec: Vec<u8> = to_unicode.iter().cloned().collect();
	let to_unicode_obj = doc.add_object(Stream::new(dictionary! {}, to_unicode_vec).with_compression(true));
	let check_mark_font_obj = doc.add_object(dictionary! {
		"Type" => "Font",
		"Subtype" => "TrueType",
		"BaseFont" => "CAAAAA+Symbola",
		"FirstChar" => 0,
		"LastChar" => 1,
		"Widths" => vec![626.into(), 784.into()],
		"FontDescriptor" => font_descripter,
		"ToUnicode" => to_unicode_obj,
	});
	let resources_id = doc.add_object(dictionary! {
		"Font" => dictionary! {
			"F1" => libre_font_obj,
			"F2" => check_mark_font_obj,
		},
	});
	let content = Content {
		operations: vec![
			Operation::new("BT", vec![]),
			Operation::new("Tf", vec!["F1".into(), 48.into()]),
			Operation::new("Td", vec![100.into(), 600.into()]),
			Operation::new("Tj", vec![Object::String(vec![0x3f, 0x12, 0x1, 0x23], StringFormat::Hexadecimal)]),
			Operation::new("Tf", vec!["F2".into(), 48.into()]),
			Operation::new("Tj", vec![Object::String(vec![0x1], StringFormat::Hexadecimal)]),
			Operation::new("ET", vec![]),
		],
	};
	let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()).with_compression(true));
	let page_id = doc.add_object(dictionary! {
		"Type" => "Page",
		"Parent" => pages_id,
		"Contents" => content_id,
	});
	let pages = dictionary! {
		"Type" => "Pages",
		"Kids" => vec![page_id.into()],
		"Count" => 1,
		"Resources" => resources_id,
		"MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
	};
	doc.objects.insert(pages_id, Object::Dictionary(pages));
	let catalog_id = doc.add_object(dictionary! {
		"Type" => "Catalog",
		"Pages" => pages_id,
	});
	doc.trailer.set("Root", catalog_id);
	doc.compress();
	doc.save("example.pdf").unwrap();
}
