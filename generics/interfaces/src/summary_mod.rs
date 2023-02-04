pub mod summary_mod {

    pub trait Summary {
        fn summary(&self) -> String {
            String::from("Not implemented")
        }
    }
}
