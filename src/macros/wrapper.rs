#[macro_export]
macro_rules! wrap {
    ($vis:vis $name:ident => $target:ty; default $default:expr) => {
        #[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
        #[repr(transparent)]
        $vis struct $name {
            inner: $target,
        }

        impl ::core::convert::From<$target> for $name {
            fn from(value: $target) -> Self {
                Self::new(value)
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl $name {
            pub fn new(value: $target) -> Self {
                Self { inner: value }
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::new($default)
            }
        }
    };

    ($vis:vis $name:ident => $target:ty) => {
        $crate::wrap!($vis $name => $target; default Default::default());
    }
}
