#[macro_export]
/// A quick macro, expands to `Default::default()`
macro_rules! no {
    () => {
        Default::default()
    };
}
