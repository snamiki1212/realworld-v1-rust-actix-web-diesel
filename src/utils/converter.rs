use convert_case::{Case, Casing};

pub fn to_kebab(text: &str) -> String {
    text.to_case(Case::Kebab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_kebab() {
        assert_eq!("this-is-blog-title", to_kebab("this is blog title"));
    }
}
