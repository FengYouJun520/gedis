#[macro_export]
macro_rules! LogArgs {
    ($($log: expr),+ $(,)?) => (
        {
             let mut v = vec![];
            $(
                v.push($log.to_string());
            )*
            v
        }
    );
}
