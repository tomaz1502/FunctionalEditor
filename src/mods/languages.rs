pub mod haskell;

#[derive(Clone, Copy)]
pub struct ColorsConfig {
    pub num_color       : &'static str,
    pub type_name_color : &'static str,
    pub keyword_color   : &'static str,
    pub default_color   : &'static str,
    pub is_type_name    : fn(&str) -> bool,
    pub is_keyword      : fn(&str) -> bool,
}

impl Default for ColorsConfig {
    fn default() -> Self {
        ColorsConfig {
            num_color       : "white",
            type_name_color : "white",
            keyword_color   : "white",
            default_color   : "white",
            is_type_name    : |_| false,
            is_keyword      : |_| false,
        }
    }
}
