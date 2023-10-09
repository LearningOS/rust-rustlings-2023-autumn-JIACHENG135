extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: We know those functions are aliases of a safe Rust function.
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456);
        }
    }
}
