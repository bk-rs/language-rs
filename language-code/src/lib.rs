#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate alloc;

//
#[macro_export]
macro_rules! language_code {
    (
        length = $length:tt;
        $( #[$meta:meta] )*
        $pub:vis enum $name:ident {
            $(
                $( #[$variant_meta:meta] )*
                $variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        $pub enum $name {
            $(
                $( #[$variant_meta] )*
                $variant,
            )+
            Other($crate::alloc::boxed::Box<str>),
        }

        //
        impl $name {
            pub const VARS: &'static [$name] = &[
                $(
                    $name::$variant,
                )+
            ];
        }

        //
        paste::paste! {
            impl ::core::str::FromStr for $name {
                type Err = $crate::error::ParseError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $(
                            ::core::stringify!($variant) | ::core::stringify!([<$variant:upper>]) => Ok(Self::$variant),
                        )+
                        s if s.len() == $length => Ok(Self::Other(s.into())),
                        s => Err($crate::error::ParseError::Invalid(s.into()))
                    }
                }
            }
        }

        //
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$variant => ::core::write!(f, "{}", ::core::stringify!($variant)),
                    )+
                    Self::Other(s) => ::core::write!(f, "{}", s)
                }
            }
        }

        //
        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $crate::alloc::format!("{}", self) == $crate::alloc::format!("{}", other)
            }
        }

        impl ::core::cmp::Eq for $name {
        }

        //
        impl_macros::impl_partial_eq_str_for_display! { str, $name }
        impl_macros::impl_partial_eq_str_for_display! { &'a str, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::borrow::Cow<'a, str>, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::string::String, $name }

        //
        #[cfg(feature = "std")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                $crate::alloc::format!("{}", self).hash(state);
            }
        }

        //
        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::core::str::FromStr as _;

                let s = $crate::alloc::boxed::Box::<str>::deserialize(deserializer)?;
                Self::from_str(&s).map_err(::serde::de::Error::custom)
            }
        }

        //
        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use $crate::alloc::string::ToString as _;

                self.to_string().serialize(serializer)
            }
        }
    };
}

//
#[macro_export]
macro_rules! language_tag {
    (
        $( #[$meta:meta] )*
        $pub:vis struct $name:ident {
            $( #[$language_code_meta:meta] )*
            $language_code_pub:vis $language_code_name:ident : $language_code_ty:ty,
            $( #[$country_code_meta:meta] )*
            $country_code_pub:vis $country_code_name:ident : Option<$country_code_ty:ty>,
        }
    ) => {
        $(#[$meta])*
        $pub struct $name {
            $( #[$language_code_meta:meta] )*
            $language_code_pub $language_code_name: $language_code_ty,
            $( #[$country_code_meta:meta] )*
            $country_code_pub $country_code_name: Option<$country_code_ty>,
        }

        //
        impl $name {
            pub fn new(language_code: $language_code_ty, country_code: Option<$country_code_ty>) -> Self {
                Self {
                    language_code,
                    country_code,
                }
            }
        }

        //
        impl ::core::str::FromStr for $name {
            type Err = $crate::error::LanguageTagParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let language_code_s = s.chars().take_while(|x| x != &'-' && x != &'_')
                                                .collect::<$crate::alloc::string::String>();
                let language_code = language_code_s.parse::<$language_code_ty>()
                                                    .map_err(|_| $crate::error::LanguageTagParseError::InvalidLanguageCode(language_code_s.as_str().into()))?;

                let country_code = if s.len() > language_code_s.len() + 1 {
                    let country_code_s = &s[language_code_s.len() + 1..];
                    let country_code = country_code_s.parse::<$country_code_ty>()
                                                    .map_err(|_| $crate::error::LanguageTagParseError::InvalidCountryCode(country_code_s.into()))?;
                    Some(country_code)
                } else {
                    None
                };

                Ok(Self::new(language_code, country_code))
            }
        }

        //
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if let Some(country_code) = &self.country_code {
                    ::core::write!(f, "{}-{}", &self.language_code, country_code)
                } else {
                    ::core::write!(f, "{}", &self.language_code)
                }
            }
        }

        //
        impl ::core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $crate::alloc::format!("{}", self) == $crate::alloc::format!("{}", other)
            }
        }

        impl ::core::cmp::Eq for $name {
        }

        //
        impl_macros::impl_partial_eq_str_for_display! { str, $name }
        impl_macros::impl_partial_eq_str_for_display! { &'a str, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::borrow::Cow<'a, str>, $name }
        impl_macros::impl_partial_eq_str_for_display! { $crate::alloc::string::String, $name }

        //
        #[cfg(feature = "std")]
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                $crate::alloc::format!("{}", self).hash(state);
            }
        }

        //
        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::core::str::FromStr as _;

                let s = $crate::alloc::boxed::Box::<str>::deserialize(deserializer)?;
                Self::from_str(&s).map_err(::serde::de::Error::custom)
            }
        }

        //
        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use $crate::alloc::string::ToString as _;

                self.to_string().serialize(serializer)
            }
        }
    };
}

//
pub mod error;

//
pub mod iso639_1;

pub use iso639_1::{LanguageCode, LanguageTag};
