fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Handle the None case, e.g., by returning or panicking
        return; // or any other appropriate action
    }

    // Corrected array syntax
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Using clear to create an empty vector
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // Using std::mem::swap to swap values
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}