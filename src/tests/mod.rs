#[cfg(test)]
mod types;

#[macro_export]
macro_rules! test_size {
    ($n:ident $($t:ty)*) => {
        #[test]
        fn $n() {
            $(
                println!("size of {} is {}", stringify!($t), std::mem::size_of::<$t>());
            )*
        }
    };
}

test_size!(test_i32 i32 i64);
