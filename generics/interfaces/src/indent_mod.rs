pub mod indent_mod {

    pub trait Indent {
        fn indent(&self) -> String {
            String::from("Not implemented")
        }
    }
}

// cargo test --all
#[cfg(test)]
pub mod indent_mod_test {

    #[test]
    fn does_it_work() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn some_test() {
        let result = 2 + 3;
        assert_eq!(result, 100, "Expect 2+3 to be 100 but found {}", result);
    }
}
