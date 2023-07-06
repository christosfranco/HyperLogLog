pub mod internal {
    pub fn add(a: i32, b: i32) -> i32 {
        if a > 5 {
            panic!("a value too high");
        }
        a + b
    }

    pub fn add_result(a: i32, b: i32) -> Result<i32, String> {
        if a > 5 {
            return Err::<i32, String>(String::from("a value too high"));
        }
        return Ok(a + b);
    }

    #[cfg(test)]
    mod tests {

        use super::add_result;
        use super::add;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
            assert_eq!(add(-1, 1), 0);
        }

        #[test]
        #[should_panic(expected = "a value too high")]
        fn test_add_panic() {
            add(6, 1);
        }

        #[test]
        fn test_add_result() {
            let value = add_result(5, 2);
            assert_eq!(value, Ok(7));

            let value = add_result(6, 2);
            assert_eq!(value.is_err(), true);
            assert_eq!(value.unwrap_err(), "a value too high")
        }
    }
}

// pub use internal::add_result;
// pub use internal::add;

// #[cfg(test)]
// pub use internal::add;
// pub use internal::add_result;
