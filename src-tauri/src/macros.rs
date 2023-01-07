#[macro_export]
macro_rules! LogArgs {
    ($($log: expr),+ $(,)?) => (
        vec![$($log.to_string()), *]
    );
}
