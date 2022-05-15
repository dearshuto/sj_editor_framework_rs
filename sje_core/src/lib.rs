use std::sync::Arc;

pub struct Document<T> {
    pub data: Arc<T>,
}

pub trait IProject<T> {}

#[macro_export]
macro_rules! test {
    ( $( $x:expr ),* ) => {{
        10
    }};
}

#[macro_export]
macro_rules! match_ty {
    ( $( $x:ty ),* ) => { pub struct AA { pub a: $($x), * } }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a() {
        let _ = crate::test!();
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
