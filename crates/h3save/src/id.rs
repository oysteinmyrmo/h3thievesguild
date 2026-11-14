//! Newtype IDs for game data.
//!
//! IDs in the save files are plain integers, and new HotA releases add new values.
//! A newtype keeps any value it reads, known or not, while named constants cover
//! the known ones. `Debug` prints the known name, or the raw value otherwise.

/// Declare an ID newtype with named constants.
#[macro_export]
#[doc(hidden)]
macro_rules! id_type {
    (
        $(#[$meta:meta])*
        $name:ident($repr:ty) {
            $(
                $(#[$vmeta:meta])*
                $konst:ident = $val:literal => $label:literal,
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub $repr);

        impl $name {
            $(
                $(#[$vmeta])*
                pub const $konst: $name = $name($val);
            )*

            /// All named values, in declaration order.
            pub const ALL: &'static [$name] = &[$($name::$konst),*];

            /// The name of a known value, or `None` for values this crate does not know.
            pub fn name(self) -> Option<&'static str> {
                match self.0 {
                    $($val => Some($label),)*
                    _ => None,
                }
            }

            /// Whether the value is one this crate knows.
            pub fn is_known(self) -> bool {
                self.name().is_some()
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self.name() {
                    Some(name) => write!(f, "{}::{}", stringify!($name), name),
                    None => write!(f, "{}({})", stringify!($name), self.0),
                }
            }
        }

        impl From<$repr> for $name {
            fn from(value: $repr) -> Self {
                $name(value)
            }
        }

        impl From<$name> for $repr {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}
