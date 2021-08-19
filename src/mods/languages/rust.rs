use super::ColorsConfig;

#[allow(non_upper_case_globals)]
static RustKeywords: [&'static str; 35] =
    [ "as"
    , "break"
    , "const"
    , "continue"
    , "crate"
    , "else"
    , "enum"
    , "extern"
    , "false"
    , "fn"
    , "for"
    , "if"
    , "impl"
    , "in"
    , "let"
    , "loop"
    , "match"
    , "mod"
    , "move"
    , "mut"
    , "pub"
    , "ref"
    , "return"
    , "self"
    , "Self"
    , "static"
    , "struct"
    , "super"
    , "trait"
    , "true"
    , "type"
    , "unsafe"
    , "use"
    , "where"
    , "while"
    ];

#[allow(non_upper_case_globals)]
static RustTypes: [&'static str; 21] =
    [ "bool"
    , "char"
    , "isize"
    , "i8"
    , "i16"
    , "i32"
    , "i64"
    , "i128"
    , "f8"
    , "f16"
    , "f32"
    , "f64"
    , "f128"
    , "usize"
    , "u8"
    , "u16"
    , "u32"
    , "u64"
    , "u128"
    , "str"
    , "tuple"
    ];

#[allow(non_upper_case_globals)]
pub static RustConfig: ColorsConfig = ColorsConfig {
    is_keyword      : |word| RustKeywords.contains(&word),
    is_type_name    : |word| RustTypes.contains(&word),
    num_color       : "red",
    type_name_color : "blue",
    keyword_color   : "green",
    default_color   : "white",
};
