// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Using if let statement to unwrap the Option
        if let Some(word) = optional_target {
            assert_eq!(word, target,);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor: Option<i8> = Some(range as i8); 

        // Using while let statement to unwrap the Option and pop from the vector
        while let Some(integer) = optional_integers.pop() {
            if let Some(value) = integer {
                assert_eq!(value, cursor.unwrap());
                cursor = Some((cursor.unwrap() - 1) as i8);
            } else {
                // Handle the case where integer is None
                println!("Error: integer is None");
                break;
            }
        }

        assert_eq!(cursor, Some(0));
    }
}
