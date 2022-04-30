// cargo expand --verbose --all-features --test macro_language_code

language_code::language_code! {
    #[derive(Debug, Clone)]
    #[allow(non_camel_case_types)]
    pub enum MyLanguageCode {
        en,
    }
}
