//! The only element vocabulary this renderer understands. `rsx!` resolves a lowercase
//! tag `foo { .. }` to `dioxus_elements::foo::TAG_NAME`, so any tag without a matching
//! module here (e.g. `div`) simply fails to compile.

macro_rules! define_tag {
    ($lit:ident) => {
        #[allow(non_camel_case_types)]
        pub mod $lit {
            pub const TAG_NAME: &str = stringify!($lit);
            pub const NAME_SPACE: Option<&str> = None;
        }
    };
}

pub mod elements {
    define_tag!(block);
    define_tag!(text);
}

pub use elements::block;
pub use elements::text;
