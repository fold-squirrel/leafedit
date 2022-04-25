use lopdf::{Dictionary, Object, Stream, dictionary};
/// list supported options

pub enum Error {
    KeyNotPresent,
}


pub fn f_10() -> Dictionary {
    dictionary!{
        "Type" => "Font",
        "Subtype" => "TrueType",
        "BaseFont" => "AHUBPT+LiberationSans",
        "FirstChar" => 32,
        "LastChar" => 122,
        "FontDescriptor" => (13, 0),
        "Encoding" => "WinAnsiEncoding",
        "Widths" => vec![
            277.into(), 277.into(), 354.into(), 556.into(), 556.into(),
            889.into(), 666.into(), 190.into(), 333.into(), 333.into(), 389.into(),
            583.into(), 277.into(), 333.into(), 277.into(), 277.into(), 556.into(),
            556.into(), 556.into(), 556.into(), 556.into(), 556.into(), 556.into(),
            556.into(), 556.into(), 556.into(), 277.into(), 277.into(), 583.into(),
            583.into(), 583.into(), 556.into(), 1015.into(), 666.into(), 666.into(),
            722.into(), 722.into(), 666.into(), 610.into(), 777.into(), 722.into(),
            277.into(), 500.into(), 666.into(), 556.into(), 833.into(), 722.into(),
            777.into(), 666.into(), 777.into(), 722.into(), 666.into(), 610.into(),
            722.into(), 666.into(), 943.into(), 666.into(), 666.into(), 610.into(),
            277.into(), 277.into(), 277.into(), 469.into(), 556.into(), 333.into(),
            556.into(), 556.into(), 500.into(), 556.into(), 556.into(), 277.into(),
            556.into(), 556.into(), 222.into(), 222.into(), 500.into(), 222.into(),
            833.into(), 556.into(), 556.into(), 556.into(), 556.into(), 333.into(),
            500.into(), 277.into(), 556.into(), 500.into(), 722.into(), 500.into(),
            500.into(), 500.into(), 333.into(), 259.into(), 333.into(), 583.into(),
        ],
        "ToUnicode" => (12, 0),
    }
}

pub fn f_12() -> Stream {
    let to_unicode = include_bytes!("../../include/(13, 0).bin");
    Stream { dict: dictionary!{"Length" => to_unicode.len() as i64}, content: to_unicode.to_vec(), allows_compression: true, start_position: None }
}

pub fn f_13() -> Dictionary {
    dictionary!{
        "Type" => "FontDescriptor",
        "FontName" => "AHUBPT+LiberationSans",
        "FontFamily" => Object::string_literal("Liberation Sans"),
        "Flags" => 32,
        "FontBBox" => vec![ (-543 as i64).into(), (-303 as i64).into(), 1301.into(), 979.into(), ],
        "ItalicAngle" => 0,
        "Ascent" => 905,
        "Descent" => (-211 as i64),
        "CapHeight" => 979,
        "StemV" => 80,
        "StemH" => 80,
        "FontFile2" => (14, 0),
    }
}

pub fn f_14() -> Stream {
    let font_file = include_bytes!("../../include/(11, 0).bin");
    let len = font_file.len() as i64;
    Stream { dict: dictionary!{ "Length" => len,  "Length1" => len}, content: font_file.to_vec(), allows_compression: false, start_position: None}
}

pub fn f_11() -> Dictionary {
    dictionary!{
        "Type" => "Font",
        "Subtype" => "Type0",
        "BaseFont" => "FHWMZH+Symbola",
        "Encoding" => "Identity-H",
        "DescendantFonts" => vec![Object::Reference((16, 0))],
        "ToUnicode" => (15, 0),
    }
}

pub fn f_15() -> Stream {
    let to_unicode = include_bytes!("../../include/(18, 0).bin");
    Stream { dict: dictionary!{ "Length" => to_unicode.len() as i64 }, content: to_unicode.to_vec(), allows_compression: false, start_position: None }
}

pub fn f_16() -> Dictionary {
    dictionary!{
        "Type" => "Font",
        "Subtype" => "CIDFontType2",
        "BaseFont" => "FHWMZH+Symbola",
        "CIDSystemInfo" => dictionary!{
            "Registry" => Object::string_literal("Adobe"),
            "Ordering" => Object::string_literal("Identity"),
            "Supplement" => Object::Integer(0),
        },
        "FontDescriptor" => (17, 0),
        "W" => vec![ 0.into(), Object::Array(vec![ 626.into(), 784.into() ]) ],
    }
}

pub fn f_17() -> Dictionary{
    dictionary!{
        "Type" => "FontDescriptor",
        "FontName" => "FHWMZH+Symbola",
        "FontFamily" => Object::string_literal("Symbola"),
        "Flags" => 4,
        "FontBBox" => vec![ (-838 as i64).into(), (-341 as i64).into(), 2992.into(), 925.into(), ],
        "ItalicAngle" => 0,
        "Ascent" => 925,
        "Descent" => -341_i64,
        "CapHeight" => 925,
        "StemV" => 80,
        "StemH" => 80,
        "FontFile2" => (18, 0),
    }
}

pub fn f_18() -> Stream {
    let font_file = include_bytes!("../../include/(16, 0).bin");
    let len = font_file.len() as i64;
    Stream { dict: dictionary!{ "Length" => len,  "Length1" => len}, content: font_file.to_vec(), allows_compression: false, start_position: None}
}
