use convert_case::{Case, Casing};

pub fn to_kebab(text: &str) -> String {
    text.to_case(Case::Kebab)
}
