// cargo expand --verbose --all-features --test macro_language_tag

use country_code::iso3166_1::alpha_2::CountryCode;
use language_code::iso639_1::LanguageCode;

language_code::language_tag! {
    #[derive(Debug, Clone)]
    pub struct MyLanguageTag {
        pub language_code: LanguageCode,
        pub country_code: Option<CountryCode>,
    }
}
