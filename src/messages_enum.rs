#[macro_export]
macro_rules! messages_enum {
    ($visibility:vis enum $name:ident {
        $($variant:ident),*,
    }) => {
        $visibility enum $name {
            $($variant),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $($name::$variant => String::from(stringify!($variant))),*
                }
            }
        }
    };
}
