fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
}

// In this example, we do not have to return the string as a tuple in order to use it again.
// This is because we didn't actually pass in the string to the calculate_length function, but we
// passed in a reference to the string instead.
//
// A reference is guaranteed to point to a valid value of a particular type for the life of the
// reference.
fn calculate_length(s: &String) -> usize {
    s.len()
}
