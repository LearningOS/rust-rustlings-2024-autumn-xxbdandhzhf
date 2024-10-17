// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

use std::vec::Vec;

// Define the AppendBar trait
trait AppendBar {
    fn append_bar(self) -> Self;
}

// Define a new struct that wraps a Vec<String> and implements the AppendBar trait
struct Foo(Vec<String>);

impl AppendBar for Foo {
    fn append_bar(self) -> Self {
        let mut inner = self.0;
        inner.push(String::from("Bar"));
        Foo(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = Foo(vec![String::from("Foo")]).append_bar();
        assert_eq!(foo.0.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.0.pop().unwrap(), String::from("Foo"));
    }
}
