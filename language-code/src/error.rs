use core::fmt;

//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Invalid(::alloc::boxed::Box<str>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageTagParseError {
    InvalidLanguageCode(::alloc::boxed::Box<str>),
    InvalidCountryCode(::alloc::boxed::Box<str>),
}

impl fmt::Display for LanguageTagParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LanguageTagParseError {}
