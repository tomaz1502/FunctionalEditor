use super::ColorsConfig;

#[allow(non_upper_case_globals)]
static HaskellKeywords: [&'static str; 17] =
    [ "as"
    , "case"
    , "class"
    , "data"
    , "deriving"
    , "do"
    , "else"
    , "family"
    , "forall"
    , "foreign"
    , "if"
    , "import"
    , "instance"
    , "let"
    , "then"
    , "type"
    , "where"
    ];

#[allow(non_upper_case_globals)]
pub static HaskellConfig: ColorsConfig = ColorsConfig {
    is_keyword      : |word| HaskellKeywords.contains(&word),
    is_type_name    : |word| !word.is_empty() && word.chars().next().unwrap().is_uppercase(),
    num_color       : "red",
    type_name_color : "blue",
    keyword_color   : "green",
    default_color   : "normal",
};
