use super::ColorsConfig;

#[allow(non_upper_case_globals)]
static HaskellKeywords: [&'static str; 24] =
    [ "case"
    , "class"
    , "data"
    , "deriving"
    , "do"
    , "else"
    , "family"
    , "forall"
    , "foreign"
    , "hiding"
    , "if"
    , "infix"
    , "infixl"
    , "infixr"
    , "import"
    , "instance"
    , "let"
    , "module"
    , "newtype"
    , "of"
    , "qualified"
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
    default_color   : "white",
};
