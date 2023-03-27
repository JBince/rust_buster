use std::fmt;

//Indent for banner entries
const INDENT: usize = 4;

//Column width for banner entries
const COL_WIDTH: usize = 22;

pub(super) struct BannerEntry {
    //Title used in the banner entry
    pub name: String,

    //Valuepassed in via config/defaults
    pub value: String,
}

impl BannerEntry {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl fmt::Display for BannerEntry {
    //Display formatter for the given banner entry
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\u{0020}{:\u{0020}<indent$}{:\u{0020}<width$}\u{2502}\u{0020}{}",
            "",
            self.name,
            self.value,
            indent = INDENT,
            width = COL_WIDTH
        )
    }
}
